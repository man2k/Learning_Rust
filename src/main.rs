// #![warn(non_snake_case)]
// #![allow(unused)] // to ignore errors with unused variables
// use std::io::*; For importing everything
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name");
    //all variables are immutable by default
    let mut name: String = String::new(); //function to create a empty string
    let greeting: &str = "by man2K"; // immutable variable
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive input");
    println!("Hello, {}! {}", name.trim_end(), greeting); // if ! is present then it is a macro
}
