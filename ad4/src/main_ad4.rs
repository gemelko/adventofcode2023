use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i32 = 0;
    let mut my_total_cards = 0;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad4input.txt") {
        let mut copy_count: Vec<i32> = vec![1; 211]; // 211 is the number of lines in the file
        for line in lines {
            my_count += 1;
            if my_count > 2000 {
                break;
            }
            let this_line = line.unwrap_or(String::new());
            if this_line != "" {
                let card_id = scan_fmt!(this_line.as_str(), "Card {}:", i32).unwrap();
                println!("{} {}", card_id, this_line);
                if let Some(card_contents) = this_line.split(":").nth(1) {
                    let card_number_sets: Vec<&str> = card_contents.split("|").collect();
                    let winning_numbers: Vec<&str> = card_number_sets[0].split_whitespace().collect();
                    let my_numbers: Vec<&str> = card_number_sets[1].split_whitespace().collect();
                    let mut card_value = 0;
                    let mut n_winners: i32 = 0;
                    for test_number in my_numbers {
                        if winning_numbers.contains(&test_number) {
                            n_winners += 1;
                            if card_value==0 {
                                card_value = 1;
                            } else {
                                card_value *= 2;
                            }
                        }
                    }
                    if n_winners > 0 {
                        for this_card_id in (card_id)..(card_id+n_winners) {
                            copy_count[this_card_id as usize] += copy_count[(card_id-1) as usize];
                        }
                    }
                    println!("Card value: {} n_winners {}", card_value, n_winners);
                    my_total += card_value;
                }
            }
        }
        for val in copy_count {
            print!("{} ", val);
            my_total_cards += val;
        }
        print!("\n");
    }
    println!("Total: {}", my_total);
    println!("Cards: {}", my_total_cards);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}
