use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i32 = 0;
    if let Ok(lines) = read_lines("C:\\rust\\ad1\\hello_world\\ad1input.txt") {
        for line in lines {
            my_count += 1;
            let step_one: &str = &line.unwrap();
            println!("{} {}", my_count, step_one);
            let for_parser = replace_number_words(step_one);
            println!("    {}", for_parser);
            println!("    {} {}", first_number(&for_parser), last_number(&for_parser));
            let v1 = first_number(&for_parser);
            let v2 = last_number(&for_parser);
            let vn = v1*10+v2;
            my_total += vn;
            if my_count > 4000 {
                panic!("Stopping");
            }
        }
    }
    println!("{}", my_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}

fn first_number(input: &str) -> i32 {
    let parser = Regex::new(r"\d").unwrap();

    if let Some(digits) = parser.find(input) {
        Some(digits.as_str()).expect("Empty string").parse().expect("First failed to find an integer")
    } else {
        0
    }
}

fn last_number(input: &str) -> i32 {
    let parser = Regex::new(r"\d").unwrap();

    let revstr: String = input.chars().rev().collect();
    if let Some(digits) = parser.find(&revstr) {
        Some(digits.as_str()).expect("Empty string").parse().expect("Last failed to find an integer")
    } else {
        0
    }
}

fn replace_number_words(input: &str) -> String {
    let replace_table = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("ten", "10"),
    ];

    let result_str = String::from(input);
    let mut output_str: String;

    output_str = String::from("");

    for (i, charac) in result_str.char_indices() {
        if charac.is_numeric() {
            output_str = output_str + &charac.to_string();
        }
        else {
            for (word, replacement) in replace_table {
                if result_str[i..].starts_with(word) {
                    output_str = output_str + replacement;
                }
            }
        }
    }
    output_str
}