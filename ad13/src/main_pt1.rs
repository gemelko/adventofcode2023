use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
// use std::cmp;

fn main() {
    let mut my_count: i32 = 0;
    // let mut my_total: i32 = 0;
    let mut this_group: Vec<String> = Vec::new();
    let mut group_count: i32 = 0;
    let mut v_total_val: i32 = 0;
    let mut h_total_val: i32 = 0;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad13input.txt") {
        println!("{} lines", lines.len());
        for line in lines {
            if line.trim().len()==0 {
                // Vertical reflection check
                // println!("AAA {}", group_count);
                let mut cur_line: i32;
                let mut matching: bool;
                let mut v_reflect_val: i32;
                cur_line = group_count-2;
                while cur_line >= 0 {
                    matching = true;
                    for next_line in cur_line+1..group_count as i32 {
                        if cur_line*2+1-next_line >= 0 {
                            if this_group[next_line as usize] != this_group[(2*cur_line+1-next_line) as usize] {
                                matching = false;
                                break;
                            }
                        }
                    }
                    if matching && cur_line>=0 {
                        v_reflect_val = cur_line+1;
                        v_total_val += v_reflect_val;
                    }
                    cur_line -= 1;
                }

                // Handle reflections within a line
                let mut h_reflect_val: i32;
                let this_len: usize = this_group[0].len();
                // println!("Len: {}", this_len);
                for cpos in (1..this_len).rev() {
                    let mut matching1: bool = true;
                    let start1: usize;
                    let len1: usize;
                    if cpos <= this_len / 2 {
                        start1 = 0;
                        len1 = cpos;
                    } else {
                        start1 = cpos*2-this_len;
                        len1 = this_len-cpos;
                    }
                    // println!("TL {} S1 {} L1 {}", cpos, start1, len1);
                    // println!("TL {} S2 {} L2 {}", cpos, start2, len2);
                    for this_line in this_group.clone() {
                        if matching1 && len1>0 {
                            let left: String = (&this_line[start1..start1+len1]).to_string();
                            let right: String = this_line[start1+len1..start1+2*len1].chars().rev().collect();
                            if left != right {
                                matching1 = false;
                            }
                        } else {
                            matching1 = false;
                        }
                    }
                    if matching1 {
                        // for tline in &this_group {
                        //     println!("{}", tline);
                        // }
                        h_reflect_val = (start1+len1) as i32;
                        h_total_val += h_reflect_val;
                        // if my_count==67 {
                        //     println!("Ref1 {} {}", cpos, my_count);
                        // }
                    }
                }
                this_group.clear();
                group_count = 0;
                my_count += 1;
            } else {
                group_count += 1;
                this_group.push(line);
            }
            // if my_count>67 {
            //     break;
            // }
        }
        println!("Groups: {}", my_count);
        println!("VTotal: {}", v_total_val);
        println!("HTotal: {}", h_total_val);
        println!("Score: {}", v_total_val*100+h_total_val);
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
