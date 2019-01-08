#![allow(overflowing_literals)]

use std::num::ParseIntError;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;


// Decode an even length hex string into its constituent bytes
// Code adapted from StackOverflow user `Sven Marnach`
fn decode_hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len()).step_by(2).map(
        |k| {
            u8::from_str_radix(&hex[k..k + 2], 16)
        }
    ).collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).expect("Error writing bytes into hex string.");
    }
    s
} 

pub fn get_opcode(bytes: &[u8]) -> u8 { (bytes[0] << 1) >> 1 }

pub fn get_rd(bytes: &[u8]) -> u8 { ((bytes[1] << 4) >> 3) + (bytes[0] >> 7) }

pub fn get_f3(bytes: &[u8]) -> u8 { (bytes[1] << 1) >> 5 }

pub fn get_rs1(bytes: &[u8]) -> u8 { (bytes[1] >> 7) + ((bytes[2] << 4) >> 3) }

pub fn get_rs2(bytes: &[u8]) -> u8 { ((bytes[3] << 7) >> 3) + (bytes[2] >> 4) }

pub fn get_f7(bytes: &[u8]) -> u8 { bytes[3] >> 1 }

pub fn get_bits(first_byte: u8) -> i32 {
    match first_byte {
        b@_ if b & 0x03 <  0x03 => 16, // 16 bit instruction
        b@_ if b & 0x1F <  0x1F => 32, // 32 bit instruction
        b@_ if b & 0x3F == 0x1F => 48, // 48 bit instruction
        b@_ if b & 0x7F == 0x3F => 64, // 64 bit instruction
        _ => -1, // >= 80 bit instruction
    }
}

pub fn decode_sb_immediate(bytes: &[u8]) -> i32 {
    let mut imm = if bytes[3] >> 7 == 0x1 { 0xFF_FF_E0_00 } else { 0x0 };
    imm += ((bytes[3] >> 7) as u32) << 12;
    imm += ((bytes[0] >> 7) as u32) << 11;
    imm += (((bytes[3] << 1) >> 2) as u32) << 5;
    imm += (((bytes[1] << 4) >> 4) as u32) << 1;

    imm as i32
}

pub fn decode_u_type_immediate(bytes: &[u8]) -> i32 {
    (((((bytes[1] as u32) >> 4) + ((bytes[2] as u32) << 4) + ((bytes[3] as u32) << 12))) << 12) as i32
}

pub fn decode_uj_type_immediate(bytes: &[u8]) -> i32 {
    (if bytes[3] >> 7 == 0x1 { 0xFF_E0_00_00 } else { 0x0 }) + 
        ((((bytes[2] >> 5) << 1) as u32) + (((bytes[3] << 1) as u32) << 3) + 
        ((((bytes[2] << 3) >> 7) as u32) << 11) + (((bytes[1] >> 4) as u32) << 12) +
        (((bytes[2] << 4) as u32) << 12) + (((bytes[3] >> 7) as u32) << 20)) as i32
}

pub fn decode_s_type_immediate(bytes: &[u8]) -> i32 {
    ((get_f7(bytes) as i32) << 5) + (get_rd(bytes) as i32) + 
        if bytes[3] >> 7 == 0x1 { 0xFF_FF_F0_00 } else { 0x0 }
} 

pub fn decode_i_type_immediate(bytes: &[u8]) -> i32 {
    (((bytes[3] as u32) << 4) + ((bytes[2] >> 4) as u32) +
        if bytes[3] >> 7 == 0x1 { 0xFF_FF_F0_00 } else { 0x0 }) as i32
}

pub fn load_into_imem(filepath: &str, imem: &mut Vec<u8>) -> Result<(), &'static str> {
    let mut file = File::open(filepath).or_else(|_| return Err("Could not open instruction file")).unwrap();
    let mut instructions = String::new();
    file.read_to_string(&mut instructions).or_else(|_| return Err("Error reading file")).unwrap();

    for mut hex_str in instructions.split(|c: char| !(c.is_digit(16) || c == 'x')) {

        if hex_str.is_empty() { continue; }

        if let Some("0x") = hex_str.get(0..2) { hex_str = hex_str.get(2..).expect("Op was only 0x?"); }
        let bytes = decode_hex_to_bytes(hex_str).expect("Could not decode instruction to bytes");

        match get_bits(bytes[bytes.len() - 1]) {
            16 if bytes.len() == 2 => { return Err("16-bit instructions not supported"); } // 16 bit instruction
            32 if bytes.len() == 4 => { // 32 bit instruction

                if bytes.len() == 4 {
                    imem.push(bytes[3]);
                    imem.push(bytes[2]);
                    imem.push(bytes[1]);
                    imem.push(bytes[0]);
                }
                else { return Err("32-bit instruction does not contain 4 bytes, but more or less"); }

            } 
            48 if bytes.len() == 6 => { return Err("48-bit instructions not supported"); } // 48 bit instruction
            64 if bytes.len() == 8 => { return Err("64-bit instructions not supported"); } // 64 bit instruction
            _  => { return Err(">=80-bit instructions not supported, or incorrect no. of bytes encoded"); } // >= 80 bit instruction
        }

    }
    
    Ok(())
}

#[cfg(test)]
mod decoder_test;