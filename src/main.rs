mod cat;
mod echo;
mod ls;
mod wc;
use cat::*;
use echo::*;
use ls::*;
use std::env;
use wc::*;

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
    if cmd == "wc" {
        wc(&arguments);
    }
}
