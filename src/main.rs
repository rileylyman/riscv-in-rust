use std::env::args;

mod decoder;
use decoder::*;

mod implementer;
use implementer::{*, rtype::*, itype::*, utype::*, stype::*, ujtype::*, sbtype::*};

mod assembler;
use assembler::*;

macro_rules! process {
    ($f:expr) => {
        if let Err(e) = $f {
            print!("Terminated: ");
            match e {
                ExecutionError::Extension(ext) => {
                    println!("The {} extension was not activated", ext)
                }
                ExecutionError::InstructionAddressMisaligned =>{
                    println!("Instruction address misaligned exception")
                }
                ExecutionError::InvalidInstruction(inst) => {
                    println!("{} is an invalid instruction", inst)
                }
                ExecutionError::Unimplemented(inst) => {
                    println!("The {} instruction is not implemented", inst)
                }
                ExecutionError::UserTerminate => {
                    println!("The user terminated the program")
                }
            }
            break;
        }
    }
}

const REGFILE_SIZE: usize = 32;
const MEM_SIZE: usize = 1048576 * 4; // 32 address space in RV32I 

const INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD: i32 = 4;

#[allow(dead_code)]
pub struct Extensions {
    m: bool,
    a: bool,
    f: bool
}

pub enum ExecutionError {
    Extension(String),
    InvalidInstruction(String),
    InstructionAddressMisaligned,
    Unimplemented(String),  
    UserTerminate
}

fn print_registers(regfile: &mut [u32]) {
    for (i, r) in regfile.into_iter().enumerate() {
        if *r != 0 {
            println!("x{}: 0x{:08x}", i, *r as i32);
        }
    }
}

fn get_extensions() -> Box<Extensions> {
    let mut a = false;
    let mut m = false;
    let mut f = false;
    for arg in args() {
        if arg == "-a" || arg == "-G" { a = true; }
        else if arg == "-m" || arg == "-G" { m = true; }
        else if arg == "-f" || arg == "-G" { f = true; }
    }
    Box::new(Extensions { m, a, f })
}

fn get_filepath() -> Result<String, ()> {
    for arg in args() {
        if arg.get(0..5).unwrap_or("") == "-src=" {
            return Ok(arg.get(5..).unwrap_or("./risc-v/sources/test.S").into());
        }
    }
    Err(())
}

fn fetch_inst(pc: u32, imem: &[u8]) -> Result<&[u8], String> {
    
    if (pc as usize + 4) > imem.len() { return Err("End of imem".into()); }

    match get_bits(imem[pc as usize]) {
        32 => {
            let bytes = &imem[(pc as usize)..(pc as usize) + 4];
            if encode_hex(bytes) == "00000000".to_string() || encode_hex(bytes) == "11111111".to_string() {
                Err(format!("The instruction 0x{} is illegal", encode_hex(bytes)))
            }
            else { Ok(bytes) }
        }
        _ => {
            Err("Only 32 bit instructions are supported".into())
        }
    }
    
}

fn main() {
    
    let mut imem: Vec<u8> = Vec::new();
    let mut regfile: Vec<u32> = vec![0; REGFILE_SIZE];
    let mut mem: Vec<u8> = vec![0; MEM_SIZE];

    let extensions   = get_extensions();
    let src_filepath = get_filepath().expect("Source Risc-V file not specified.");

    assemble_and_load(src_filepath, &mem, &imem);
    
    let mut pc: u32 = 0;
    loop {
        
        let bytes = match fetch_inst(pc, &imem) { Ok(b) => b, _ => break };

        match get_opcode(bytes) {
            0x3 | 0x13 | 0x1B | 0x67 | 0x73 => { 
                process! {
                    handle_i_type(&mut regfile, &mut mem, bytes, &mut pc, &extensions)
                }
            }
            0x17 | 0x37 => { 
                process! {
                    handle_u_type(&mut regfile, bytes, &mut pc, &extensions)
                }
            }
            0x23 => { 
                process! {
                    handle_s_type(&mut regfile, &mut mem, bytes, &mut pc, &extensions)
                }
            }
            0x33 | 0x3B => { 
                process! {
                    handle_r_type(&mut regfile, bytes, &mut pc, &extensions)
                }
            }
            0x63 => { 
                process! {
                    handle_sb_type(&mut regfile, bytes, &mut pc, &extensions)
                }
            }
            0x6F => { 
                process! {
                    handle_uj_type(&mut regfile, bytes, &mut pc, &extensions)
                }
            }
            0xFF => {
                process! {
                    handle_fence(&mut regfile, bytes, &mut pc)
                }
            }
            _ => {
                println!("UNRECOGNIZED OPCODE: {}", get_opcode(bytes));
                println!("{:?}", bytes);
                break;
            }
        }
        
        regfile[0] = 0;
        print_registers(&mut regfile);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
