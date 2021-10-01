#![allow(non_snake_case)]

use std::env;

fn get_message() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() >= 1 {
        args.join(" ")
    } else {
        "y".to_string()
    }
}

fn main() {
    let print_until_dead = get_message();

    loop {
        println!("{}", print_until_dead);
    }
}
