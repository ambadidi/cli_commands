use chrono::prelude::*;
use file_mode::ModePath;
use file_owner::PathExt;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub fn ls(args: &Vec<String>) {
    let bold = "\x1b[1m";
    let blue = "\x1b[34m";
    let reset = "\x1b[0m";
    let paths = args
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>();
    if args.len() == 2 {
        let mut entries: Vec<PathBuf> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        sort_non_hidden(&mut entries);
        for entry in entries {
            // let bold = "\x1b[1m";
            // let blue = "\x1b[34m";
            // let reset = "\x1b[0m";
            let file_name = entry
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy();
            if &file_name[0..1] == "." {
                continue;
            }
            if entry.is_dir() {
                print!("{}{}{}{}  ", bold, blue, file_name, reset);
            } else {
                print!("{}{}  ", file_name, reset);
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
        sort_with_hidden(&mut entries);
        for i in 0..entries.len() {
            for j in 0..entries.len() - i - 1 {
                let file_name_j = entries[j].components().last().unwrap().as_os_str().to_str();
                let file_name_j1 = entries[j + 1]
                    .components()
                    .last()
                    .unwrap()
                    .as_os_str()
                    .to_str();
                if file_name_j == Some("..") && file_name_j1 == Some(".") {
                    entries.swap(j, j + 1);
                }
            }
        }
        for entry in entries {
            // let bold = "\x1b[1m";
            // let blue = "\x1b[34m";
            // let reset = "\x1b[0m";
            let file_name = entry
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy();
            if entry.is_dir() {
                print!("{}{}{}{}  ", bold, blue, file_name, reset);
            } else {
                print!("{}{}  ", file_name, reset);
            }
        }
        println!();
        return;
    }
    if args[2] == "-l" {
        let mut entries: Vec<PathBuf> = fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        sort_non_hidden(&mut entries);
        // let total_blocks = get_total_blocks(&entries);
        // println!("total {total_blocks}");
        for entry in entries {
            // let bold = "\x1b[1m";
            // let blue = "\x1b[34m";
            // let reset = "\x1b[0m";
            let file_name = entry
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_string_lossy();
            if &file_name[0..1] == "." {
                continue;
            }
            let mode = entry.mode().unwrap().to_string();
            let file_size = entry
                .metadata()
                .map(|m| m.len())
                .expect("problem with file_size metadata");
            let n_hard_link = entry.metadata().map(|m| m.nlink()).unwrap();
            let modif_time = entry.metadata().map(|m| m.mtime()).unwrap();
            let (month, day, hour_minutes) = convert_ts(modif_time);
            let owner_name = entry.owner().map(|o| o.name().unwrap()).unwrap().unwrap();
            let group_name = entry.group().map(|o| o.name().unwrap()).unwrap().unwrap();
            if entry.is_dir() {
                println!("{mode} {n_hard_link} {owner_name} {group_name} {file_size:>5} {month} {day:>2} {hour_minutes} {bold}{blue}{file_name}{reset}");
            } else {
                println!("{mode} {n_hard_link} {owner_name} {group_name} {file_size:>5} {month} {day:>2} {hour_minutes} {file_name}{reset}");
            }
        }
    }
    if args.len() == 3 {
        let path = paths.last().unwrap();
            if path.is_dir() {
                let mut entries = fs::read_dir(path).unwrap().map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, _>>().unwrap();
                sort_non_hidden(&mut entries);
                for entry in entries {
                    let file_name = entry
                        .components()
                        .last()
                        .unwrap()
                        .as_os_str()
                        .to_string_lossy();
                    if &file_name[0..1] == "." {
                        continue;
                    }
                    if entry.is_dir() {
                        print!("{}{}{}{}  ", bold, blue, file_name, reset);
                    } else {
                        print!("{}{}  ", file_name, reset);
                    }
                }
                println!();
            } else if path.is_file() {
                println!("{}", path.components().last().unwrap().as_os_str().to_string_lossy());
            }
    }
}
fn sort_non_hidden(entries: &mut Vec<PathBuf>) {
    entries.sort_by(|a, b| {
        let a_str = a.file_name().unwrap().to_string_lossy();
        let b_str = b.file_name().unwrap().to_string_lossy();
        if a_str.to_lowercase() == b_str {
            std::cmp::Ordering::Greater
        } else if b_str.to_lowercase() == a_str {
            std::cmp::Ordering::Less
        } else {
            a_str.to_lowercase().cmp(&b_str.to_lowercase())
        }
    })
}
fn sort_with_hidden(entries: &mut Vec<PathBuf>) {
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
    })
}

fn convert_ts(ts: i64) -> (String, String, String) {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
    let dt: DateTime<FixedOffset> = DateTime::from_utc(nt, *Local::now().offset());
    let hour_minutes = dt.format("%H:%M");
    let day = dt.format("%-d");
    let month = dt.format("%b");
    (month.to_string(), day.to_string(), hour_minutes.to_string())
}
// fn get_total_blocks(entries: &Vec<PathBuf>) -> u64 {
//     let mut total_size = 0;
//     for entry in entries {
//         let file_name = entry
//                 .components()
//                 .last()
//                 .unwrap()
//                 .as_os_str()
//                 .to_string_lossy();
//             if &file_name[0..1] == "." {
//                 continue;
//             }
//             println!("{entry:?}");
//         let metadata = entry.metadata().unwrap();
//         total_size += metadata.blocks();
//     }
//     // let block_size = 1024;
//     // (total_size + block_size - 1) / block_size
//     total_size
// }
