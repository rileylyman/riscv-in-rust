#![allow(overflowing_literals)]

use super::decode::*;
use super::*;

fn mulh(first: u32, second: u32, weight: i64) -> u32 {
    (((first as i64) * (second as i64) * weight) >> 32) as u32
}

pub fn handle_r_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, extensions: &Extensions) -> Result<(), &'static str> {
    
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes) as usize;
    let f3     = get_f3(bytes);
    let rs1    = get_rs1(bytes) as usize;
    let rs2    = get_rs2(bytes) as usize;
    let f7     = get_f7(bytes) as u32;

    if opcode == 0x33 && f3 == 0x0 && f7 == 0x0 { // add
        regfile[rd] = regfile[rs1] + regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x0 && f7 == 0x20 { // sub
        regfile[rd] = regfile[rs1] - regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x1 && f7 == 0x00 { // sll
        regfile[rd] = regfile[rs1] << regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x2 && f7 == 0x00 { // slt
        regfile[rd] = if (regfile[rs1] as i32) < (regfile[rs2] as i32) { 1 } else { 0 }; 
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x3 && f7 == 0x00 { // sltu
        regfile[rd] = if regfile[rs1] < regfile[rs2] { 1 } else { 0 };
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x4 && f7 == 0x00 { // xor
        regfile[rd] = regfile[rs1] ^ regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x5 && f7 == 0x00 { // srl
        regfile[rd] = regfile[rs1] >> regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x5 && f7 == 0x20 { // sra
        regfile[rd] = ((regfile[rs1] as i32) >> (regfile[rs2] as i32)) as u32;
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x6 && f7 == 0x00 { // or
        regfile[rd] = regfile[rs1] | regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x7 && f7 == 0x00 { // and
        regfile[rd] = regfile[rs1] & regfile[rs2];
        *pc += 4;
    }
    else if opcode == 0x33 && f3 == 0x0 && f7 == 0x1 { //mul
        if extensions.m {
            regfile[rd] = ((regfile[rs1] as u64) * (regfile[rs2] as u64)) as u32;
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x1 && f7 == 0x1 { //mulh
        if extensions.m {
            let mut first = regfile[rs1];
            let mut second = regfile[rs2];

            let mut weight = 1;
            if (first as i32) < 0 { 
                first = ((first as i32) * -1) as u32; 
                weight *= -1; 
            }
            if (second as i32) < 0 { 
                second = ((second as i32) * -1) as u32; 
                weight *= -1; 
            }

            regfile[rd] = mulh(first, second, weight);
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x2 && f7 == 0x1 { //mulhsu
        if extensions.m {
            let mut first = regfile[rs1];
            let second = regfile[rs2];

            let mut weight = 1;
            if (first as i32) < 0 { 
                first = ((first as i32) * -1) as u32; 
                weight *= -1; 
            }

            regfile[rd] = mulh(first, second, weight);
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x3 && f7 == 0x1 { //mulhu
        if extensions.m {
            regfile[rd] = (((regfile[rs1] as u64) * (regfile[rs2] as u64)) >> 32) as u32;
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x4 && f7 == 0x1 { //div
        if extensions.m {
            if regfile[rs2] == 0 {
                regfile[rd] = 0xFF_FF_FF_FF;
            }
            else if (regfile[rs1] as i32) == -0x80000000 && (regfile[rs2] as i32) == -0x1 {
                regfile[rd] = regfile[rs1];
            }
            else {
                regfile[rd] = ((regfile[rs1] as i32) / (regfile[rs2] as i32)) as u32;
            }
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x5 && f7 == 0x1 { //divu
        if extensions.m {
            if regfile[rs2] == 0 {
                regfile[rd] = 0xFF_FF_FF_FF;
            }
            else {
                regfile[rd] = regfile[rs1] / regfile[rs2];
            }
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x6 && f7 == 0x1 { //rem
        if extensions.m {
            if regfile[rs2] == 0 {
                regfile[rd] = regfile[rs1];
            }
            else if (regfile[rs1] as i32) == -0x80000000 && (regfile[rs2] as i32) == -0x1 {
                regfile[rd] = 0;
            }
            else {
                regfile[rd] = ((regfile[rs1] as i32) % (regfile[rs2] as i32)) as u32;
            }
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else if opcode == 0x33 && f3 == 0x7 && f7 == 0x1 { //remu
        if extensions.m {
            if regfile[rs2] == 0 {
                regfile[rd] = regfile[rs1];
            }
            else {
                regfile[rd] = regfile[rs1] % regfile[rs2];
            }
            *pc += 4;
        }
        else {
            return Err("M Extension not in use");
        }
    }
    else {
        return Err("Invalid R-Type Instruction");
    }
    
    Ok(())
}

pub fn handle_i_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), &'static str> {

    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes) as usize;
    let f3     = get_f3(bytes);
    let rs1    = get_rs1(bytes) as usize;
    let f7     = get_f7(bytes) as u32;

    let immediate = decode_i_type_immediate(bytes);

    if opcode == 0x3 && f3 == 0x0 { //lb
        let byte: u32 = mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] as u32;
        regfile[rd] = if byte >> 7 == 0x1 {
            0xFF_FF_FF_00 + byte
        } else {
           byte
        }; 
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x1 { //lh
        let bottom = mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let top = mem[((regfile[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        let total = bottom + (top << 8);
        regfile[rd] = if top >> 7 == 0x1 {
            0xFF_FF_00_00 + total 
        } else {
            total
        };
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x2 { //lw
        let bottom = mem[((regfile[rs1] as i32) + immediate) as usize] as u32;
        let low_mid = mem[((regfile[rs1] as i32) + immediate + 1) as usize] as u32;
        let high_mid = mem[((regfile[rs1] as i32) + immediate + 2) as usize] as u32;
        let top = mem[((regfile[rs1] as i32) + immediate + 3) as usize] as u32;
        regfile[rd] = bottom + (low_mid << 8) + (high_mid << 16) + (top << 24);
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x4 { //lbu
        regfile[rd] = mem[((regfile[rs1] as i32) + immediate) as usize] as u32;
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x5 { //lhu
        let bottom = mem[((regfile[rs1] as i32) + immediate) as usize] as u32;
        let top = mem[((regfile[rs1] as i32) + immediate + 1) as usize] as u32;
        regfile[rd] = bottom + (top << 8);
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x0 { //addi
        regfile[rd] = ((regfile[rs1] as i32) + immediate) as u32;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x1 && f7 == 0x0 { //slli
        regfile[rd] = regfile[rs1] << immediate;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x2 { //slti
        regfile[rd] = if (regfile[rs1] as i32) < immediate { 1 } else { 0 };
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x3 { //sltiu
        regfile[rd] = if regfile[rs1] < (immediate as u32) { 1 } else { 0 };
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x4 { //xori
        regfile[rd] = regfile[rs1] ^ (immediate as u32);
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x0 { //srli
        regfile[rd] = regfile[rs1] >> (immediate as u32); 
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x20 { //srai
        regfile[rd] = ((regfile[rs1] as i32) >> immediate) as u32;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x6 { //ori
        regfile[rd] = regfile[rs1] | (immediate as u32);
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x7 { //andi
        regfile[rd] = regfile[rs1] & (immediate as u32);
        *pc += 4;
    }
    else if opcode == 0x67 && f3 == 0x0 { // jalr
        let destination = ((regfile[rs1] as i32) + immediate) & 0xFF_FF_FF_FE;
        if destination % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
            return Err("Instruction address misaligned exception");
        }
        regfile[rd] = *pc + 4;
        *pc = destination as u32;
    }
    else if opcode == 0x73 && f3 == 0x0 && f7 == 0x0 { //ecall
        match regfile[10] {
            0x1 => {
                println!("PRINT ECALL: {}", regfile[11]);
            }
            0xA => {
                println!("TERMINATE ECALL");
                return Err("Program terminated");
            }
            _ => {}
        } 
    }
    else if opcode == 0x73 && f3 == 0x0 && f7 == 0x1 { //ebreak
        return Err("ebreak: unimplemented instruction");
    }
    else {
        return Err("Invalid I-Type Instruction");
    }

    Ok(())
}

pub fn handle_s_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), &'static str> {
    
    let opcode   = get_opcode(bytes);
    let f3       = get_f3(bytes);
    let rs1      = get_rs1(bytes) as usize;
    let rs2      = get_rs2(bytes) as usize;

    let immediate = decode_s_type_immediate(bytes);

    if opcode == 0x23 && f3 == 0x0 { //sb
        mem[((regfile[rs1] as i32) + immediate) as usize] = regfile[rs2] as u8;

        *pc += 4;
    }
    else if opcode == 0x23 && f3 == 0x1 { //sh
        let word = regfile[rs2] as u32;
        let bottom = word as u8;
        let top = (word >> 8) as u8;
        mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] = bottom;
        mem[((regfile[rs1] as i32) + (immediate as i32) + 1) as usize] = top;

        *pc += 4;
    }
    else if opcode == 0x23 && f3 == 0x2 { //sw
        let word = regfile[rs2] as u32;
        let bottom = word as u8;
        let low_mid = (word >> 8) as u8;
        let high_mid = (word >> 16) as u8;
        let top = (word >> 24) as u8;
        mem[((regfile[rs1] as i32) + (immediate as i32))     as usize] = bottom;
        mem[((regfile[rs1] as i32) + (immediate as i32) + 1) as usize] = low_mid;
        mem[((regfile[rs1] as i32) + (immediate as i32) + 2) as usize] = high_mid;
        mem[((regfile[rs1] as i32) + (immediate as i32) + 3) as usize] = top;

        *pc += 4;
    }
    else { 
        return Err("Invalid S-Type Instruction");
    }

    Ok(())
}

pub fn handle_sb_type(regfile: &mut[u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), &'static str> {
    let opcode          = get_opcode(bytes);
    let f3              = get_f3(bytes);
    let rs1             = get_rs1(bytes) as usize;
    let rs2             = get_rs2(bytes) as usize;

    let immediate = decode_sb_immediate(bytes);

    if opcode == 0x63 && f3 == 0x0 { // beq
        if regfile[rs1 as usize] == regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x1 { // bne
        if regfile[rs1 as usize] != regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x4 { // blt
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x5 { // bge
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x6 { //bltu 
        if regfile[rs1 as usize] < regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x7 { //bgeu
        if regfile[rs1 as usize] >= regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err("Instruction address misaligned exception");
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else {
        return Err("Invalid SB-Type Instruction");
    }

    Ok(())
}

pub fn handle_u_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), &'static str> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let immediate = decode_u_type_immediate(bytes);

    if opcode == 0x17 { // auipc
        regfile[rd as usize] = ((*pc as i32) + immediate) as u32;
        *pc += 4;
    }
    else if opcode == 0x37 { // lui
        regfile[rd as usize] = immediate as u32;
        *pc += 4;
    }
    else { 
        return Err("Invalid U-Type Instruction");
    }

    Ok(())
}

pub fn handle_uj_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), &'static str> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let immediate = decode_uj_type_immediate(bytes);

    if opcode == 0x6F { //jal 
        if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
            return Err("Instruction address misaligned exception");
        }
        regfile[rd as usize] = *pc + 4;
        *pc = ((*pc as i32) + immediate) as u32;
    }
    else {
        return Err("Invalid UJ-Type Instruction")
    }

    Ok(())
}
