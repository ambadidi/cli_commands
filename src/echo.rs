// use std::env;
pub fn echo(args: &Vec<String>){
    if !args[2].contains('-') {
        let my_args = &args[2..];
        for arg in my_args {
            print!("{arg} ");
        }
        println!();
    }
    else if args[2].contains('-') {
        let short_op = &args[2];
        let my_args = &args[3..];
        if short_op == "-n" {
            for arg in my_args {
                print!("{arg} ");
            }
        }
    }
}