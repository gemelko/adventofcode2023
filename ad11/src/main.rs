use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use scan_fmt::scan_fmt;

fn main() {
    let mut vexp_lines: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("C:\\rust\\DevelopmentFolder\\ad11input.txt") {
        println!("Height: {}", lines.len());
        for line in lines.iter() {
            vexp_lines.push(line.to_string());
            if *line == ".".repeat(line.len()) {
                vexp_lines.push(line.to_string());
            }
        }
        println!("Vexp Height: {}", vexp_lines.len());
        println!("Width: {}", vexp_lines[0].len());
        let mut hexp_pos: Vec<usize> = Vec::new();
        for (icol, col) in vexp_lines[0].char_indices() {
            if col=='.' {
                let mut all_empty: bool = true;
                for line in vexp_lines.iter() {
                    if line.chars().nth(icol) != Some('.') {
                        all_empty = false;
                        break;
                    }
                }
                if all_empty {
                    hexp_pos.push(icol);
                }
            }
        }
        let mut exp_lines: Vec<String> = Vec::new();
        for line in vexp_lines.iter() {
            // println!("Before: {}", line);
            let mut new_line = line.clone();
            for ipos in hexp_pos.iter().rev() {
                let mut new_chars: Vec<char> = new_line.chars().collect();
                new_chars.insert(*ipos, '.');
                new_line = new_chars.into_iter().collect();
            }
            println!("After.: {}", new_line);
            // break;
            exp_lines.push(new_line);
        }
        println!("Hexp width: {}", exp_lines[0].len());

        // Pre expansion
        let mut g_coords: Vec<(usize, usize)> = Vec::new();
        for (y_coord, line) in lines.iter().enumerate() {
            let x_coord: Vec<usize> = line.char_indices().filter(|(_, c)| *c=='#').map(|(p, _)| p).collect();
            g_coords.extend(x_coord.iter().map(|&p| (p, y_coord)));
        }
        // println!("{:?}", exp_lines);
        println!("Number of galaxies {}", g_coords.len());
        let g_pairs: Vec<((usize, usize), (usize, usize))> = g_coords.iter()
            .enumerate()
            .flat_map(|(i, &x)| g_coords.iter().skip(i + 1).map(move |&y| (x, y)))
            .collect();
        println!("Number of pairs {}", g_pairs.len());
        let mut pre_total_len: i32 = 0;
        for (from_g, to_g) in g_pairs.iter() {
            let my_dist = (from_g.0 as i32 - to_g.0 as i32).abs() + (from_g.1 as i32 - to_g.1 as i32).abs();
            // println!("{:?} {:?} {}", from_g, to_g, my_dist);
            pre_total_len += my_dist;
        }
        println!("Pre expansion length {}", pre_total_len);

// Post expansion
        let mut g_coords: Vec<(usize, usize)> = Vec::new();
        for (y_coord, line) in exp_lines.iter().enumerate() {
            let x_coord: Vec<usize> = line.char_indices().filter(|(_, c)| *c=='#').map(|(p, _)| p).collect();
            g_coords.extend(x_coord.iter().map(|&p| (p, y_coord)));
        }
        // println!("{:?}", exp_lines);
        println!("Number of galaxies {}", g_coords.len());
        let g_pairs: Vec<((usize, usize), (usize, usize))> = g_coords.iter()
            .enumerate()
            .flat_map(|(i, &x)| g_coords.iter().skip(i + 1).map(move |&y| (x, y)))
            .collect();
        println!("Number of pairs {}", g_pairs.len());
        let mut total_len: i32 = 0;
        for (from_g, to_g) in g_pairs.iter() {
            let my_dist = (from_g.0 as i32 - to_g.0 as i32).abs() + (from_g.1 as i32 - to_g.1 as i32).abs();
            // println!("{:?} {:?} {}", from_g, to_g, my_dist);
            total_len += my_dist;
        }
        println!("Total distance {}", total_len);
        println!("Expansion at 1Mx {}", ((total_len-pre_total_len) as i64)*999999 + pre_total_len as i64);
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
