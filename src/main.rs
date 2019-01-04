use std::fs::File;
use std::io::prelude::*;


const IMEM_SIZE: usize = 2048;
const REGFILE_SIZE: usize = 32;
const MEM_SIZE: usize = 1048576 * 4;

const INSTRUCTIONS: &'static str = "./risc-v/assembled/test.hex";

const INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD: i32 = 4;

mod decode;
use decode::*;

mod implement;
use implement::*;


fn load_into_imem(filepath: &str, imem: &mut [u8]) -> Result<(), &'static str> {
    let mut file = File::open(filepath).or_else(|_| return Err("Could not open instruction file")).unwrap();
    let mut instructions = String::new();
    file.read_to_string(&mut instructions).or_else(|_| return Err("Error reading file")).unwrap();

    let mut index = 0;
    for hex_str in instructions.split(|c: char| !c.is_digit(16)) {

        let bytes = decode_hex_to_bytes(hex_str).expect("Could not decode instruction to bytes");

        match get_bits(bytes[bytes.len() - 1]) {
            16 => { return Err("16-bit instructions not supported"); } // 16 bit instruction
            32 => { // 32 bit instruction

                if bytes.len() == 4 {
                    imem[index] = bytes[3];
                    imem[index + 1] = bytes[2];
                    imem[index + 2] = bytes[1];
                    imem[index + 3] = bytes[0];
                }
                else { return Err("32-bit instruction does not contain 4 bytes, but more or less"); }

                index += 4;

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
            println!("x{}: {}", i, *r as i32);
        }
    }
}

fn main() {
    
    let mut imem: [u8; IMEM_SIZE] = [0; IMEM_SIZE];
    let mut regfile: [u32; REGFILE_SIZE] = [0; REGFILE_SIZE];
    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];

    load_into_imem(INSTRUCTIONS, &mut imem).unwrap();
    
    let mut pc: u32 = 0;
    loop {
        
        let bytes = match get_bits(imem[pc as usize]) {
            32 => &imem[(pc as usize)..(pc as usize) + 4],
            _ => panic!("Only 32 bit instructions are supported")
        };

        match get_opcode(bytes) {
            0x3 | 0x13 | 0x1B | 0x67 | 0x73 => { 
                handle_i_type(&mut regfile, &mut mem, bytes, &mut pc).unwrap(); 
            }
            0x17 | 0x37 => { 
                handle_u_type(&mut regfile, bytes, &mut pc).unwrap();
            }
            0x23 => { 
                handle_s_type(&mut regfile, &mut mem, bytes, &mut pc).unwrap(); 
            }
            0x33 | 0x3B => { 
                handle_r_type(&mut regfile, bytes, &mut pc).unwrap(); 
            }
            0x63 => { 
                handle_sb_type(&mut regfile, bytes, &mut pc).unwrap(); 
            }
            0x6F => { 
                handle_uj_type(&mut regfile, bytes, &mut pc).unwrap(); 
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
