use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use scan_fmt::scan_fmt;
use std::collections::HashMap;

fn main() {
    let mut my_count: i32 = 0;
    let mut my_total: i64 = 0;
    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad12input.txt") {
        for line in lines {
            if let Some((s1_val, t1_val)) = line.split_once(' ') {
                let s_val = (0..5).map(|_| s1_val).collect::<Vec<&str>>().join("?");
                let t_val = (0..5).map(|_| t1_val).collect::<Vec<&str>>().join(",");
                // println!("{} {}",my_count, s_val);
                // println!("{} {}",my_count, t_val);
                let combos = generate_combinations(&s_val, &t_val);
                // println!("Combos: {}", combos);
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
                // println!("{}", my_total);
                my_count += 1;
                // if my_count>0 {
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
    let mut current_combination = String::new();
    let tvec: Vec<i32> = parse_lengths(t);
    let svec: Vec<char> = s.chars().collect();
    let mut v_cache: HashMap<(Vec<i32>, Vec<char>), i64> = HashMap::new();
    generate_combinations_recursive(&svec, tvec, &mut current_combination, 0, 0, &mut v_cache)
}

fn generate_combinations_cached(svec: &[char], tvec: Vec<i32>, v_cache: &mut HashMap<(Vec<i32>, Vec<char>), i64>) -> i64 {
    let mut current_combination = String::new();
    if let Some(&value) = v_cache.get(&(tvec.clone(), svec.to_vec())) {
        // println!("Cached: {} {}: {}", tstr, s[curindex..].to_string(), value);
        return value;
    } else {
        let mut cachevalue: i64 = 0;
        cachevalue = generate_combinations_recursive(svec, tvec.clone(), &mut current_combination, 0, 0, v_cache);
        v_cache.insert((tvec, (*svec).to_vec()), cachevalue);
        return cachevalue;
    }
    // let this_value: i64 = generate_combinations_recursive(svec, tvec.clone(), &mut current_combination, 0, 0, v_cache);
    // if this_value>0 {
    //     println!("Sub: {:?} {:?} {}", svec, tvec, this_value);
    // }
    // this_value
}

fn generate_combinations_recursive(svec: &[char], tvec: Vec<i32>, current: &mut String, curindex: usize, clen: usize, v_cache: &mut HashMap<(Vec<i32>, Vec<char>), i64>) -> i64 {
    let mut my_value: i64 = 0;
    // println!(": {} {} {} {:?}", current, &s[curindex..], clen, tvec);
    if curindex == svec.len() {
        if tvec.len()==0 || ((tvec.len()==1) && (clen==(tvec[0] as usize))) {
            // println!("EoS: {} {:?}", current, tvec);
            // combinations.push(current.clone());
            return 1;
            // *combinations = *combinations + 1 as i64;
        // } else {
        //     println!("not enough vecs");
        //     println!(": {} {} {} {:?}", current, &s[curindex..], clen, tvec);
        }
        return 0;
    }

    if tvec.len()==0 {
        if svec[curindex..].iter().any(|c| *c == '#') == false {
            // for nth in curindex..s.len() {
            //     current.push('.');
            // }
            // println!("Vec: {} {:?}", current, tvec);
            // *combinations = *combinations + 1 as i64;
            return 1;
            // combinations.push(current.clone());
            // println!("Try: {}", current);
        } else {
            // println!("too many #'s");
            // println!(": {} {} {} {:?}", current, svec[curindex..].to_string(), clen, tvec);
        }
        return 0;
    }

    if clen > tvec[0] as usize {
        return 0;
    }

    let current_char = svec[curindex];

    if current_char == '?' {
        // Try '.' as the next character
        if clen>0 {
            if clen == tvec[0] as usize {
                let mut d_value: i64 = 0;
                // println!("Force .");
                current.push('.');
                if tvec.len()>0 {
                    d_value = generate_combinations_cached(&svec.clone()[curindex+1..], tvec.clone()[1..].to_vec(), v_cache);
                } else {
                    d_value = generate_combinations_recursive(svec, tvec.clone()[1..].to_vec(), current, curindex + 1, 0, v_cache);
                }
                my_value += d_value;
                current.pop();
                    // } else {
            //     // println!("Add .");
            //     generate_combinations_recursive(s, tvec.clone(), combinations, current, curindex + 1, 0);
            }
        } else {
            // println!("Test .");
            current.push('.');
            my_value += generate_combinations_recursive(svec, tvec.clone(), current, curindex + 1, 0, v_cache);
            current.pop();
            // if s[curindex..].len()<15 && my_value>0 {
            //     let tstr = tvec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
            //     v_cache.insert((tstr.clone(), s[curindex..].to_string()), my_value);
            //     // println!("Computed: {} {}: {}", tstr, s[curindex..].to_string(), my_value);
            // }
        }

        // Try '#' as the next character only if it doesn't result in more than tvec[0] '#'s in a row
        if clen < tvec[0] as usize {
            // println!("Try #");
            current.push('#');
            my_value += generate_combinations_recursive(svec, tvec.clone(), current, curindex + 1, clen+1, v_cache);
            current.pop();
        }
    } else {
        if current_char=='.' {
            if clen==tvec[0] as usize {
                let mut d_value: i64 = 0;
                current.push('.');
                // my_value += generate_combinations_cached(&svec[curindex+1..], tvec.clone()[1..tvec.len()].to_vec(), v_cache);
                if tvec.len()>0 {
                    d_value = generate_combinations_cached(&svec.clone()[curindex+1..], tvec.clone()[1..].to_vec(), v_cache);
                } else {
                    d_value = generate_combinations_recursive(svec, tvec.clone()[1..].to_vec(), current, curindex + 1, 0, v_cache);
                }
                my_value += d_value;
                current.pop();
            } else if clen==0 {
            //     println!("c-len wrong: {} {}", clen, tvec[0]);
                current.push('.');
                my_value += generate_combinations_recursive(svec, tvec.clone(), current, curindex + 1, 0, v_cache);
                current.pop();
            }
        } else {
            if clen < tvec[0] as usize {
                current.push('#');
                my_value += generate_combinations_recursive(svec, tvec.clone(), current, curindex + 1, clen+1, v_cache);
                current.pop();
            }
        }
        // current.pop();
    }
    return my_value;
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
