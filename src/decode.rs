#![allow(overflowing_literals)]

use std::num::ParseIntError;
use std::fmt::Write;

// Decode an even length hex string into its constituent bytes
// Code adapted from StackOverflow user `Sven Marnach`
pub fn decode_hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
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

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn test_decode_sb_immediate() {
        let predicted_imm = decode_sb_immediate(&[0x63,0x2,0x0,0x0]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 4); 
        assert_eq!(predicted_imm, 4);

        let predicted_imm = decode_sb_immediate(&[0xe3, 0x0e, 0x00, 0xfe]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -4); 
        assert_eq!(predicted_imm, -4);
    }

    #[test]
    fn test_decode_u_immediate() {
        let predicted_imm = decode_u_type_immediate(&[0xb7, 0xf2, 0xff, 0xff]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0xFFFFF000); 
        assert_eq!(predicted_imm, 0xFFFFF000);

        let predicted_imm = decode_u_type_immediate(&[0xb7, 0x02, 0x0f, 0x0f]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x0F0F0000); 
        assert_eq!(predicted_imm, 0x0F0F0000);
    }

    #[test]
    fn test_decode_i_immediate() {
        let predicted_imm = decode_i_type_immediate(&[0x93, 0x02, 0xf0, 0x7f]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x7FF); 
        assert_eq!(predicted_imm, 0x7FF);

        let predicted_imm = decode_i_type_immediate(&[0x93, 0x02, 0x10, 0x80]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x7FF); 
        assert_eq!(predicted_imm, -0x7FF);
    }

    #[test]
    fn test_decode_s_immediate() {
        let predicted_imm = decode_s_type_immediate(&[0xa3, 0xaf, 0x52, 0x02]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x3F); 
        assert_eq!(predicted_imm, 0x3F);

        let predicted_imm = decode_s_type_immediate(&[0xa3, 0xa0, 0x52, 0xfc]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x3F); 
        assert_eq!(predicted_imm, -0x3F);
    }

    #[test]
    fn test_decode_uj_immediate() {
        let predicted_imm = decode_uj_type_immediate(&[0x6f, 0x00, 0x40, 0x08]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x84); 
        assert_eq!(predicted_imm, 0x84);

        let predicted_imm = decode_uj_type_immediate(&[0x6f, 0xf0, 0xdf, 0xf7]);
        println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x84); 
        assert_eq!(predicted_imm, -0x84);
    }
} 