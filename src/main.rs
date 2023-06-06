// #![warn(non_snake_case)]
#![allow(unused)] // to ignore errors with unused variables
                  // use std::io::*; For importing everything
use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // println!("What is your name");
    // //all variables are immutable by default
    // let mut name: String = String::new(); //function to create a empty string
    // let greeting: &str = "by man2K"; // immutable variable
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't Receive input");
    // println!("Hello, {}! {}", name.trim_end(), greeting); // if ! is present then it is a macro

    // const ONE_MIL: u32 = 1_000_000; // constant number of type integer(unsigned 32 bit) //big numbers can be separated with "_"

    // const PI: f32 = 3.141592; // constant number of type float 32

    // let age: &str = "47"; // type string

    // let mut age: u32 = age.trim().parse().expect(("Age wasn't assigned a number")); // shadowing -> two variables with same name and different types

    // age = age + 1;

    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // -----------------
    // Data types

    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max i64 : {}, Min i64 : {}", i64::MAX, i64::MIN);
    // println!("Max i128 : {}, Min i128 : {}", i128::MAX, i128::MIN);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64 : {}", f64::MAX);

    // //boolean
    // let is_true: bool = true;
    // let _is_true: bool = true; // unused variable but ignored because it starts with "_"
    // let my_grade: char = 'A'; // single quotes for characters
    // let num_1: f32 = 1.111111111111111;
    // println!("f32: {}", num_1 + 0.111111111111111); // 32 bits has 6 digits of precision
    // let num_2: f64 = 1.111111111111111;
    // println!("f64: {}", num_2 + 0.111111111111111); // 64 bits has 14 digits of precision

    // let num_3: u32 = 5;
    // let num_4: u32 = 4;
    // println!("5+4 = {}", num_3 + num_4);
    // println!("5-4 = {}", num_3 - num_4);
    // println!("5*4 = {}", num_3 * num_4);
    // println!("5/4 = {}", num_3 / num_4);
    // println!("5%4 = {}", num_3 % num_4);

    // Random Numbers
    // let random_num: i32 = rand::thread_rng().gen_range(1022516..103151511);
    // let random_num_2: u64 = rand::thread_rng().gen(); //same as writing "let random_num_2: u64 = rand::thread_rng().gen::<u64>();"
    //                                                   // let random_num_2: u64 = rand::thread_rng().gen::<u32>().into(); //randoming and typecasting
    // println!("Random number 32: {}", random_num); // (Range 1..101 means 1 to 100)
    // println!("Random number 64: {}", random_num_2);

    // let random_num_128: u128 = rand::thread_rng().gen();
    // println!("Random number 128: {}", random_num_128);

    //------------------
    // Conditionals

    // let age: i32 = 8;
    // let mut age: String = String::new();
    // println!("Enter your age");
    // io::stdin().read_line(&mut age).expect("Input failed");
    // // println!("Your age is {}", age);
    // let age: i32 = age.trim().parse().unwrap(); // trimming is important when taken input from stdin .trim()
    // if (age >= 1) && (age <= 18) {
    //     println!("Important Birthday {}", age);
    // } else if (age == 21) || (age == 50) {
    //     println!("Important Birthday {}", age);
    // } else if (age >= 65) {
    //     println!("Important Birthday {}", age);
    // } else {
    //     println!("Not an important Birthday {}", age);
    // }

    // leet code problem
    // let name: String = "my name is man2k".to_string();
    // let mut word: String = String::new();
    // let rname: String = name.chars().rfind().rev().collect::<String>();
    // rname.matches(pat)
    // for n in rname.chars() {
    //     word.push(n);
    //     if (n == ' ') {
    //         break;
    //     }
    // }
    // let name: String = word.chars().rev().collect::<String>();

    // println!("{}", name);
    // println!("Your name is {}", name.chars().rev().collect::<String>());

    // Ternary operator
    // let mut my_age = 47;
    // let can_vote: bool = if my_age >= 18 { true } else { false }; // equivalent to ternary
    // println!("Can vote: {}", can_vote);

    // let age2: i32 = 8;
    // match age2 {
    //     1..=18 => println!("Important Birthday"), // 1 to and equal to 18
    //     21 | 50 => println!("Important Birthday"), // 21 or 50
    //     65..=i32::MAX => println!("Important Birthday"), // 65 to i32 max
    //     _ => println!("Not Important Birthday"), // everything else
    // };

    // let my_age: i32 = 18;
    // const VOTING_AGE: i32 = 18;
    // match my_age.cmp(&VOTING_AGE) {
    //     Ordering::Less => println!("Can't vote"),
    //     Ordering::Greater => println!("Can vote"),
    //     Ordering::Equal => println!("You gained the right to vote"),
    // };

    // ARRAYS

    // let arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // declaring arrays
    // println!("1st : {}", arr_1[0]);
    // println!("Length : {}", arr_1.len());

    //Looping over arrays
    // let arr_2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut loop_idx: usize = 0;

    // loop method
    // loop {
    //     if arr_2[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_2[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("Val : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }

    // while loop
    // while loop_idx < arr_2.len() {
    //     println!("Array : {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }

    // for loop
    // for val in arr_2.iter() {
    //     println!("Val : {}", val);
    // }

    // TUPLE:  can store multiple data types
    // let my_tuple: (u8, String, f64) = (99, "Man2k".to_string(), 50_000.00);

    // println!("Name : {}", my_tuple.1); // ref to index
    // let (v1, v2, v3) = my_tuple; // like destructuring in js
    // println!("age : {}", v1);
}
