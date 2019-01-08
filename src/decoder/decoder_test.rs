use super::*;

#[test]
fn test_decode_sb_immediate() {
    let predicted_imm = decode_sb_immediate(&[0x63,0x2,0x0,0x0]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 4); 
    assert_eq!(predicted_imm, 4);

    let predicted_imm = decode_sb_immediate(&[0xe3, 0x0e, 0x00, 0xfe]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -4); 
    assert_eq!(predicted_imm, -4);
}

#[test]
fn test_decode_u_immediate() {
    let predicted_imm = decode_u_type_immediate(&[0xb7, 0xf2, 0xff, 0xff]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0xFFFFF000); 
    assert_eq!(predicted_imm, 0xFFFFF000);

    let predicted_imm = decode_u_type_immediate(&[0xb7, 0x02, 0x0f, 0x0f]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x0F0F0000); 
    assert_eq!(predicted_imm, 0x0F0F0000);
}

#[test]
fn test_decode_i_immediate() {
    let predicted_imm = decode_i_type_immediate(&[0x93, 0x02, 0xf0, 0x7f]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x7FF); 
    assert_eq!(predicted_imm, 0x7FF);

    let predicted_imm = decode_i_type_immediate(&[0x93, 0x02, 0x10, 0x80]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x7FF); 
    assert_eq!(predicted_imm, -0x7FF);
}

#[test]
fn test_decode_s_immediate() {
    let predicted_imm = decode_s_type_immediate(&[0xa3, 0xaf, 0x52, 0x02]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x3F); 
    assert_eq!(predicted_imm, 0x3F);

    let predicted_imm = decode_s_type_immediate(&[0xa3, 0xa0, 0x52, 0xfc]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x3F); 
    assert_eq!(predicted_imm, -0x3F);
}

#[test]
fn test_decode_uj_immediate() {
    let predicted_imm = decode_uj_type_immediate(&[0x6f, 0x00, 0x40, 0x08]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, 0x84); 
    assert_eq!(predicted_imm, 0x84);

    let predicted_imm = decode_uj_type_immediate(&[0x6f, 0xf0, 0xdf, 0xf7]);
    println!("Decoded: {:032b}\n Actual: {:032b}", predicted_imm, -0x84); 
    assert_eq!(predicted_imm, -0x84);
}