mod echo;
use echo::*;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let cmd = &arguments[1];
    if cmd == "echo" {
        echo(&arguments);
    }
}
