use super::*;

pub fn handle_s_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32, _ext: &Extensions) -> Result<(), ExecutionError> {
    
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
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }

    Ok(())
}