mod opcode;

use opcode::Opcode;
use std::env;
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, PartialEq)]
pub enum IR {
    // >>>> ---> SHR(4)
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    // jump if zero
    JIZ(u32),
    // jump if not zero
    JNZ(u32),
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs: Vec<IR> = Vec::new();
        let mut jstack: Vec<u32> = Vec::new();
        for e in data {
            match e {
                Opcode::SHR => match instrs.last_mut() {
                    Some(IR::SHR(n)) => *n += 1,
                    _ => instrs.push(IR::SHR(1)),
                },
                Opcode::SHL => match instrs.last_mut() {
                    Some(IR::SHL(n)) => *n += 1,
                    _ => instrs.push(IR::SHL(1)),
                },
                Opcode::ADD => match instrs.last_mut() {
                    Some(IR::ADD(n)) => {
                        let (b, _) = n.overflowing_add(1);
                        *n = b
                    }
                    _ => instrs.push(IR::ADD(1)),
                },
                Opcode::SUB => match instrs.last_mut() {
                    Some(IR::SUB(n)) => {
                        let (b, _) = n.overflowing_add(1);
                        *n = b
                    }
                    _ => instrs.push(IR::SUB(1)),
                },
                Opcode::PUTCHAR => {
                    instrs.push(IR::PUTCHAR);
                }
                Opcode::GETCHAR => {
                    instrs.push(IR::GETCHAR);
                }
                Opcode::LB => {
                    instrs.push(IR::JIZ(0));
                    jstack.push((instrs.len() - 1) as u32);
                }
                Opcode::RB => {
                    let j = jstack.pop().ok_or("pop from empty jstack")?;
                    instrs.push(IR::JNZ(j));
                    let instrs_len = instrs.len();
                    match &mut instrs[j as usize] {
                        IR::JIZ(n) => *n = (instrs_len - 1) as u32,
                        _ => unreachable!(),
                    }
                }
            }
        }
        Ok(Code { instrs })
    }
}

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = Code::from(opcode_code.instrs)?;
        let code_len = code.instrs.len();
        let mut pc = 0;
        let mut sp = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                IR::SHR(n) => {
                    sp += n as usize;
                    if sp >= self.stack.len() {
                        let expand = sp - self.stack.len() + 1;
                        for _ in 0..expand {
                            self.stack.push(0);
                        }
                    }
                }
                IR::SHL(n) => sp = if sp == 0 { 0 } else { sp - n as usize },
                IR::ADD(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_add(x).0;
                }
                IR::SUB(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(x).0;
                }
                IR::PUTCHAR => {
                    stdout().write_all(&[self.stack[sp]])?;
                }
                IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                IR::JIZ(x) => {
                    if self.stack[sp] == 0x00 {
                        pc = x as usize;
                    }
                }
                IR::JNZ(x) => {
                    if self.stack[sp] != 0x00 {
                        pc = x as usize;
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = std::fs::File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();
    i.run(c)?;
    Ok(())
}
