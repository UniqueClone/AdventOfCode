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
        // 2 => part2(file_path),
        _ => println!("No such part"),
    }
}

fn part1(file_path: String) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut total_safe = 0;
    let mut report_safe = true;

    for line in contents.lines() {
        report_safe = true;
        let mut prev: i32 = 0;
        let mut increasing: bool = false;

        // iterate over the numbers in the line
        for (i, num) in line.split_whitespace().enumerate() {
            // let cur_char = char.to_string();

            let current = match num.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Failed to parse number: {}", num);
                    report_safe = false;
                    break;
                }
            };

            if i == 0 {
                prev = current;
                continue;
            } else if current == prev {
                println!();
                report_safe = false;
                prev = current;
                break;
            } else if i == 1 {
                if (current - prev).abs() > 3 {
                    println!();
                    report_safe = false;
                    prev = current;
                    break;
                }
                increasing = current > prev;
                prev = current;
                continue;
            } else {
                if (current - prev).abs() > 3 {
                    println!();
                    report_safe = false;
                    prev = current;
                    break;
                } else if increasing && current < prev {
                    println!();
                    report_safe = false;
                    prev = current;
                    break;
                } else if !increasing && current > prev {
                    println!();
                    report_safe = false;
                    prev = current;
                    break;
                }
                prev = current;
            }

            if i == line.len() - 1 {
                println!();
            }
        }
        println!("Report {} safe: {}", line, report_safe);
        if report_safe {
            total_safe += 1;
        }
    }
    println!("{}", total_safe);
}
