use std::fs::File;
use std::io::{self, BufRead};
pub fn cat(args: &Vec<String>){
    let file = File::open(&args[2]).expect("failed to open file");
    let reader = io::BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result.expect("failed to read line");
        println!("{}", line);
    }
}