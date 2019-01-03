use std::fs::File;
use std::io::prelude::*;

const IMEM_SIZE: usize = 2048;
const IMEM: [u8; IMEM_SIZE] = [0; IMEM_SIZE];

const REGFILE_SIZE: usize = 32;
const REGFILE: [u32; REGFILE_SIZE] = [0; REGFILE_SIZE];

const MEM_SIZE: usize = 1048576 * 4;
const MEM: [u8; MEM_SIZE] = [0; MEM_SIZE];


fn load_into_imem(filepath: &str) -> std::io::Result<()> {
    let mut file = File::open(filepath)?;
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)?;

    for (i, c) in instructions.chars().enumerate() {
        let shift_bits = if i % 2 == 0 { 4 } else { 0 };
        match c {
            '0'       => IMEM[i] += 0x0 << shift_bits,
            '1'       => IMEM[i] += 0x1 << shift_bits,
            '2'       => IMEM[i] += 0x2 << shift_bits,
            '3'       => IMEM[i] += 0x3 << shift_bits,
            '4'       => IMEM[i] += 0x4 << shift_bits,
            '5'       => IMEM[i] += 0x5 << shift_bits,
            '6'       => IMEM[i] += 0x6 << shift_bits,
            '7'       => IMEM[i] += 0x7 << shift_bits,
            '8'       => IMEM[i] += 0x8 << shift_bits,
            '9'       => IMEM[i] += 0x9 << shift_bits,
            'A' | 'a' => IMEM[i] += 0xA << shift_bits,
            'B' | 'b' => IMEM[i] += 0xB << shift_bits,
            'C' | 'c' => IMEM[i] += 0xC << shift_bits,
            'D' | 'd' => IMEM[i] += 0xD << shift_bits,
            'E' | 'e' => IMEM[i] += 0xE << shift_bits,
            'F' | 'f' => IMEM[i] += 0xF << shift_bits,
            _         => {
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
            }
        }
    }
    Ok(())
}

fn handle_r_type(bytes: &[u8]) -> Result<(), &str> {
    
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes) as usize;
    let f3     = get_f3(bytes);
    let rs1    = get_rs1(bytes) as usize;
    let rs2    = get_rs2(bytes) as usize;
    let f7     = get_f7(bytes) as u32;

    if opcode == 0x33 && f3 == 0x0 && f7 == 0x0 { // add
        REGFILE[rd] = REGFILE[rs1] + REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x0 && f7 == 0x20 { // sub
        REGFILE[rd] = REGFILE[rs1] - REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x1 && f7 == 0x00 { // sll
        REGFILE[rd] = REGFILE[rs1] << REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x2 && f7 == 0x00 { // slt
        REGFILE[rd] = if (REGFILE[rs1] as i32) < (REGFILE[rs2] as i32) { 1 } else { 0 }; 
    }
    else if opcode == 0x33 && f3 == 0x3 && f7 == 0x00 { // sltu
        REGFILE[rd] = if REGFILE[rs1] < REGFILE[rs2] { 1 } else { 0 };
    }
    else if opcode == 0x33 && f3 == 0x4 && f7 == 0x00 { // xor
        REGFILE[rd] = REGFILE[rs1] ^ REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x5 && f7 == 0x00 { // srl
        REGFILE[rd] = REGFILE[rs1] >> REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x5 && f7 == 0x20 { // sra
        REGFILE[rd] = ((REGFILE[rs1] as i32) >> (REGFILE[rs2] as i32)) as u32;
    }
    else if opcode == 0x33 && f3 == 0x6 && f7 == 0x00 { // or
        REGFILE[rd] = REGFILE[rs1] | REGFILE[rs2];
    }
    else if opcode == 0x33 && f3 == 0x7 && f7 == 0x00 { // and
        REGFILE[rd] = REGFILE[rs1] & REGFILE[rs2];
    }
    else {
        return Err("Invalid R-Type Instruction");
    }
    
    Ok(())
}

fn handle_i_type(bytes: &[u8]) -> Result<(), &str> {

    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes) as usize;
    let f3     = get_f3(bytes);
    let rs1    = get_rs1(bytes) as usize;
    let f7     = get_f7(bytes) as u32;

    let raw_immediate: u32 = ((bytes[3] as u32) << 4) + ((bytes[2] >> 4) as u32);
    let immediate = if (0x800 & raw_immediate) == 0x800 {
        0xFF_FF_F0_00 + raw_immediate
    } else {
        raw_immediate
    };

    if opcode == 0x3 && f3 == 0x0 { //lb
        let byte: u32 = MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] as u32;
        REGFILE[rd] = if byte >> 7 == 0x1 {
            0xFF_FF_FF_00 + byte
        } else {
           byte
        }; 
    }
    else if opcode == 0x3 && f3 == 0x1 { //lh
        let bottom = MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let top = MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        let total = bottom + (top << 8);
        REGFILE[rd] = if top >> 7 == 0x1 {
            0xFF_FF_00_00 + total 
        } else {
            total
        };
    }
    else if opcode == 0x3 && f3 == 0x2 { //lw
        let bottom = MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let low_mid = MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        let high_mid = MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 2) as usize] as u32;
        let top = MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 3) as usize] as u32;
        REGFILE[rd] = bottom + (low_mid << 8) + (high_mid << 16) + (top << 24);
    }
    else if opcode == 0x3 && f3 == 0x4 { //lbu
        REGFILE[rd] = MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] as u32;
    }
    else if opcode == 0x3 && f3 == 0x5 { //lhu
        let bottom = MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let top = MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        REGFILE[rd] = bottom + (top << 8);
    }
    else if opcode == 0x13 && f3 == 0x0 { //addi
        REGFILE[rd] = REGFILE[rs1] + immediate;
    }
    else if opcode == 0x13 && f3 == 0x1 && f7 == 0x0 { //slli
        REGFILE[rd] = REGFILE[rs1] << immediate;
    }
    else if opcode == 0x13 && f3 == 0x2 { //slti
        REGFILE[rd] = if (REGFILE[rs1] as i32) < (immediate as i32) { 1 } else { 0 };
    }
    else if opcode == 0x13 && f3 == 0x3 { //sltiu
        REGFILE[rd] = if REGFILE[rs1] < immediate { 1 } else { 0 };
    }
    else if opcode == 0x13 && f3 == 0x4 { //xori
        REGFILE[rd] = REGFILE[rs1] ^ immediate;
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x0 { //srli
        REGFILE[rd] = REGFILE[rs1] >> raw_immediate; 
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x20 { //srai
        REGFILE[rd] = ((REGFILE[rs1] as i32) >> (immediate as i32)) as u32;
    }
    else if opcode == 0x13 && f3 == 0x6 { //ori
        REGFILE[rd] = REGFILE[rs1] | immediate;
    }
    else if opcode == 0x13 && f3 == 0x7 { //andi
        REGFILE[rd] = REGFILE[rs1] & immediate;
    }
    else {
        return Err("Invalid I-Type Instruction")
    }

    Ok(())
}

fn handle_s_type(bytes: &[u8]) -> Result<(), &str> {
    
    let opcode   = get_opcode(bytes);
    let imm_4_0  = get_rd(bytes) as u32;
    let f3       = get_f3(bytes);
    let rs1      = get_rs1(bytes) as usize;
    let rs2      = get_rs2(bytes) as usize;
    let imm_11_5 = get_f7(bytes) as u32;

    let raw_immediate = (imm_11_5 << 5) + imm_4_0;
    let immediate = if (raw_immediate & 0x800) == 0x800 {
        0xFF_FF_F0_00 + raw_immediate
    } else {
        raw_immediate
    };

    if opcode == 0x23 && f3 == 0x0 { //sb
        MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] = REGFILE[rs2] as u8;
    }
    else if opcode == 0x23 && f3 == 0x1 { //sh
        let word = REGFILE[rs2] as u32;
        let bottom = word as u8;
        let top = (word >> 8) as u8;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32)) as usize] = bottom;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 1) as usize] = top;
    }
    else if opcode == 0x23 && f3 == 0x2 { //sw
        let word = REGFILE[rs2] as u32;
        let bottom = word as u8;
        let low_mid = (word >> 8) as u8;
        let high_mid = (word >> 16) as u8;
        let top = (word >> 24) as u8;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32))     as usize] = bottom;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 1) as usize] = low_mid;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 2) as usize] = high_mid;
        MEM[((REGFILE[rs1] as i32) + (immediate as i32) + 3) as usize] = top;
    }
    else { 
        return Err("Invalid S-Type Instruction");
    }

    Ok(())
}

fn handle_sb_type(bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    let opcode          = get_opcode(bytes);
    let imm_4_to_1_11  = get_rd(bytes) as u32;
    let f3              = get_f3(bytes);
    let rs1             = get_rs1(bytes) as usize;
    let rs2             = get_rs2(bytes) as usize;
    let imm_12_10_to_5 = get_f7(bytes) as u32;

    let raw_immediate = 0x0 + (((imm_4_to_1_11 as u32) >> 1) << 1) + (((imm_12_10_to_5 << 2) as u32) << 2) +
        ((((imm_4_to_1_11 << 7) >> 7) as u32) << 12) + (((imm_12_10_to_5 >> 6) as u32) << 13);
    
    let immediate = if raw_immediate >> 12 == 0x1 {
        0xFF_FF_E0_00 + raw_immediate
    } else {
        raw_immediate
    };

    if opcode == 0x63 && f3 == 0x0 { // beq
        if REGFILE[rs1 as usize] == REGFILE[rs2 as usize] {
            *pc += immediate
        }
    }
    else if opcode == 0x63 && f3 == 0x1 { // bne
        if REGFILE[rs1 as usize] != REGFILE[rs2 as usize] {
            *pc += immediate
        }
    }
    else if opcode == 0x63 && f3 == 0x4 { // blt
        if (REGFILE[rs1 as usize] as i32) < (REGFILE[rs2 as usize] as i32) {
            *pc += immediate
        }
    }
    else if opcode == 0x63 && f3 == 0x5 { // bge
        if (REGFILE[rs1 as usize] as i32) < (REGFILE[rs2 as usize] as i32) {
            *pc += immediate
        }
    }
    else if opcode == 0x63 && f3 == 0x6 { //bltu 
        if REGFILE[rs1 as usize] < REGFILE[rs2 as usize] {
            *pc += immediate
        }
    }
    else if opcode == 0x63 && f3 == 0x7 { //bgeu
        if REGFILE[rs1 as usize] >= REGFILE[rs2 as usize] {
            *pc += immediate
        }
    }
    else {
        return Err("Invalid SB-Type Instruction");
    }

    Ok(())
}

fn handle_u_type(bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let immediate = (((bytes[1] as u32) >> 4) + ((bytes[2] as u32) << 4) + ((bytes[3] as u32) << 12)) << 12;

    if opcode == 0x17 { // auipc
        REGFILE[rd as usize] = ((*pc as i32) + (immediate as i32)) as u32;
    }
    else if opcode == 0x37 { // lui
        REGFILE[rd as usize] = immediate;
    }
    else { 
        return Err("Invalid U-Type Instruction");
    }

    Ok(())
}

fn handle_uj_type(bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let raw_immediate = 0x0 + (((bytes[2] >> 5) << 1) as u32) + (((bytes[3] << 1) as u32) << 3) + 
        ((((bytes[2] << 3) >> 7) as u32) << 11) + (((bytes[1] >> 4) as u32) << 12) +
        (((bytes[2] << 4) as u32) << 12) + (((bytes[3] >> 7) as u32) << 20);

    let immediate = if (raw_immediate >> 20) == 0x1 {
        0xFF_E0_00_00 + raw_immediate
    } else {
        raw_immediate
    };

    if opcode == 0x6F { //jal 
        REGFILE[rd as usize] = *pc + 4;
        *pc = ((*pc as i32) + (immediate as i32)) as u32;
    }
    else {
        return Err("Invalid UJ-Type Instruction")
    }

    Ok(())
}

fn get_opcode(bytes: &[u8]) -> u8 { (bytes[0] << 1) >> 1 }

fn get_rd(bytes: &[u8]) -> u8 { ((bytes[1] << 4) >> 3) + (bytes[0] >> 7) }

fn get_f3(bytes: &[u8]) -> u8 { (bytes[1] << 1) >> 5 }

fn get_rs1(bytes: &[u8]) -> u8 { (bytes[1] >> 7) + ((bytes[2] << 4) >> 3) }

fn get_rs2(bytes: &[u8]) -> u8 { ((bytes[3] << 7) >> 3) + (bytes[2] >> 4) }

fn get_f7(bytes: &[u8]) -> u8 { bytes[3] >> 1 }

fn main() {
    println!("Hello, world!");
}
