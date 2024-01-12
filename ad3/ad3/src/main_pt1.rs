use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use scan_fmt::scan_fmt;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i32 = 0;
    let mut l_prev: String = String::from("");
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad3\\ad3\\ad3input.txt") {
        let mut pa_lines = lines.peekable();
        while let Some(this_line) = pa_lines.next() {
            my_count += 1;
            if my_count > 55000 {
                break;
            }
            let l_curr = this_line.as_ref().unwrap();
            if l_prev == "" {
                l_prev = this_line.as_ref().unwrap().clone();
            }
            let l_next;
            if let Some(next_line) = pa_lines.peek() {
                l_next = next_line.as_ref().unwrap();
            }
            else {
                l_next = this_line.as_ref().unwrap();
            }

//            println!("{}\n{}\n{}\n", l_prev, l_curr, l_next);
            println!("{} {}", my_count, l_curr);
            let values = get_values(&l_prev, l_curr, l_next);
            for value in values {
                my_total += value;
//                println!("{}", value)
            }
            l_prev = this_line.as_ref().unwrap().clone();
        }
    }
    println!("Total: {}", my_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}

fn get_values(a0: &String, a1: &String, a2: &String) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current_number = String::new();

    let ac0 = a0.chars();
    let ac1 = a1.chars();
    let ac2 = a2.chars();

    let mut valid: bool = false;
    let mut last_valid: bool = false;
    let mut num_valid = false;

    for (c0, (c1, c2)) in ac0.zip(ac1.zip(ac2)) {
        if (!c0.is_numeric() && c0!='.') ||
           (!c1.is_numeric() && c1!='.') ||
           (!c2.is_numeric() && c2!='.') {
            valid = true;
           }
           else {
            valid = false;
           }
        if c1.is_numeric() {
            if current_number.is_empty() {
                num_valid = false;
            }
            num_valid = num_valid | last_valid | valid;
            current_number.push(c1);
        } else if !current_number.is_empty() {
            num_valid = num_valid | valid;
            if let Ok(number) = current_number.parse::<i32>() {
                if num_valid {
                    result.push(number);
                }
            }
            current_number.clear();
        }
        last_valid = valid;
    }

    if !current_number.is_empty() {
        if let Ok(number) = current_number.parse::<i32>() {
            if num_valid {
                result.push(number);
            }
        }
    }

    result
}
