use regex::Regex;
use std::fs;
use std::io;

pub fn day3(file_path: String) {
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
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let re2 = Regex::new(r"[0-9]+").unwrap();

    for line in contents.lines() {
        let captures_iter = re.captures_iter(line);

        for cap in captures_iter {
            // println!("{:?}", cap.get(0).unwrap().as_str());

            let captures_iter2 = re2.captures_iter(cap.get(0).unwrap().as_str());

            let mut product = 1;
            for cap2 in captures_iter2 {
                product *= cap2.get(0).unwrap().as_str().parse::<i32>().unwrap();
            }

            total += product;
        }
    }
    println!("Total: {}", total);
}

fn part2(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total: i64 = 0;
    let mut should_multiply = true;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let results = re.captures_iter(&contents);

    for cap in results {
        if cap.get(0).unwrap().as_str() == "do()" {
            should_multiply = true;
        } else if cap.get(0).unwrap().as_str() == "don't()" {
            should_multiply = false;
        } else {
            if should_multiply {
                total += cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                    * cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
            }
        }
    }

    println!("Total: {}", total);
}
