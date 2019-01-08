use super::*;

pub fn handle_sb_type(regfile: &mut[u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), ExecutionError> {
    let opcode          = get_opcode(bytes);
    let f3              = get_f3(bytes);
    let rs1             = get_rs1(bytes) as usize;
    let rs2             = get_rs2(bytes) as usize;

    let immediate = decode_sb_immediate(bytes);

    if opcode == 0x63 && f3 == 0x0 { // beq
        if regfile[rs1 as usize] == regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x1 { // bne
        if regfile[rs1 as usize] != regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x4 { // blt
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x5 { // bge
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x6 { //bltu 
        if regfile[rs1 as usize] < regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x7 { //bgeu
        if regfile[rs1 as usize] >= regfile[rs2 as usize] {
            if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
                return Err(ExecutionError::InstructionAddressMisaligned);
            }
            *pc = ((*pc as i32) + immediate) as u32;
        } else {
            *pc += 4;
        }
    }
    else {
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }

    Ok(())
}