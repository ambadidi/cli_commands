use std::io;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufRead;

pub fn wc(args: &Vec<String>) {
    let mut sum_n_lines = 0;
    let mut sum_n_words = 0;
    let mut sum_n_bytes = 0;
    let mut sum_n_chars;
    let mut total_n_lines = Vec::new();
    let mut total_n_words = Vec::new();
    let mut total_n_bytes = Vec::new();
    let mut total_n_chars = Vec::new();
    if args.len() == 2 {
        let mut input = String::new();
        while let Ok(n) = io::stdin().read_line(&mut input) {
            // Process input here
            let line = input.split("\\n").count();
            sum_n_lines += line;
            let word = input.split_whitespace().count();
            sum_n_words += word;
            let bytes = input.as_bytes().len();
            sum_n_bytes += bytes;
            input.clear();
            if n == 0 {
                // Handle EOF here
                sum_n_lines -= 1;
                println!("      {sum_n_lines}       {sum_n_words}       {sum_n_bytes}");
                return;
            }
        }
    }
    if args.len() > 2 {
        let paths = &args[2..].iter().map(PathBuf::from).collect::<Vec<PathBuf>>();
        for path in paths {
            if path.is_dir() {
                let file_name = path.clone().into_os_string().into_string().unwrap();
                println!("wc: {file_name}: Is a directory\n      0       0       0 {file_name}");
            }
            if path.is_file() {
                let file_name = path.clone().into_os_string().into_string().unwrap();
                let file = File::open(&path).expect("failed to open file in wc");
                let reader = io::BufReader::new(file);
                let mut n_lines = Vec::new();
                let mut n_words = Vec::new();
                let mut n_bytes = Vec::new();
                for line in reader.lines() {
                    let line = line.as_ref().unwrap();
                    n_lines.push(line.split("\\n").count());
                    n_words.push(line.split_whitespace().count());
                    n_bytes.push(line.as_bytes().len());
                }
                sum_n_lines = n_lines.iter().sum();
                total_n_lines.push(sum_n_lines);
                sum_n_words = n_words.iter().sum();
                total_n_words.push(sum_n_words);
                sum_n_bytes = n_bytes.iter().sum();
                sum_n_bytes += 1;
                total_n_bytes.push(sum_n_bytes);
                println!(" {sum_n_lines:<} {sum_n_words:<} {sum_n_bytes:>} {file_name:<}");
                if total_n_lines.len() > 1 {
                    let total_n_lines = total_n_lines.iter().sum::<usize>();
                    let total_n_words = total_n_words.iter().sum::<usize>();
                    let total_n_bytes = total_n_bytes.iter().sum::<usize>();
                    println!(" {total_n_lines:>} {total_n_words:>} {total_n_bytes:>} total");
                    return;
                }
            }
            if !path.is_dir() && !path.is_file() {
                let rest_args = &args[3..].iter().map(PathBuf::from).collect::<Vec<PathBuf>>();
                if path.to_str().unwrap() == "-l" {
                    for rest_arg in rest_args {
                        if rest_arg.is_dir() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            println!("wc: {file_name}: Is a directory\n0 {file_name}");
                        }
                        if rest_arg.is_file() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            let file = File::open(&rest_arg).expect("failed to open file in wc -l");
                            let reader = io::BufReader::new(file);
                            let mut n_lines = Vec::new();
                            for line in reader.lines() {
                                let line = line.as_ref().unwrap();
                                n_lines.push(line.split("\\n").count());
                            }
                            sum_n_lines = n_lines.iter().sum();
                            total_n_lines.push(sum_n_lines);
                            println!(" {sum_n_lines:<} {file_name:<}");
                            if total_n_lines.len() > 1 {
                                let total_n_lines = total_n_lines.iter().sum::<usize>();
                                println!(" {total_n_lines:>} total");
                                return;
                            }
                        }
                    }
                }
                if path.to_str().unwrap() == "-w" {
                    for rest_arg in rest_args {
                        if rest_arg.is_dir() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            println!("wc: {file_name}: Is a directory\n0 {file_name}");
                        }
                        if rest_arg.is_file() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            let file = File::open(&rest_arg).expect("failed to open file in wc -w");
                            let reader = io::BufReader::new(file);
                            let mut n_words = Vec::new();
                            for line in reader.lines() {
                                let line = line.as_ref().unwrap();
                                n_words.push(line.split_whitespace().count());
                            }
                            sum_n_words = n_words.iter().sum();
                            total_n_words.push(sum_n_words);
                            println!(" {sum_n_words:<} {file_name:<}");
                            if total_n_words.len() > 1 {
                                let total_n_words = total_n_words.iter().sum::<usize>();
                                println!(" {total_n_words:>} total");
                                return;
                            }
                        }
                    }
                }
                if path.to_str().unwrap() == "-c" {
                    for rest_arg in rest_args {
                        if rest_arg.is_dir() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            println!("wc: {file_name}: Is a directory\n0 {file_name}");
                        }
                        if rest_arg.is_file() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            let file = File::open(&rest_arg).expect("failed to open file in wc -c");
                            let reader = io::BufReader::new(file);
                            let mut n_bytes = Vec::new();
                            for line in reader.lines() {
                                let line = line.as_ref().unwrap();
                                n_bytes.push(line.as_bytes().len());
                            }
                            sum_n_bytes = n_bytes.iter().sum();
                            sum_n_bytes += 1;
                            total_n_bytes.push(sum_n_bytes);
                            println!(" {sum_n_bytes:<} {file_name:<}");
                            if total_n_bytes.len() > 1 {
                                let total_n_bytes = total_n_bytes.iter().sum::<usize>();
                                println!(" {total_n_bytes:>} total");
                                return;
                            }
                        }
                    }
                }
                if path.to_str().unwrap() == "-m" {
                    for rest_arg in rest_args {
                        if rest_arg.is_dir() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            println!("wc: {file_name}: Is a directory\n0 {file_name}");
                        }
                        if rest_arg.is_file() {
                            let file_name = rest_arg.clone().into_os_string().into_string().unwrap();
                            let file = File::open(&rest_arg).expect("failed to open file in wc -m");
                            let reader = io::BufReader::new(file);
                            let mut n_chars = Vec::new();
                            for line in reader.lines() {
                                let line = line.as_ref().unwrap();
                                n_chars.push(line.chars().count());
                            }
                            sum_n_chars = n_chars.iter().sum();
                            sum_n_chars += 1;
                            total_n_chars.push(sum_n_chars);
                            println!(" {sum_n_chars:<} {file_name:<}");
                            if total_n_chars.len() > 1 {
                                let total_n_chars = total_n_chars.iter().sum::<usize>();
                                println!(" {total_n_chars:>} total");
                                return;
                            }
                        }
                    }
                }
                return;
            }
        }
        if args.len() > 3 {
            println!("      {sum_n_lines}       {sum_n_words}       {sum_n_bytes} total");
        }
    }
}
