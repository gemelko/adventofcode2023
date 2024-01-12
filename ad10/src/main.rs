use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use scan_fmt::scan_fmt;

fn main() {
    let mut my_loop: Vec<(usize, usize)> = Vec::new();
    let mut next_loci: (usize, usize);
    let mut new_lines: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad10input2.txt") {
        let mut row: usize = 0;
        let mut col: usize = 0;
        for (iline, line) in lines.iter().enumerate() {
            if let Some(pos) = line.find('S') {
                row = iline;
                col = pos;
                println!("{:?}", line)
            }
            let another_line: Vec<char> = ".".repeat(line.len()).chars().collect();
            new_lines.push(another_line);
        }
        // println!("{} {}", row, col);
        // println!("{:?}", lines[row])
        next_loci = (row, col);
        let mut steps: i32 = 0;
        let mut stepped: bool = false;
        let mut go_up: bool = false;
        let mut go_down: bool = true;
        let mut go_left: bool = false;
        let mut go_right: bool = false;
        while !my_loop.contains(&next_loci) {
            stepped = false;
            new_lines[row][col] = lines[row].chars().nth(col).unwrap(); // Copy the loop over
            my_loop.push(next_loci);
            (row, col) = next_loci;
            steps += 1;
            if col < lines[row].len()-1 && go_right {
                let right_char = lines[row].chars().nth(col+1).unwrap();
                if right_char=='-' || right_char=='7' || right_char=='J' {
                    next_loci = (row, col+1);
                    stepped = true;
                    if right_char=='-' {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down = false;
                    } else if right_char=='7' {
                        go_down = true;
                        go_right = false;
                        go_up = false;
                        go_left=false;
                    } else {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;                            
                    }
                }
            }
            if col > 0 && !stepped && go_left {
                let left_char = lines[row].chars().nth(col-1).unwrap();
                if left_char=='-' || left_char=='F' || left_char=='L' {
                    next_loci = (row, col-1);
                    stepped = true;
                    if left_char == '-' {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    } else if left_char == 'F' {
                        go_down = true;
                        go_up = false;
                        go_right = false;
                        go_left = false;
                    } else {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;
                    }
                }
            }
            if row > 0 && !stepped && go_up {
                let up_char = lines[row-1].chars().nth(col).unwrap();
                if up_char=='|' || up_char=='7' || up_char=='F' {
                    next_loci = (row-1, col);
                    stepped = true;
                    if up_char == '|' {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;
                    } else if up_char == '7' {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    } else {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down = false;
                    }
                }
            }
            if row < lines.len()-1 && !stepped && go_down {
                let down_char = lines[row+1].chars().nth(col).unwrap();
                if down_char=='|' || down_char=='L' || down_char=='J' {
                    next_loci = (row+1, col);
                    stepped = true;
                    if down_char == '|' {
                        go_down = true;
                        go_up = false;
                        go_left = false;
                        go_right = false;
                    } else if down_char == 'L' {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down=false;
                    } else {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    }
                }
            }
            if my_loop.len()<2 {
                println!("{:?} {:?} {:?} {:?}", go_left, go_right, go_up, go_down);
            }
            if !stepped {
                println!("{:?} {:?}", stepped, next_loci);
                println!("{:?} {:?} {:?} {:?}", go_left, go_right, go_up, go_down);
                // println!("{:?}", my_loop.contains(&(65 as usize, 79 as usize)));
                // panic!("Failed to follow the loop : {:?}", next_loci);
                break;
            }
            // println!("{:?} {:?} {:?}", stepped, my_loop, next_loci);
        }
        println!("Loop steps: {}", steps);
        println!("Distance: {}", (steps+1)/2);
        let mut steps: i32 = 0;
        let mut stepped: bool = false;
        let mut go_up: bool = true;
        let mut go_down: bool = false;
        let mut go_left: bool = false;
        let mut go_right: bool = false;
        next_loci = (63, 79);
        my_loop.clear();
        while !my_loop.contains(&next_loci) {
            stepped = false;
            new_lines[row][col] = lines[row].chars().nth(col).unwrap(); // Copy the loop over
            my_loop.push(next_loci);
            (row, col) = next_loci;
            steps += 1;
            if col < lines[row].len()-1 && go_right {
                let right_char = lines[row].chars().nth(col+1).unwrap();
                if right_char=='-' || right_char=='7' || right_char=='J' {
                    next_loci = (row, col+1);
                    stepped = true;
                    if right_char=='-' {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down = false;
                    } else if right_char=='7' {
                        go_down = true;
                        go_right = false;
                        go_up = false;
                        go_left=false;
                    } else {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;                            
                    }
                }
            }
            if col > 0 && !stepped && go_left {
                let left_char = lines[row].chars().nth(col-1).unwrap();
                if left_char=='-' || left_char=='F' || left_char=='L' {
                    next_loci = (row, col-1);
                    stepped = true;
                    if left_char == '-' {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    } else if left_char == 'F' {
                        go_down = true;
                        go_up = false;
                        go_right = false;
                        go_left = false;
                    } else {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;
                    }
                }
            }
            if row > 0 && !stepped && go_up {
                let up_char = lines[row-1].chars().nth(col).unwrap();
                if up_char=='|' || up_char=='7' || up_char=='F' {
                    next_loci = (row-1, col);
                    stepped = true;
                    if up_char == '|' {
                        go_up = true;
                        go_down = false;
                        go_left = false;
                        go_right = false;
                    } else if up_char == '7' {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    } else {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down = false;
                    }
                }
            }
            if row < lines.len()-1 && !stepped && go_down {
                let down_char = lines[row+1].chars().nth(col).unwrap();
                if down_char=='|' || down_char=='L' || down_char=='J' {
                    next_loci = (row+1, col);
                    stepped = true;
                    if down_char == '|' {
                        go_down = true;
                        go_up = false;
                        go_left = false;
                        go_right = false;
                    } else if down_char == 'L' {
                        go_right = true;
                        go_left = false;
                        go_up = false;
                        go_down=false;
                    } else {
                        go_left = true;
                        go_right = false;
                        go_up = false;
                        go_down = false;
                    }
                }
            }
            if my_loop.len()<2 {
                println!("{:?} {:?} {:?} {:?}", go_left, go_right, go_up, go_down);
            }
            if !stepped {
                println!("{:?} {:?}", stepped, next_loci);
                println!("{:?} {:?} {:?} {:?}", go_left, go_right, go_up, go_down);
                // println!("{:?}", my_loop.contains(&(65 as usize, 79 as usize)));
                // panic!("Failed to follow the loop : {:?}", next_loci);
                break;
            }
            // println!("{:?} {:?} {:?}", stepped, my_loop, next_loci);
        }
        println!("Loop steps: {}", steps);
        println!("Distance: {}", (steps+1)/2);
        let mut inside_total: i32 = 0;
        let mut inside_flag: bool = false;
        let mut ch_flag: bool = false;
        let mut my_count: i32 = 0;
        for (this_row, this_line) in new_lines.iter().enumerate() {
            inside_flag = false;
            ch_flag = false;
            let mut last_c: char = '.';
            for this_c in this_line {
                if *this_c=='F' || *this_c=='|' || *this_c=='S' || *this_c=='7' || *this_c=='L' || *this_c=='J' {
                    if last_c=='F' && *this_c=='J' {
                        inside_flag = !inside_flag;
                        last_c=' ';
                    } else if last_c=='L' && *this_c=='7' {
                        inside_flag = !inside_flag;
                        last_c=' ';
                    } else if *this_c=='|' || *this_c=='S'{
                        inside_flag = !inside_flag;
                        last_c=' ';
                    } else if *this_c=='F' || *this_c=='L' {
                        last_c = *this_c;
                    }
                } else if *this_c=='.' {
                    if inside_flag {
                        inside_total += 1;
                        ch_flag = true;
                    }
                    last_c = ' ';
                }
            }
            if ch_flag {
                // println!("{} {:?}", this_row, this_line);
                // println!("this_row: {} inside_total: {}", this_row, inside_total);
                my_count += 1;
                if my_count>2 {
                    // break;
                }
            }
            // let this_str: String = this_line.into_iter().collect();
            // println!("{} {:?}", this_row, this_str);
        }
        println!("inside_total: {}", inside_total);
    }
}


fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let my_file = File::open(filename)?;
    let reader = BufReader::new(my_file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}
