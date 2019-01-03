use std::fs::File;
use std::io::prelude::*;

const IMEM_SIZE: usize = 2048;
const REGFILE_SIZE: usize = 32;
const MEM_SIZE: usize = 1048576 * 4;

const INSTRUCTIONS: &'static str = "./risc-v/assembled/test.hex";

fn load_into_imem(filepath: &str, imem: &mut [u8]) -> std::io::Result<usize> {
    let mut file = File::open(filepath)?;
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)?;

    let mut len = 0;

    for (i, c) in instructions.chars().enumerate() {

        let shift_bits = if i % 2 == 0 { 4 } else { 0 };
        len = (i / 2) + 1;

        match c {
            '0' => { 
                imem[i / 2] += 0x0 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '1' => { 
                imem[i / 2] += 0x1 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '2' => { 
                imem[i / 2] += 0x2 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '3' => { 
                imem[i / 2] += 0x3 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '4' => { 
                imem[i / 2] += 0x4 << shift_bits ;
                println!("{}:{}:{}", c, i, imem[i]);
            }
            '5' => { 
                imem[i / 2] += 0x5 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '6' => { 
                imem[i / 2] += 0x6 << shift_bits ;
                println!("{}:{}:{}", c, i, imem[i]);
            }
            '7' => { 
                imem[i / 2] += 0x7 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '8' => { 
                imem[i / 2] += 0x8 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            '9' => { 
                imem[i / 2] += 0x9 << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            'A' | 'a' => { 
                imem[i / 2] += 0xA << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            'B' | 'b' => { 
                imem[i / 2] += 0xB << shift_bits ;
                println!("{}:{}:{}", c, i, imem[i]);
            }
            'C' | 'c' => { 
                imem[i / 2] += 0xC << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            'D' | 'd' => { 
                imem[i / 2] += 0xD << shift_bits ;
                println!("{}:{}:{}", c, i, imem[i]);
            }
            'E' | 'e' => { 
                imem[i / 2] += 0xE << shift_bits;
                println!("{}:{}:{}", c, i, imem[i]); 
            }
            'F' | 'f' => { 
                imem[i / 2] += 0xF << shift_bits ;
                println!("{}:{}:{}", c, i, imem[i]);
            }
            _         => { 
                return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
            }
        }
    }
    Ok(len)
}

fn handle_r_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    
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
    else {
        return Err("Invalid R-Type Instruction");
    }
    
    Ok(())
}

fn handle_i_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {

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
        let bottom = mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let low_mid = mem[((regfile[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        let high_mid = mem[((regfile[rs1] as i32) + (immediate as i32) + 2) as usize] as u32;
        let top = mem[((regfile[rs1] as i32) + (immediate as i32) + 3) as usize] as u32;
        regfile[rd] = bottom + (low_mid << 8) + (high_mid << 16) + (top << 24);
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x4 { //lbu
        regfile[rd] = mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] as u32;
        *pc += 4;
    }
    else if opcode == 0x3 && f3 == 0x5 { //lhu
        let bottom = mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] as u32;
        let top = mem[((regfile[rs1] as i32) + (immediate as i32) + 1) as usize] as u32;
        regfile[rd] = bottom + (top << 8);
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x0 { //addi
        regfile[rd] = regfile[rs1] + immediate;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x1 && f7 == 0x0 { //slli
        regfile[rd] = regfile[rs1] << immediate;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x2 { //slti
        regfile[rd] = if (regfile[rs1] as i32) < (immediate as i32) { 1 } else { 0 };
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x3 { //sltiu
        regfile[rd] = if regfile[rs1] < immediate { 1 } else { 0 };
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x4 { //xori
        regfile[rd] = regfile[rs1] ^ immediate;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x0 { //srli
        regfile[rd] = regfile[rs1] >> raw_immediate; 
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x5 && f7 == 0x20 { //srai
        regfile[rd] = ((regfile[rs1] as i32) >> (immediate as i32)) as u32;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x6 { //ori
        regfile[rd] = regfile[rs1] | immediate;
        *pc += 4;
    }
    else if opcode == 0x13 && f3 == 0x7 { //andi
        regfile[rd] = regfile[rs1] & immediate;
        *pc += 4;
    }
    else {
        return Err("Invalid I-Type Instruction")
    }

    Ok(())
}

fn handle_s_type(regfile: &mut [u32], mem: &mut [u8], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    
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
        mem[((regfile[rs1] as i32) + (immediate as i32)) as usize] = regfile[rs2] as u8;

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

fn handle_sb_type(regfile: &mut[u32], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
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
        if regfile[rs1 as usize] == regfile[rs2 as usize] {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x1 { // bne
        if regfile[rs1 as usize] != regfile[rs2 as usize] {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x4 { // blt
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x5 { // bge
        if (regfile[rs1 as usize] as i32) < (regfile[rs2 as usize] as i32) {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x6 { //bltu 
        if regfile[rs1 as usize] < regfile[rs2 as usize] {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else if opcode == 0x63 && f3 == 0x7 { //bgeu
        if regfile[rs1 as usize] >= regfile[rs2 as usize] {
            *pc += immediate
        } else {
            *pc += 4;
        }
    }
    else {
        return Err("Invalid SB-Type Instruction");
    }

    Ok(())
}

fn handle_u_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
    let opcode = get_opcode(bytes);
    let rd     = get_rd(bytes);

    let immediate = (((bytes[1] as u32) >> 4) + ((bytes[2] as u32) << 4) + ((bytes[3] as u32) << 12)) << 12;

    if opcode == 0x17 { // auipc
        regfile[rd as usize] = ((*pc as i32) + (immediate as i32)) as u32;
        *pc += 4;
    }
    else if opcode == 0x37 { // lui
        regfile[rd as usize] = immediate;
        *pc += 4;
    }
    else { 
        return Err("Invalid U-Type Instruction");
    }

    Ok(())
}

fn handle_uj_type(regfile: &mut [u32], bytes: &[u8], pc: &mut u32) -> Result<(), &'static str> {
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
        regfile[rd as usize] = *pc + 4;
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

fn print_registers(regfile: &mut [u32]) {
    for (i, r) in regfile.into_iter().enumerate() {
        if *r != 0 {
            println!("x{}: {}", i, *r as i32);
        }
    }
}

fn main() {
    
    let mut imem: [u8; IMEM_SIZE] = [0; IMEM_SIZE];
    let mut regfile: [u32; REGFILE_SIZE] = [0; REGFILE_SIZE];
    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];

    let length = load_into_imem(INSTRUCTIONS, &mut imem).unwrap();
    println!("{}", length);
    
    let mut pc: u32 = 0;
    while pc < (length as u32) {
        
        let mut bytes: [u8;4] = [0;4]; 
        for i in 0..4 {
            bytes[i] = imem[(pc + 3 - (i as u32)) as usize]
        }

        match get_opcode(&bytes) {
            0x3 | 0x13 | 0x1B | 0x67 | 0x73 => { 
                handle_i_type(&mut regfile, &mut mem, &bytes, &mut pc).unwrap(); 
            }
            0x17 | 0x37 => { 
                handle_u_type(&mut regfile, &bytes, &mut pc).unwrap();
            }
            0x23 => { 
                handle_s_type(&mut regfile, &mut mem, &bytes, &mut pc).unwrap(); 
            }
            0x33 | 0x3B => { 
                handle_r_type(&mut regfile, &bytes, &mut pc).unwrap(); 
            }
            0x63 => { 
                handle_sb_type(&mut regfile, &bytes, &mut pc).unwrap(); 
            }
            0x6F => { 
                handle_uj_type(&mut regfile, &bytes, &mut pc).unwrap(); 
            }
            _ => {
                break;
            }
        }
        
        regfile[0] = 0;
        print_registers(&mut regfile);

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
