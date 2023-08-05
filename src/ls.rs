use std::fs;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn ls(args: &Vec<String>) {
    if args.len() > 2 && args[2] == "-a" {
        let mut entries: Vec<OsString> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.push(PathBuf::from(".").as_os_str().to_os_string());
        entries.push(PathBuf::from("..").as_os_str().to_os_string());
        for entry in entries {
            print!("{} ", entry.to_string_lossy());
        }
        println!();
        return;
    }
    if args.len() == 2 {
        let entries: Vec<OsString> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        // let mut not_dots: Vec<OsString> = Vec::new();
        for entry in entries {
            if entry.to_str().map(|f| f.starts_with(".")).unwrap() {
                continue;
            }
            print!("{}  ", entry.to_string_lossy());
        }
        println!();
        return;
    }
}

