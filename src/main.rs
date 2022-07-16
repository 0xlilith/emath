/*
project: emath
author: 0xlilith
*/
use emath::*;
use std::io;

fn main() {
    menu();
    let mut input = String::new();
    println!("Enter Choice: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read choice");   
    matching(&input)
}

fn matching(choice: &str) {
    match choice.trim() {
        "1" => newton_rapson(),
        _ => print!("Invalid Choice"),
    }
}