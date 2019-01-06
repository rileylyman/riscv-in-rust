use std::fs::File;
use std::io::prelude::*;
use std::env::args;

mod decode;
use decode::*;

mod implement;
use implement::*;

macro_rules! process {
    ($f:expr) => {
        if let Err(e) = $f {
            instruction_error(e);
            break;
        }
    }
}

const REGFILE_SIZE: usize = 32;
const MEM_SIZE: usize = 1048576 * 4; // 32 address space in RV32I 

const INSTRUCTIONS: &'static str = "./risc-v/assembled/test.hex";

const INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD: i32 = 4;

#[allow(dead_code)]
pub struct Extensions {
    m: bool,
    a: bool,
    f: bool
}

fn load_into_imem(filepath: &str, imem: &mut Vec<u8>) -> Result<(), &'static str> {
    let mut file = File::open(filepath).or_else(|_| return Err("Could not open instruction file")).unwrap();
    let mut instructions = String::new();
    file.read_to_string(&mut instructions).or_else(|_| return Err("Error reading file")).unwrap();

    for mut hex_str in instructions.split(|c: char| !(c.is_digit(16) || c == 'x')) {

        if let Some("0x") = hex_str.get(0..2) { hex_str = hex_str.get(2..).expect("Op was only 0x?"); }
        let bytes = decode_hex_to_bytes(hex_str).expect("Could not decode instruction to bytes");

        match get_bits(bytes[bytes.len() - 1]) {
            16 => { return Err("16-bit instructions not supported"); } // 16 bit instruction
            32 => { // 32 bit instruction

                if bytes.len() == 4 {
                    imem.push(bytes[3]);
                    imem.push(bytes[2]);
                    imem.push(bytes[1]);
                    imem.push(bytes[0]);
                }
                else { return Err("32-bit instruction does not contain 4 bytes, but more or less"); }

            } 
            48 => { return Err("48-bit instructions not supported"); } // 48 bit instruction
            64 => { return Err("64-bit instructions not supported"); } // 64 bit instruction
            _  => { return Err(">=80-bit instructions not supported"); } // >= 80 bit instruction
        }

    }
    
    Ok(())
}

fn print_registers(regfile: &mut [u32]) {
    for (i, r) in regfile.into_iter().enumerate() {
        if *r != 0 {
            println!("x{}: 0x{:08x}", i, *r as i32);
        }
    }
}

fn get_extensions() -> Extensions {
    let mut a = false;
    let mut m = false;
    let mut f = false;
    for arg in args() {
        if arg == "-a" || arg == "-G" { a = true; }
        else if arg == "-m" || arg == "-G" { m = true; }
        else if arg == "-f" || arg == "-G" { f = true; }
    }
    Extensions { m, a, f }
}

fn instruction_error(e: &'static str) {
    println!("Error while executing instruction: {}", e);
}

fn main() {
    
    let extensions = get_extensions();

    let mut imem: Vec<u8> = Vec::new();
    let mut regfile: [u32; REGFILE_SIZE] = [0; REGFILE_SIZE];
    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];

    load_into_imem(INSTRUCTIONS, &mut imem).unwrap();
    
    let mut pc: u32 = 0;
    loop {
        
        if (pc as usize + 4) >= imem.len() { break; }

        let bytes = match get_bits(imem[pc as usize]) {
            32 => &imem[(pc as usize)..(pc as usize) + 4],
            _ => panic!("Only 32 bit instructions are supported")
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
