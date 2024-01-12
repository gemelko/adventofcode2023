use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use scan_fmt::scan_fmt;

fn main() {
    let mut count: i32 = 0;
    let mut my_total: i64 = 0;
    let mut my_hist_total: i64 = 0;
    if let Ok(lines_raw) = read_lines("C:\\rust\\DevelopmentFolder\\ad9input.txt") {
        for line in lines_raw {
            let this_line = line.unwrap();
            let top_vec = this_line.trim().split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let mut vec_deltas: Vec<Vec<i64>> = Vec::new();
            let mut this_delta: Vec<i64> = Vec::new();
            println!("{:?}", top_vec);
            this_delta = top_vec.clone();
            vec_deltas.push(this_delta.clone());
            while this_delta.iter().any(|x| *x!=(0 as i64)) {
                let mut next_delta: Vec<i64> = Vec::new();
                for i in 0..(this_delta.len()-1) {
                    next_delta.push(this_delta[i+1]-this_delta[i]);
                }
                vec_deltas.push(next_delta.clone());
                this_delta = next_delta.clone();
            }
            println!("steps: {}", top_vec.len()-this_delta.len());
            println!("{:?}", vec_deltas);
            let mut new_row: Vec<i64> = Vec::new();
            let mut new_hrow: Vec<i64> = Vec::new();
            let mut prev_val: i64 = 0;
            let mut prev_hval: i64 = 0;
            for i in (0..(vec_deltas.len())).rev() {
                let next_val = prev_val + vec_deltas[i][vec_deltas[i].len()-1];
                new_row.push(next_val);
                prev_val = next_val;
                let next_val = vec_deltas[i][0] - prev_hval;
                new_hrow.push(next_val);
                prev_hval = next_val;
            }
            println!("{:?} {}", new_row, new_row[new_row.len()-1]);
            println!("{:?} {}", new_hrow, new_hrow[new_hrow.len()-1]);
            my_total += new_row[new_row.len()-1];
            my_hist_total += new_hrow[new_hrow.len()-1];
            count += 1;
            if count>2 {
                //break;
            }
        }
    }
    println!("Total: {}", my_total);
    println!("Hist Total: {}", my_hist_total);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}
