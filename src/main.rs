use std::io::prelude::*;
use std::fs::File;

fn main() {

    // get asm string from file
        let mut file = File::open("./input.asm").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
    // parse fileString into tokens
        let mut lines: Vec<&str> = contents.split("\n").collect();

        // remove comments (anything after a ";" char)
            let mut lines_no_comments = Vec::new();
            for (i,line) in lines.iter().enumerate() {
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
            // for line in lines_no_comments{
            //     let tokens: Vec<String> = line.split(" ").collect();
            //     let tokens: Vec<&str> = tokens.into_iter().filter(|x| x != &"").collect();
            //     println!("{:?}", tokens);
            //     // tokens_no_empty.push(tokens);
            // }

            let tokens: Vec<Vec<String>> = lines_no_comments.into_iter().map(|x| x.split(" ").map(|x| x.to_string()).collect()).collect();

            println!("{:?}", tokens)

            // remove empty tokens
                // let mut tokens_no_empty = Vec::new();




        // let rawTokens: Vec<Vec<&str>> = lines_no_comments.iter().map(|x| x.split(" ")).collect();


            

        
        // println!("{:?}",lines_no_comments);
    // turn to machine code

}
