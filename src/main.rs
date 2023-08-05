mod echo;
mod cat;
mod ls;
use echo::*;
use cat::*;
use ls::*;
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
    if cmd == "ls" {
        ls(&arguments);
    }
}
