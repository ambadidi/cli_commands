use std::fs::File;
use std::io::{self, BufRead};
pub fn cat(args: &Vec<String>){
    if args[2] == "-n" {
        let my_files = &args[3..].to_owned();
        for my_file in my_files {
            let file = File::open(&my_file).expect("failed to open file in -n");
            let reader = io::BufReader::new(file);
            for (i, line_result) in reader.lines().enumerate() {
                let line = line_result.expect("failed to read line");
                println!("     {}  {}", i+1, line);
            }
        }
        return;
    }
    if args[2] == "-e" {
        let my_files = &args[3..].to_owned();
        for my_file in my_files {
            let file = File::open(&my_file).expect("failed to open file in -e");
            let reader = io::BufReader::new(file);
            for line_result in reader.lines() {
                let line = line_result.expect("failed to read line");
                println!("{}$",line);
            }
        }
        return;
    }
    if args[2] == "-b" {
        let my_files = &args[3..].to_owned();
        let mut line_number = 1;
        for my_file in my_files {
            let file = File::open(&my_file).expect("failed to open file in -b");
            let reader = io::BufReader::new(file);
            for line_result in reader.lines() {
                let line = line_result.expect("failed to read line");
                if line.trim().is_empty() {
                    println!();
                } else {
                    println!("     {}  {}", line_number, line);
                    line_number += 1;
                }
            }
        }
        return;
    }



    let my_files = &args[2..].to_owned();
    for my_file in my_files {
        let file = File::open(&my_file).expect("failed to open file");
        let reader = io::BufReader::new(file);
        for line_result in reader.lines() {
            let line = line_result.expect("failed to read line");
            println!("{}", line);
        }
    }
}