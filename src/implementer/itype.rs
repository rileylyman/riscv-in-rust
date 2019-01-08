use super::*;

pub fn handle_i_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), ExecutionError> {

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
            return Err(ExecutionError::InstructionAddressMisaligned);
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
                return Err(ExecutionError::UserTerminate);
            }
            _ => {}
        } 
    }
    else if opcode == 0x73 && f3 == 0x1 { //csrrw
        return Err(ExecutionError::Unimplemented("csrrw".into()));
    }
    else if opcode == 0x73 && f3 == 0x2 { //csrrs
        return Err(ExecutionError::Unimplemented("csrrs".into()));
    }
    else if opcode == 0x73 && f3 == 0x3 { //csrrc
        return Err(ExecutionError::Unimplemented("csrrc".into()));
    }
    else if opcode == 0x73 && f3 == 0x4 { //csrrwi
        return Err(ExecutionError::Unimplemented("csrrwi".into()));
    }
    else if opcode == 0x73 && f3 == 0x5 { //csrrsi
        return Err(ExecutionError::Unimplemented("csrrsi".into()));
    }
    else if opcode == 0x73 && f3 == 0x6 { //csrrci
        return Err(ExecutionError::Unimplemented("csrrci".into()));
    }
    else {
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }

    Ok(())
}