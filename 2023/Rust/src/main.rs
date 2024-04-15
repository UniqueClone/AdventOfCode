mod day1;
mod day2;
use day1::day1;
use day2::day2;
use std::io;

fn main() {
    println!("Please enter a day:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse::<i32>().unwrap();

    let file_path_prefix = "/Users/ryanlynch/repos/personal/AdventOfCode/2023/Rust/src/";

    println!();

    match input {
        1 => day1(file_path_prefix.to_string() + "day1/input.txt"),
        2 => day2(file_path_prefix.to_string() + "day2/input.txt"),
        _ => println!("No such day"),
    }
}
