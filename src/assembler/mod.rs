use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const INST_WIDTH: usize = 32;

pub fn assemble_and_load(filepath: &str, mem: &mut [u8], imem: &mut [u8]) -> () {
    let src: String = read_to_string(filepath);
    
    let text: Vec<String> = get_section_text(&src).unwrap();
    let base_instructions = resolve_labels(&text).expect("Could not resolve all labels");
    let machine_code = assemble(&base_instructions);
}

fn assemble(instructions: &Vec<String>) {

}

fn resolve_labels(text: &Vec<String>) -> Result<Vec<String>, &'static str>{
    //TODO: resolve global references
    
    let mut label_map: HashMap<&str, usize> = HashMap::new();
    let mut ret: Vec<String> = Vec::new();

    for i in 0..text.len() {
        if let Some(num) = text[i].find(':') { 
            label_map.insert(text[i].get(0..num).expect("Could not parse label"), i);
            ret.push(text[i].get(num..).expect("Could not remove label").into());
        }
        else {
            ret.push(text[i].clone());
        }
    }

    for i in 0..ret.len() {
        for (label, inst) in &label_map {
            ret[i] = ret[i].replace(label, &inst.to_string());
        }
    }

    Ok(ret)
}

fn get_section_text(src: &String) -> Option<Vec<String>> {
    
    //TODO: Allow other .sections after .text
    if let Some(text_start) = src.find(".text") {
        Some(
            src.get(text_start + 5 ..)
                .expect("Could not parse text section")
                .split("\n")
                .map(|s: &str| s.into())
                .collect()
        )
    } else if src.split(".section").collect::<Vec<&str>>().len() == 1 {
        Some(
            src.split("\n")
                .map(|s: &str| s.into())
                .collect()
        )
    }  else {
        None
    }
}

fn read_to_string(filepath: &str) -> String {
    let mut file = File::open(filepath).or_else(|_| return Err("Could not open instruction file")).unwrap();
    let mut instructions = String::new();
    file.read_to_string(&mut instructions).or_else(|_| return Err("Error reading file")).unwrap();
    instructions
}