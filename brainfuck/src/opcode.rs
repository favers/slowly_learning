use std::collections::HashMap;
// https://zh.wikipedia.org/wiki/Brainfuck
// >	指针加一
// <	指针减一
// +	指针指向的字节的值加一
// -	指针指向的字节的值减一
// .	输出指针指向的单元内容（ASCII码）
// ,	输入内容到指针指向的单元（ASCII码）
// [	如果指针指向的单元值为零，向后跳转到对应的]指令的次一指令处
// ]	如果指针指向的单元值不为零，向前跳转到对应的[指令的次一指令处

#[derive(Debug, PartialEq)]
pub enum Opcode {
    // > python hex(ord('>')) -> 0x3e
    SHR = 0x3E,
    // < python hex(ord('<')) -> 0x3c
    SHL = 0x3C,
    // + python hex(ord('+')) -> 0x2b
    ADD = 0x2B,
    // - python hex(ord('-')) -> 0x2d
    SUB = 0x2D,
    // . python hex(ord('.')) -> 0x2e
    PUTCHAR = 0x2E,
    // , python hex(ord(',')) -> 0x2c
    GETCHAR = 0x2F,
    // [ python hex(ord('[')) -> 0x5b
    LB = 0x5B,
    // ] python hex(ord(']')) -> 0x5d
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2F => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];

        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();
        for (i, e) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i);
            }
            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty list")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }
        Ok(Code { instrs, jtable })
    }
}
