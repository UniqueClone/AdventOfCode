use std::cmp::max;
use std::fs;
use std::io;

pub fn day2(file_path: String) {
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
        let input = line.split(" ").collect::<Vec<&str>>();

        let game_id = String::from(input[1]);
        let game_id = game_id.replace(":", "");

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut num = 0;

        for word in input {
            if word.to_string().parse::<i32>().is_ok() {
                num = word.parse::<i32>().expect("Failed to parse number");
            } else {
                match word.replace(",", "").replace(";", "").as_str() {
                    "red" => {
                        red = max(num, red);
                        num = 0;
                    }
                    "green" => {
                        green = max(num, green);
                        num = 0;
                    }
                    "blue" => {
                        blue = max(num, blue);
                        num = 0;
                    }
                    _ => (),
                }
            }
        }

        if (red <= 12) && (green <= 13) && (blue <= 14) {
            total += game_id.parse::<i32>().expect("Failed to parse game id");
        }
    }
    println!("{}", total);
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.lines() {
        let input = line.split(" ").collect::<Vec<&str>>();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut num = 0;

        for word in input {
            if word.to_string().parse::<i32>().is_ok() {
                num = word.parse::<i32>().expect("Failed to parse number");
            } else {
                match word.replace(",", "").replace(";", "").as_str() {
                    "red" => {
                        red = max(num, red);
                        num = 0;
                    }
                    "green" => {
                        green = max(num, green);
                        num = 0;
                    }
                    "blue" => {
                        blue = max(num, blue);
                        num = 0;
                    }
                    _ => (),
                }
            }
        }

        sum += red * green * blue;
    }
    println!("{}", sum);
}
