use super::*;

fn mulh(first: u32, second: u32, weight: i64) -> u32 {
    (((first as i64) * (second as i64) * weight) >> 32) as u32
}

pub fn handle_r_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32, extensions: &Extensions) -> Result<(), ExecutionError> {
    
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
            return Err(ExecutionError::Extension("M".into()));
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
            return Err(ExecutionError::Extension("M".into()));
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
            return Err(ExecutionError::Extension("M".into()));
        }
    }
    else if opcode == 0x33 && f3 == 0x3 && f7 == 0x1 { //mulhu
        if extensions.m {
            regfile[rd] = (((regfile[rs1] as u64) * (regfile[rs2] as u64)) >> 32) as u32;
            *pc += 4;
        }
        else {
            return Err(ExecutionError::Extension("M".into()));
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
            return Err(ExecutionError::Extension("M".into()));
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
            return Err(ExecutionError::Extension("M".into()));;
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
            return Err(ExecutionError::Extension("M".into()));
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
            return Err(ExecutionError::Extension("M".into()));
        }
    }
    else {
        return Err(ExecutionError::InvalidInstruction(encode_hex(bytes)));
    }
    
    Ok(())
}