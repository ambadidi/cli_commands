use std::fs;
use std::path::PathBuf;

pub fn ls(args: &Vec<String>) {
    if args.len() == 2 {
        let mut entries: Vec<PathBuf> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by(|a,b| {
            let a_str = a.file_name().unwrap().to_string_lossy();
            let b_str = b.file_name().unwrap().to_string_lossy();
            if a_str.to_lowercase() == b_str {
                std::cmp::Ordering::Greater
            } else if b_str.to_lowercase() == a_str {
                std::cmp::Ordering::Less
            } else {
                a_str.to_lowercase().cmp(&b_str.to_lowercase())
            }
        });
        for entry in entries {
            let bold = "\x1b[1m";
            let blue = "\x1b[34m";
            let reset = "\x1b[0m";
            let file_name = entry.file_name().unwrap().to_string_lossy();
            if &file_name[0..1] == "." {
                continue;
            }
            if entry.is_dir() {
                print!("{}{}{}{}  ",bold, blue ,file_name, reset);
            } else {
                print!("{}{}  ",file_name, reset);
            }
        }
        println!();
        return;
    }
    if args[2] == "-a" {
        let mut entries: Vec<PathBuf> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.push(PathBuf::from("././"));
        entries.push(PathBuf::from("./../"));
        entries.sort_by(|a, b| {
            let mut a_str = a
                .file_name()
                .map(|s| s.to_str().unwrap_or(""))
                .unwrap_or_else(|| a.to_str().unwrap_or(""));
            let mut b_str = b
                .file_name()
                .map(|s| s.to_str().unwrap_or(""))
                .unwrap_or_else(|| b.to_str().unwrap_or(""));
            if a_str.starts_with(".") && a_str.len() >= 2 {
                a_str = &a_str[1..];
            }
            if b_str.starts_with(".") && b_str.len() >= 2 {
                b_str = &b_str[1..];
            }
            if a_str.to_lowercase() == b_str.to_string() {
                std::cmp::Ordering::Greater
            } else if b_str.to_lowercase() == a_str.to_string() {
                std::cmp::Ordering::Less
            } else {
                a_str.to_lowercase().cmp(&b_str.to_lowercase())
            }
        });
        for i in 0..entries.len() {
            for j in 0..entries.len() - i - 1 {
                let file_name_j = entries[j].components().last().unwrap().as_os_str().to_str();
                let file_name_j1 = entries[j+1].components().last().unwrap().as_os_str().to_str();
                if file_name_j == Some("..") && file_name_j1 == Some(".") {
                    entries.swap(j, j + 1);
                }
            }
        }
        for entry in entries {
            let bold = "\x1b[1m";
            let blue = "\x1b[34m";
            let reset = "\x1b[0m";
            let file_name = entry
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy();
            if entry.is_dir() {
                print!("{}{}{}{}  ",bold, blue ,file_name, reset);
            } else {
                print!("{}{}  ",file_name, reset);
            }
        }
        println!();
        return;
    }
}
