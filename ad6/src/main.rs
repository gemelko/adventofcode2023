// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
// use scan_fmt::scan_fmt;

// Time:        60     80     86     76
// Distance:   601   1163   1559   1300

fn main() {
    let time: Vec<i64> = vec![60, 80, 86, 76, 60808676];
    let dist: Vec<i64> = vec![601, 1163, 1559, 1300, 601116315591300];
    let mut win_combos: Vec<i64> = vec![0, 0, 0, 0, 0];
    let mut win_prod: i64 = 1;
    for trial in 0..5 {
        for hold in 1..time[trial] {
            let calc_dist = (time[trial]-hold)*hold;
            if calc_dist > dist[trial] {
                win_combos[trial] += 1;
            }
        }
        println!("Race {} Wins {}", trial+1, win_combos[trial]);
        win_prod *= win_combos[trial];
    }
    println!("Prod of wins: {}", win_prod)
}
