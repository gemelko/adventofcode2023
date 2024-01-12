use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i32 = 0;
    let mut power_total: i32 = 0;
    let red_limit: i32 = 12;
    let green_limit: i32 = 13;
    let blue_limit: i32 = 14;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad2\\ad2\\ad2input.txt") {
        for line in lines {
            my_count += 1;
            if my_count > 5000 {
                break;
            }
            let step_one: String = line.unwrap();
            let step_two: Vec<&str> = step_one.split(":").collect();
            let mut possible: bool = true;
            println!("{}: {}", my_count, step_two[0]);
            let game_id = scan_fmt!(step_two[0], "Game {}", i32).unwrap();
            println!("... {} {}",game_id, step_two[1]);
            let mut max_red: i32 = 0;
            let mut max_green: i32 = 0;
            let mut max_blue: i32 = 0;
            for trial in step_two[1].split(';') {
                let mut this_red: i32 = 0;
                let mut this_green: i32 = 0;
                let mut this_blue: i32 = 0;
                for this_pick in trial.split(',') {
                    let this_pick_lc = this_pick.to_lowercase();
                    if this_pick_lc.contains("red") {
                        this_red += scan_fmt!(this_pick, "{}", i32).unwrap();
                        if this_red > max_red {
                            max_red = this_red;
                        }
                    }
                    if this_pick_lc.contains("green") {
                        this_green += scan_fmt!(this_pick, "{}", i32).unwrap();
                        if this_green > max_green {
                            max_green = this_green;
                        }
                    }
                    if this_pick_lc.contains("blue") {
                        this_blue += scan_fmt!(this_pick, "{}", i32).unwrap();
                        if this_blue > max_blue {
                            max_blue = this_blue;
                        }
                    }
                }
                possible = possible && (
                    this_red <= red_limit &&
                    this_green <= green_limit && 
                    this_blue <= blue_limit
                );
            }
            if possible {
                my_total += game_id;
                // println!("Possible: {} {} {}", this_red, this_green, this_blue)
            }
            println!("Power {}", max_red*max_green*max_blue);
            power_total += max_red*max_green*max_blue;
        }
    }
    println!("Total: {} Power total: {}", my_total, power_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}
