// Crate Name
#![allow(non_snake_case)]

use std::env;

fn get_message_to_print_until_die() -> String {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() >= 1 {
        args.join(" ")
    } else {
        "y".to_string()
    }
}

fn main() {
    let message_to_print_until_die = get_message_to_print_until_die();

    loop {
        println!("{}", message_to_print_until_die);
    }
}
