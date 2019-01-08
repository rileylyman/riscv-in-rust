use super::*;

pub fn handle_u_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), ExecutionError> {
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
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }

    Ok(())
}