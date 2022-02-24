mod opcode;

use opcode::Code;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[1])?;
    let code = Code::from(data)?;
    println!("{:?}", code.instrs);

    Ok(())
}
