mod day1;
use day1::day1;
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
        _ => println!("No such day"),
    }
}
