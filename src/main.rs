use std::io::prelude::*;
use std::fs::File;

mod assembler;

fn main() {

    // get asm string from file
        let mut file = File::open("./input.asm").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
       
    let machine_code = assembler::assemble(&contents);
    println!("{:?}",machine_code);
}
