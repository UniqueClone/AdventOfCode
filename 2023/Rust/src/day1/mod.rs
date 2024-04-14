use std::fs;
use std::io;

pub fn day1(file_path: String) {
    println!("Please enter a part:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse::<i32>().unwrap();

    match input {
        1 => part1(file_path),
        2 => part2(file_path),
        _ => println!("No such part"),
    }
}

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total = 0;

    for line in contents.lines() {
        let mut pos1 = String::from("");
        let mut pos2 = String::from("");

        for char in line.chars() {
            if char.is_numeric() {
                if pos1 == "" {
                    pos1 = char.to_string();
                    pos2 = char.to_string();
                } else {
                    pos2 = char.to_string();
                }
            }
        }

        let number = pos1 + &pos2;
        let number: i32 = number.parse().unwrap();

        total += number;
    }

    println!("{}", total);
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines() {
        let mut pos1 = String::from("");
        let mut pos2 = String::from("");

        // TODO - add in ability to detect the word "one" and call convert_secret_number on it.
        for char in line.chars() {
            if char.is_numeric() {
                if pos1 == "" {
                    pos1 = char.to_string();
                    pos2 = char.to_string();
                } else {
                    pos2 = char.to_string();
                }
            }
        }
    }
}

/**
 * Converts a secret number to a real number.
 * e.g. "one" -> "1"
 * e.g. "two" -> "2"
 */
fn convert_secret_number(string: String) -> String {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";
    let six = "six";
    let seven = "seven";
    let eight = "eight";
    let nine = "nine";

    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "0",
    }
}
