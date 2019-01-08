macro_rules! process {
    ($f:expr) => {
        if let Err(e) = $f {
            print!("Terminated: ");
            match e {
                ExecutionError::Extension(ext) => {
                    println!("The {} extension was not activated", ext)
                }
                ExecutionError::InstructionAddressMisaligned =>{
                    println!("Instruction address misaligned exception")
                }
                ExecutionError::InvalidInstruction(inst) => {
                    println!("{} is an invalid instruction", inst)
                }
                ExecutionError::Unimplemented(inst) => {
                    println!("The {} instruction is not implemented", inst)
                }
                ExecutionError::UserTerminate => {
                    println!("The user terminated the program")
                }
            }
            break;
        }
    }
}

macro_rules! parse_extensions {
    ([$ap:ident; $ext:ident] $($t:ident),*) => {
        $(
            $ap.refer(&mut $ext.$t).add_option(&[
                Box::leak(format!("-{}", stringify!($t)).into_boxed_str())
            ], StoreTrue, Box::leak(format!("Enable {} extension", stringify!($t)).into_boxed_str()));
        )*

    }
}