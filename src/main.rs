// Crate Name
#![allow(non_snake_case)]

use std::env;

fn message_to_print_until_die() ->  String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() >= 1 {
        args.join(" ")
    } else {
        "y".to_string()
    }
}

fn main() {
    loop {
        println!("{}", message_to_print_until_die());
    }
}
