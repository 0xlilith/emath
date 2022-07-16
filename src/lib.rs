/*
emath library
author: 0xlilith
*/
use std::f64;

pub fn menu() {
    println!("1. Newton-Raphson method");
}


fn f(x: f64) -> f64 {
    x.powi(3) - 2.0 * x - 5.0
}
fn fd(x: f64) -> f64 {
    3.0 * x.powi(2) - 2.0
}
fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    let mut result = guess;
    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
    result
}
fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
    guess - f(guess) / fd(guess)
}
pub fn newton_rapson() {
    print!("Example: x^3 - 2x - 5");
    assert_eq!(0.8654740331016144, find_root(f, fd, 0.5, 3));
}