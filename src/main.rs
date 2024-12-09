#![crate_name = "rust_test_project"]

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use rust_test_project::add_constant;

fn main() {
    do_a_string();
    do_a_number();
}

fn do_a_string() {
    // Comment me up
    const RAW_PROMPT: &str = "This shit is bananas, B A N A N A S: ";
    let prompt: String = String::from(RAW_PROMPT);
    let mut funky_string = String::new();

    println!("{}", RAW_PROMPT);
    print!("{}", prompt);
    let _flush = stdout().flush();

    stdin().read_line(&mut funky_string).unwrap();
    blah(funky_string);
}

/// Blah's a String
pub fn blah(a_string: String) -> () {
    println!("You know it {}", a_string);
}

fn do_a_number() {
    let prompt: String = String::from("Smash in a number: ");
    let mut num = String::new();

    print!("{}", prompt);
    let _flush = stdout().flush();

    stdin().read_line(&mut num).unwrap();

    let n = num.trim().parse().unwrap();
    println!("{}", add_constant(n));
}