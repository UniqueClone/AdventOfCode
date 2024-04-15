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
        let number = parse_from_left_to_right(line.to_string())
            + &parse_from_right_to_left(line.to_string());

        println!("{}, {}", line, number);
        let number: i32 = number.parse::<i32>().expect("Failed to parse number");

        total += number;
    }

    println!("{}", total);
}

fn parse_from_left_to_right(string: String) -> String {
    if string.len() < 1 {
        return "".to_string();
    } else if string.chars().next().unwrap().is_numeric() {
        return string.chars().next().unwrap().to_string();
    } else if string.len() >= 3 && convert_secret_number(string[..3].to_string()) != "" {
        return convert_secret_number(string[..3].to_string());
    } else if string.len() >= 4 && convert_secret_number(string[..4].to_string()) != "" {
        return convert_secret_number(string[..4].to_string());
    } else if string.len() >= 5 && convert_secret_number(string[..5].to_string()) != "" {
        return convert_secret_number(string[..5].to_string());
    } else {
        return parse_from_left_to_right(string[1..].to_string());
    }
}

fn parse_from_right_to_left(string: String) -> String {
    if string.len() < 1 {
        return "".to_string();
    } else if string.chars().next_back().unwrap().is_numeric() {
        return string.chars().next_back().unwrap().to_string();
    } else if string.len() >= 3
        && convert_secret_number(string[string.len() - 3..].to_string()) != ""
    {
        return convert_secret_number(string[string.len() - 3..].to_string());
    } else if string.len() >= 4
        && convert_secret_number(string[string.len() - 4..].to_string()) != ""
    {
        return convert_secret_number(string[string.len() - 4..].to_string());
    } else if string.len() >= 5
        && convert_secret_number(string[string.len() - 5..].to_string()) != ""
    {
        return convert_secret_number(string[string.len() - 5..].to_string());
    } else {
        return parse_from_right_to_left(string[..string.len() - 1].to_string());
    }
}

#[test]
fn test_parse_from() {
    let answer = parse_from_left_to_right(String::from("abconetwo23fourdef"))
        + &parse_from_right_to_left(String::from("abconetwo23fourdef"));
    assert_eq!(answer, String::from("14"));

    let answer = parse_from_left_to_right(String::from("zdcrnpddv2"))
        + &parse_from_right_to_left(String::from("zdcrnpddv2"));
    assert_eq!(answer, String::from("22"));
}

#[test]
fn test_parse_from_right_to_left() {
    let answer = parse_from_right_to_left(String::from("abconetwo23fourdef"));
    assert_eq!(answer, String::from("4"));
}

/**
 * Converts a secret number to a real number.
 * e.g. "one" -> "1"
 * e.g. "two" -> "2"
 */
fn convert_secret_number(string: String) -> String {
    match string.as_str() {
        "one" => String::from("1"),
        "two" => String::from("2"),
        "three" => String::from("3"),
        "four" => String::from("4"),
        "five" => String::from("5"),
        "six" => String::from("6"),
        "seven" => String::from("7"),
        "eight" => String::from("8"),
        "nine" => String::from("9"),
        _ => String::from(""),
    }
}

#[test]
fn test_convert_secret_number() {
    let answer = convert_secret_number(String::from("one"));
    assert_eq!(answer, String::from("1"));

    let answer = convert_secret_number(String::from("two"));
    assert_eq!(answer, String::from("2"));

    let answer = convert_secret_number(String::from("three"));
    assert_eq!(answer, String::from("3"));
}
