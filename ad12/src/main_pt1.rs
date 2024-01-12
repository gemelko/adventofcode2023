use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use scan_fmt::scan_fmt;

fn main() {
    let mut my_total: i32 = 0;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad12inputx.txt") {
        for line in lines {
            if let Some((s1_val, t1_val)) = line.split_once(' ') {
                let s_val = (0..1).map(|_| s1_val).collect::<Vec<&str>>().join("?");
                let t_val = (0..1).map(|_| t1_val).collect::<Vec<&str>>().join(",");
                let max_grps = (t_val.chars().filter(|c| *c==',').count()+1) as i32;
                let combos = generate_combinations(&s_val, max_grps);
                let max_grps = (t_val.chars().filter(|c| *c==',').count()+1) as i32;
                // println!("Combos: {}", combos.len());
                for combo in combos {
                    println!("{}", combo);
                    // let this_grps = (combo.clone().chars().filter(|c| *c==',').count()+1) as i32;
                    let te_val = combo_eval(combo.clone());
                    if te_val == *t_val {
                        my_total += 1;
                        // println!("True");
                    } else {
                        // println!("False");
                    }
                }
                println!("{}", my_total);
            }
        }
    }
    println!("Total: {}", my_total);
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

fn generate_combinations(s: &str, max_grps: i32) -> Vec<String> {
    let mut combinations = Vec::new();
    generate_combinations_helper(s, &mut combinations, max_grps, String::new());
    combinations
}

fn generate_combinations_helper(s: &str, combinations: &mut Vec<String>, mut max_grps: i32, current: String) {
    if let Some((first, rest)) = s.split_once('?') {
        let mut current = current.clone();
        current.push_str(first);
        // generate_combinations_helper(rest, combinations, current.clone());

        // if first.contains('.') {
        //     max_grps -= 1;
        // }
        if max_grps>=0 {
            current.push('.');
            generate_combinations_helper(rest, combinations, max_grps, current.clone());
            current.pop();
            current.push('#');
            generate_combinations_helper(rest, combinations, max_grps, current);
        // } else {
        //     current.push('#');
        }

    } else {
        // let test=current.clone()+s;
        // let ce = combo_eval(test);
        // let grps = (ce.chars().filter(|c| *c==',').count()+1) as i32;
        // if grps>max_grps {
        combinations.push(current + s);
            // println!("{} {}", grps, max_grps);
        // } else {
        //     panic!("Not correct # {}", ce);
        // }
    }
}

fn combo_eval(s: String) -> String {
    let split_s: Vec<usize> = s.split('.').filter(|&s| !s.is_empty()).map(|s| s.len()).collect();
    let out_str = split_s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    // println!("{} {}", s, out_str);
    out_str
}
