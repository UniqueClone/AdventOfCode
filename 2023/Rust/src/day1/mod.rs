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
    let mut total = 0;

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
        let number = pos1 + &pos2;
        let number: i32 = number.parse().unwrap();

        total += number;
    }

    println!("{}", total);
}

/**
 * Converts a secret number to a real number.
 * e.g. "one" -> "1"
 * e.g. "two" -> "2"
 */
fn convert_secret_number(string: String) -> String {
    match string {
        one => String::from("1"),
        two => String::from("2"),
        three => String::from("3"),
        four => String::from("4"),
        five => String::from("5"),
        six => String::from("6"),
        seven => String::from("7"),
        eight => String::from("8"),
        nine => String::from("9"),
        _ => String::from(""),
    }
}
