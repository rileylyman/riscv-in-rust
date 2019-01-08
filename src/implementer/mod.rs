#![allow(overflowing_literals)]

use super::decoder::*;
use super::*;

pub mod rtype;
pub mod stype;
pub mod itype;
pub mod ujtype;
pub mod utype;
pub mod sbtype;

#[cfg(test)]
mod implementer_test;

pub fn handle_fence(_regfile: &mut [u32], bytes: &[u8], _pc: &mut u32) -> Result<(), ExecutionError> {
    let opcode = get_opcode(bytes);
    let _rd = get_rd(bytes);
    let f3 = get_f3(bytes);
    let _rs1 = get_rs1(bytes);

    //TODO: decode immediate

    if opcode == 0x00FF && f3 == 0x0 {
        return Err(ExecutionError::Unimplemented("FENCE".into()));
    }
    else if opcode == 0x00FF && f3 == 0x1 {
        return Err(ExecutionError::Unimplemented("FENCE.I".into()));
    }
    else {
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }
}
