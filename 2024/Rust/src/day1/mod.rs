use std::collections::HashMap;
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

/**
 * Part 1
 * Sample Input:
 * 3   4
 * 4   3
 * 2   5
 * 1   3
 * 3   9
 * 3   3
 */
fn part1(file_path: String) {
    let mut group1: Vec<i32> = Vec::new();
    let mut group2: Vec<i32> = Vec::new();

    // Read the file
    let contents = fs::read_to_string(file_path.clone())
        .expect(&("Should have been able to read the file ".to_string() + &file_path));

    // Parse the file, storing the first number in group1 and the second number in group2
    for line in contents.lines() {
        group1.push(
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        );
        group2.push(
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        );
    }

    // Sort the groups from least to greatest
    group1.sort();
    group2.sort();

    let mut distance = 0;

    // For each number in group1, add the absolute difference between it and the corresponding number in group2
    for i in 0..group1.len() {
        distance += (group2[i] - group1[i]).abs();
    }

    println!("{}", distance);
}

/**
 * Part 2
 */
fn part2(file_path: String) {
    let mut group1: Vec<i32> = Vec::new();
    let mut occurrences_in_group2: HashMap<i32, i32> = HashMap::new();

    // Read the file
    let contents = fs::read_to_string(file_path.clone())
        .expect(&("Should have been able to read the file ".to_string() + &file_path));

    // Parse the file
    // Store the first number in group1, and the number of occurrences of the numbers
    // in group2 in a hashmap
    for line in contents.lines() {
        group1.push(
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        );

        let number = line
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        if occurrences_in_group2.contains_key(&number) {
            *occurrences_in_group2.get_mut(&number).unwrap() += 1;
        } else {
            occurrences_in_group2.insert(number, 1);
        }
    }

    let mut simularity = 0;

    // For each number in group1, if it is in group2, multiply the number by the number of occurrences
    // in group2 and add it to the simularity
    for number in group1 {
        if occurrences_in_group2.contains_key(&number) {
            simularity += number * occurrences_in_group2.get(&number).unwrap();
        }
    }

    println!("{}", simularity);
}
