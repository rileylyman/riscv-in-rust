use super::*;

pub fn handle_uj_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, _extensions: &Extensions) -> Result<(), ExecutionError> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let immediate = decode_uj_type_immediate(bytes);

    if opcode == 0x6F { //jal 
        if immediate % INSTRUCTION_ADDRESS_MISALIGNED_THRESHOLD != 0 {
            return Err(ExecutionError::InstructionAddressMisaligned);
        }
        regfile[rd as usize] = *pc + 4;
        *pc = ((*pc as i32) + immediate) as u32;
    }
    else {
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }

    Ok(())
}