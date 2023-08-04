// use std::env;
pub fn echo(args: &Vec<String>){
    if args[2].contains('-') {
        let short_op = &args[2];
        let my_args = &args[3..].to_owned();
        if short_op == "-n" {
            for arg in my_args {
                print!("{arg} ");
            }
            return;
        }
        if short_op == "-e" {
            let sub = my_args.join(" ");
            let sub_vec = sub.split("\\n").collect::<Vec<_>>();
            for s in sub_vec {
                let s = s.replace("\\t", "\t");
                let s = s.replace("\\b", "\x08");
                let s = s.replace("\\v", "\x0B");
                if s.contains("\\c") {
                    let s = s.split("\\c").take(1).collect::<String>();
                    print!("{}", s);
                    return;
                }
                println!("{}", s);
            }
            return;
        }
    }
    let my_args = &args[2..].to_owned();
    for arg in my_args {
        print!("{arg} ");
    }
    println!();
}