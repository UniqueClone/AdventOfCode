extern crate regex;

mod day1;
mod day2;
mod day3;
use day1::day1;
use day2::day2;
use day3::day3;
use std::io;

fn main() {
    println!("Please enter a day:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse::<i32>().unwrap();

    let file_path_prefix =
        "C:/Users/ryanlynch/Documents/repos/personal/AdventOfCode/2024/Rust/src/";

    println!();

    match input {
        // 1 => day1(file_path_prefix.to_string() + "day1/sample.txt"),
        1 => day1(file_path_prefix.to_string() + "day1/input.txt"),
        2 => day2(file_path_prefix.to_string() + "day2/input.txt"),
        3 => day3(file_path_prefix.to_string() + "day3/input.txt"),
        _ => println!("No such day"),
    }
}
