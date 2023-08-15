use std::io;

pub fn wc(args: &Vec<String>) {
    // let mut n_lines: Vec<usize> = vec![0];
    // let mut n_words: Vec<usize> = vec![0];
    // let mut n_bytes: Vec<usize> = vec![0];
    let mut sum_n_lines = 0;//n_lines.iter().sum::<usize>();
    let mut sum_n_words = 0;//n_words.iter().sum::<usize>();
    let mut sum_n_bytes = 0;//n_bytes.iter().sum::<usize>();
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
                break;
            }
            // input.clear();
        }
    }
}
