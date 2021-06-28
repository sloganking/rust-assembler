use std::io::prelude::*;
use std::fs::File;

const REGS: [&str; 4] = ["A", "B", "C", "D"];

fn is_a_label(token: &str)-> bool {
    false
}

fn return_type(token: &str) -> Option<&str>{
    let mut token = token.to_string();
    let mut is_address = false;
    if token.starts_with("[") && token.ends_with("]"){
        is_address = true;
        token = token.replace("[","");
        token = token.replace("]","");
    }

    // if token represents int
    if let Ok(i) = token.parse::<u32>(){
        if is_address{
            return Some("[const]")
        }else{
            return Some("const")
        }
    };

    if REGS.contains(&&token[..]){
        if is_address{
            return Some("[reg]")
        }else{
            return Some("reg")
        }
    }

    if is_a_label(token){
        if is_address{
            return Some("[const]")
        }else{
            return Some("const")
        }
    }

    None

}

fn main() {

    // get data from files
        // get asm string from file
            let mut file = File::open("./input.asm").expect("Unable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read the file");
        // get mapping for instructions to binary
            let mut file = File::open("./instrucToBinary.json").expect("Unable to open the file");
            let mut instruc_to_binary_file = String::new();
            file.read_to_string(&mut instruc_to_binary_file).expect("Unable to read the file");
            let instruc_to_binary: serde_json::Value = serde_json::from_str(&instruc_to_binary_file).expect("JSON was not well-formatted");
    // parse fileString into tokens
        let lines: Vec<&str> = contents.split("\n").collect();

        // remove comments (anything after a ";" char)
            let mut lines_no_comments = Vec::new();
            for line in lines.into_iter() {
                let commentless_lines = match line.find(";") {
                    Some(x) => line[0..x].to_string(),
                    None => line.to_string()
                };
                // println!("{}",commentless_lines);

                lines_no_comments.push(commentless_lines);
            }
        // remove ","
            lines_no_comments = lines_no_comments.into_iter().map(|x| x.replace(",", "")).collect();
        // remove "\r"
            lines_no_comments = lines_no_comments.into_iter().map(|x| x.replace("\r", "")).collect();


        // tokenize 
            // tokenizes but yeilds some empty string tokens
                let mut tokens: Vec<Vec<String>> = lines_no_comments.into_iter().map(|x| x.split(" ").map(|x| x.to_string()).collect()).collect();
            // remove empty string tokens
                let emptyless_tokens: Vec<Vec<String>> = tokens.into_iter().map(|x| x.into_iter().filter(|x| x != "").collect()).collect();
            // remove empty lines
                let raw_tokens: Vec<Vec<String>> = emptyless_tokens.into_iter().filter(|x| x.len() != 0).collect();
                println!("{:?}", raw_tokens);
        // generate instruction mapping
            


    println!("{}",return_type("[A"));

    // turn to machine code

}
