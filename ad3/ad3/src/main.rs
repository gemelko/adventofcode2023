use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// use scan_fmt::scan_fmt;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i32 = 0;
    let mut my_gear_total: i32 = 0;
    let mut l_prev: String = String::from("");
    let mut empty_prev: bool = false;
    let mut empty_next: bool = false;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad3\\ad3\\ad3input.txt") {
        let mut pa_lines = lines.peekable();
        while let Some(this_line) = pa_lines.next() {
            my_count += 1;
            if my_count > 9000 {
                break;
            }
            let l_curr = this_line.as_ref().unwrap();
            if l_prev == "" {
                l_prev = this_line.as_ref().unwrap().clone();
                empty_prev = true;
            } else {
                empty_prev = false;
            }
            let l_next;
            if let Some(next_line) = pa_lines.peek() {
                l_next = next_line.as_ref().unwrap();
                empty_next = false;
            } else {
                l_next = this_line.as_ref().unwrap();
                empty_next = true;
            }

//            println!("\n::{}\n{}\n{}\n{}\n", my_count, l_prev, l_curr, l_next);
            print!("\n{}", l_curr);
            let values = get_values(&l_prev, l_curr, l_next);
            let gears = get_gears(empty_prev, &l_prev, l_curr, empty_next, l_next);
            for value in values {
                my_total += value;
//                println!("{}", value);
            }
            for gear in gears {
//                println!("{}", gear);
                my_gear_total += gear;
            }
            l_prev = this_line.as_ref().unwrap().clone();
        }
    }
    println!("Total: {}", my_total);
    println!("Gear Total: {}", my_gear_total);
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
        if (!c0.is_digit(10) && c0!='.') ||
           (!c1.is_digit(10) && c1!='.') ||
           (!c2.is_digit(10) && c2!='.') {
            valid = true;
           }
           else {
            valid = false;
           }
        if c1.is_digit(10) {
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

fn get_gears(s0: bool, a0: &String, a1: &String, s2: bool, a2: &String) -> Vec<i32> {
    let mut result = Vec::new();
    let a0c: Vec<char> = a0.chars().collect();
    let a1c: Vec<char> = a1.chars().collect();
    let a2c: Vec<char> = a2.chars().collect();
    let mut v1: i32 = 0;
    let mut v2: i32 = 0;
    // println!("u0: {} u2: {}", !s0, !s2);
    //print!("\n");
    for (c1i, c1) in a1c.clone().into_iter().enumerate() {
        if c1 == '*' {
            let mut n_count: i32 = 0;
            if c1i > 0 {
                if a1c[(c1i-1) as usize].is_digit(10) {
                    n_count += 1;
                }
                let mut cp = c1i as usize;
                while cp > 0 {
                    if a1c[cp-1].is_digit(10) {
                        cp -= 1;
                    } else {
                        break;
                    }
                }
                let mut current_number = String::new();
                while a1c[cp].is_digit(10) {
                    current_number.push(a1c[cp]);
                    cp += 1;
                }
                if let Ok(number) = current_number.parse::<i32>() {
//                    print!("A: {}  ", current_number);
                    v1 = number;
                }
            }
            if c1i < a1.len() - 1 {
                if a1c[(c1i+1) as usize].is_digit(10) {
                    n_count += 1;
                }
                let mut cp = c1i+1;
                let mut current_number = String::new();
                while a1c[cp].is_digit(10) {
                    current_number.push(a1c[cp]);
                    cp += 1;
                    if cp==a1c.len() {
                        break;
                    }
                }
                if let Ok(number) = current_number.parse::<i32>() {
//                    print!("B: {}  ", current_number);
                    if v1==0 {
                        v1 = number;
                    } else {
                        v2 = number;
                    }
                }
            }
            if !s0 {
                if a0c[c1i as usize].is_digit(10) {
                    n_count += 1;
                    let mut cp = c1i as usize;
                    while cp > 0 {
                        if a0c[cp-1].is_digit(10) {
                            cp -= 1;
                        } else {
                            break;
                        }
                    }
                    let mut current_number = String::new();
                    while a0c[cp].is_digit(10) {
                        current_number.push(a0c[cp]);
                        cp += 1;
                        if cp==a1c.len() {
                            break;
                        }
                    }
                    if let Ok(number) = current_number.parse::<i32>() {
//                        print!("C: {}  ", current_number);
                        if v1==0 {
                            v1 = number;
                        } else {
                            v2 = number;
                        }
                    }
                } else {
                    if c1i > 0 {
                        if a0c[(c1i-1) as usize].is_digit(10) {
                            n_count += 1;
                        }
                        let mut cp = c1i as usize;
                        while cp > 0 {
                            if a0c[cp-1].is_digit(10) {
                                cp -= 1;
                            } else {
                                break;
                            }
                        }
                        let mut current_number = String::new();
                        while a0c[cp].is_digit(10) {
                            current_number.push(a0c[cp]);
                            cp += 1;
                        }
                        if let Ok(number) = current_number.parse::<i32>() {
//                            print!("D: {}  ", current_number);
                            if v1==0 {
                                v1 = number;
                            } else {
                                v2 = number;
                            }
                        }
                    }
                    if c1i < a1.len() - 1 {
                        if a0c[(c1i+1) as usize].is_digit(10) {
                            n_count += 1;
                        }
                        let mut cp = (c1i+1) as usize;
                        let mut current_number = String::new();
                        while a0c[cp].is_digit(10) {
                            current_number.push(a0c[cp]);
                            cp += 1;
                            if cp==a1c.len() {
                                break;
                            }
                        }
                        if let Ok(number) = current_number.parse::<i32>() {
//                            print!("E: {}  ", current_number);
                            if v1==0 {
                                v1 = number;
                            } else {
                                v2 = number;
                            }
                        }
                    }
                }
            }
            if !s2 {
                if a2c[c1i as usize].is_digit(10) {
                    n_count += 1;
                    let mut cp = c1i as usize;
                    while cp > 0 {
                        if a2c[cp-1].is_digit(10) {
                            cp -= 1;
                        } else {
                            break;
                        }
                    }
                    let mut current_number = String::new();
                    while a2c[cp].is_digit(10) {
                        current_number.push(a2c[cp]);
                        cp += 1;
                        if cp==a1c.len() {
                            break;
                        }
                    }
                    if let Ok(number) = current_number.parse::<i32>() {
//                        print!("F: {}  ", current_number);
                        if v1==0 {
                            v1 = number;
                        } else {
                            v2 = number;
                        }
                    }
                } else {
                    if c1i>0 {
                        if a2c[(c1i-1) as usize].is_digit(10) {
                            n_count += 1;
                        }
                        let mut cp = c1i as usize;
                        while cp > 0 {
                            if a2c[cp-1].is_digit(10) {
                                cp -= 1;
                            } else {
                                break;
                            }
                        }
                        let mut current_number = String::new();
                        while a2c[cp].is_digit(10) {
                            current_number.push(a2c[cp]);
                            cp += 1;
                        }
                        if let Ok(number) = current_number.parse::<i32>() {
//                            print!("G: {}  ", current_number);
                            if v1==0 {
                                v1 = number;
                            } else {
                                v2 = number;
                            }
                        }
                    }
                    if c1i < a1.len() - 1 {
                        if a2c[(c1i+1) as usize].is_digit(10) {
                            n_count += 1;
                        }
                        let mut cp = (c1i+1) as usize;
                        let mut current_number = String::new();
                        while a2c[cp].is_digit(10) {
                            current_number.push(a2c[cp]);
                            cp += 1;
                            if cp==a1c.len() {
                                break;
                            }
                        }
                        if let Ok(number) = current_number.parse::<i32>() {
//                            print!("H: {}  ", current_number);
                            if v1==0 {
                                v1 = number;
                            } else {
                                v2 = number;
                            }
                        }
                    }
                }
            }
            if n_count==2 {
                print!(":{} {} {} {}", c1i, n_count, v1, v2);
                result.push(v1*v2);
            }
            v1=0;
            v2=0;
        }
    }

    result
}
