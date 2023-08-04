mod echo;
mod cat;
use echo::*;
use cat::*;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let cmd = &arguments[1];
    if cmd == "echo" {
        echo(&arguments);
    }
    if cmd == "cat" {
        cat(&arguments);
    }
}
