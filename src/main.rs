extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

mod decoder;
use decoder::*;

mod implementer;
use implementer::{*, rtype::*, itype::*, utype::*, stype::*, ujtype::*, sbtype::*};

mod assembler;
use assembler::*;

#[macro_use]
mod macro_definitions;


const REGFILE_SIZE: usize = 32;
const MEM_SIZE: usize = 1048576 * 4; // 32 address space in RV32I 

const INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD: i32 = 4;

#[allow(dead_code)]
pub struct Extensions {
    m: bool,
    a: bool,
    f: bool,
    e: bool,
    c: bool,
    d: bool,
    q: bool,
}

pub enum ExecutionError {
    Extension(String),
    InvalidInstruction(String),
    InstructionAddressMisaligned,
    Unimplemented(String),  
    UserTerminate
}

fn main() {
    
    let mut imem: Vec<u8> = Vec::new();
    let mut regfile: Vec<u32> = vec![0; REGFILE_SIZE];
    let mut mem: Vec<u8> = vec![0; MEM_SIZE];

    let mut src_filepath: String = "./risc-v/sources/test.S".into();
    let mut extensions = Extensions{ a: false, m: false, e: false, f: false, d: false, q: false, c: false };
    let mut use_hex = false;

    {
        let mut ap = ArgumentParser::new();

        parse_extensions!([ap;extensions] a, m, e, f, d, q, c);

        ap.refer(&mut src_filepath)
            .add_option(&["--file"], Store, "File to emulate");
        ap.refer(&mut use_hex)
            .add_option(&["--hex", "-h"], StoreTrue, "Set if the source file is assembled hex");
        ap.parse_args_or_exit();
    }
 
    if use_hex {
        if let Err(e) = load_into_imem(&src_filepath, &mut imem) { 
            println!("Error loading into IMEM: {}", e); 
        }
    } else {
        assemble_and_load(&src_filepath, &mut mem, &mut imem);
    }
    
    let mut pc: u32 = 0;
    loop {
        
        let bytes = match fetch_inst(pc, &imem) { 
            Ok(b) => b, 
            Err(e) => {
                println!("{}", e);
                break;
            } 
        };

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
                println!("Instruction was: {:?}", bytes);
                break;
            }
        }
        
        regfile[0] = 0;
        print_registers(&mut regfile);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

// Helper functions

fn print_registers(regfile: &mut [u32]) {
    for (i, r) in regfile.into_iter().enumerate() {
        if *r != 0 {
            println!("x{}: 0x{:08x}", i, *r as i32);
        }
    }
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
