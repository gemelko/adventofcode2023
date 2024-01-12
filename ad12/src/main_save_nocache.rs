use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use scan_fmt::scan_fmt;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i64 = 0;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad12input.txt") {
        for line in lines {
            if let Some((s1_val, t1_val)) = line.split_once(' ') {
                let s_val = (0..5).map(|_| s1_val).collect::<Vec<&str>>().join("?");
                let t_val = (0..5).map(|_| t1_val).collect::<Vec<&str>>().join(",");
                println!("{} {}",my_count, s_val);
                println!("{} {}",my_count, t_val);
                let combos = generate_combinations(&s_val, &t_val);
                println!("Combos: {}", combos);
                // for combo in combos {
                //     // println!("{}", combo);
                //     let te_val = combo_eval(combo.clone());
                //     if te_val == *t_val {
                //         my_total += 1;
                //         // println!("True");
                //     } else {
                //         // println!("False");
                //     }
                // }
                my_total += combos;
                println!("{}", my_total);
                my_count += 1;
                // if my_count>1 {
                //     break;
                // }
                // break;
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

fn generate_combinations(s: &str, t: &str) -> i64 {
    let mut combinations: i64 = 0;
    let mut current_combination = String::new();
    let tvec: Vec<i32> = parse_lengths(t);
    generate_combinations_recursive(s, tvec, &mut combinations, &mut current_combination, 0, 0);
    combinations
}

fn generate_combinations_recursive(s: &str, tvec: Vec<i32>, combinations: &mut i64, current: &mut String, curindex: usize, clen: usize) {
    // println!(": {} {} {} {:?}", current, &s[curindex..], clen, tvec);
    if curindex == s.len() {
        if tvec.len()==0 || ((tvec.len()==1) && (clen==(tvec[0] as usize))) {
            // println!("EoS: {} {:?}", current, tvec);
            // combinations.push(current.clone());
            *combinations = *combinations + 1 as i64;
        // } else {
        //     println!("not enough vecs");
        //     println!(": {} {} {} {:?}", current, &s[curindex..], clen, tvec);
        }
        return;
    }

    if tvec.len()==0 {
        if s[curindex..].chars().any(|c| c == '#') == false {
            // for nth in curindex..s.len() {
            //     current.push('.');
            // }
            // println!("Vec: {} {:?}", current, tvec);
            *combinations = *combinations + 1 as i64;
            // combinations.push(current.clone());
            // println!("Try: {}", current);
        } else {
            println!("too many #'s");
            println!(": {} {} {} {:?}", current, &s[curindex..], clen, tvec);
        }
        return;
    }

    if clen > tvec[0] as usize {
        return;
    }

    let current_char = s.chars().nth(curindex).unwrap();

    if current_char == '?' {
        // Try '.' as the next character
        if clen>0 {
            if clen == tvec[0] as usize {
                // println!("Force .");
                current.push('.');
                generate_combinations_recursive(s, tvec.clone()[1..tvec.len()].to_vec(), combinations, current, curindex + 1, 0);
                current.pop();
                // } else {
            //     // println!("Add .");
            //     generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, 0);
            }
        } else {
            // println!("Test .");
            current.push('.');
            generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, 0);
            current.pop();
        }

        // Try '#' as the next character only if it doesn't result in more than tvec[0] '#'s in a row
        if clen < tvec[0] as usize {
            // println!("Try #");
            current.push('#');
            generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, clen+1);
            current.pop();
        }
    } else {
        if current_char=='.' {
            if clen==tvec[0] as usize {
                current.push('.');
                generate_combinations_recursive(s, tvec.clone()[1..tvec.len()].to_vec(), combinations, current, curindex + 1, 0);
                current.pop();
            } else if clen==0 {
            //     println!("c-len wrong: {} {}", clen, tvec[0]);
                current.push('.');
                generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, 0);
                current.pop();
            }
        } else {
            if clen < tvec[0] as usize {
                current.push('#');
                generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, clen+1);
                current.pop();
            }
        }
        // current.pop();
    }
}

fn combo_eval(s: String) -> String {
    let split_s: Vec<usize> = s.split('.').filter(|&s| !s.is_empty()).map(|s| s.len()).collect();
    let out_str = split_s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    // println!("{} {}", s, out_str);
    out_str
}

fn parse_lengths(inpstr: &str) -> Vec<i32> {
    inpstr
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}
