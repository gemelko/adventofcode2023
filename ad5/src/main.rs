use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {
    if let Ok(lines_raw) = read_lines("C:\\rust\\DevelopmentFolder\\ad5input.txt") {
        let lines: Vec<_> = lines_raw.collect();
        let mut seeds: Vec<i64> = Vec::new();
        let mut seed_to_soil: Vec<(i64, i64, i64)> = Vec::new();
        let mut soil_to_fert: Vec<(i64, i64, i64)> = Vec::new();
        let mut fert_to_water: Vec<(i64, i64, i64)> = Vec::new();
        let mut water_to_light: Vec<(i64, i64, i64)> = Vec::new();
        let mut light_to_temp: Vec<(i64, i64, i64)> = Vec::new();
        let mut temp_to_hum: Vec<(i64, i64, i64)> = Vec::new();
        let mut hum_to_loc: Vec<(i64, i64, i64)> = Vec::new();
        let mut map_val: i64 = 0;
        for line in lines {
            let this_line = line.unwrap_or(String::new()).to_lowercase();
            if this_line.len() > 0 {
                if this_line.chars().nth(0).unwrap().is_digit(10) {
                    if let Ok(map_row) = scan_fmt!(this_line.as_str(), "{} {} {}", i64, i64, i64) {
                        match map_val {
                            0 => seed_to_soil.push(map_row),
                            1 => soil_to_fert.push(map_row),
                            2 => fert_to_water.push(map_row),
                            3 => water_to_light.push(map_row),
                            4 => light_to_temp.push(map_row),
                            5 => temp_to_hum.push(map_row),
                            6 => hum_to_loc.push(map_row),
                            _ => panic!("No map {}", map_val),
                        }
                    } else {
                        panic!("Can't parse {}", this_line);
                    }
                } else {
                    if this_line.contains("map") {
                        if this_line.contains("seed-to-") {
                            map_val=0;
                        } else if this_line.contains("soil-to-") {
                            map_val=1;
                        } else if this_line.contains("fertilizer-to") {
                            map_val=2;
                        } else if this_line.contains("water-to") {
                            map_val=3;
                        } else if this_line.contains("light-to") {
                            map_val=4;
                        } else if this_line.contains("temperature-to") {
                            map_val=5;
                        } else if this_line.contains("humidity-to") {
                            map_val=6;
                        } else {
                            panic!("Unknown map: {}", this_line);
                        }
                    } else if this_line.contains("seeds:") {
                        seeds = this_line.split(":").nth(1).unwrap().trim().split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                        println!("{:?}", seeds);
                    } else {
                        panic!("Processing error: Line {}", this_line);
                    }
                }
            }
        } 
        let mut min_loc: i64 = i64::MAX;
        for seed in &seeds {
            let mut map_val: i64 = *seed;
            map_val = xmap(&seed_to_soil, map_val);
            map_val = xmap(&soil_to_fert, map_val);
            map_val = xmap(&fert_to_water, map_val);
            map_val = xmap(&water_to_light, map_val);
            map_val = xmap(&light_to_temp, map_val);
            map_val = xmap(&temp_to_hum, map_val);
            map_val = xmap(&hum_to_loc, map_val);
            if map_val < min_loc {
                min_loc = map_val;
            }
        }
        println!("Part 1 min location: {}", min_loc);
        let mut min_loc_2: i64 = i64::MAX;
        let seeds2: Vec<(i64, i64)> = seeds.chunks(2).map(|item| (item[0], item[1])).collect();
        for (seed2, len2) in seeds2 {
            println!("Part 2 seed/count {}/{}", seed2, len2);
            for seed in (seed2)..(seed2+len2) {
                let mut map_val: i64 = seed;
                map_val = xmap(&seed_to_soil, map_val);
                map_val = xmap(&soil_to_fert, map_val);
                map_val = xmap(&fert_to_water, map_val);
                map_val = xmap(&water_to_light, map_val);
                map_val = xmap(&light_to_temp, map_val);
                map_val = xmap(&temp_to_hum, map_val);
                map_val = xmap(&hum_to_loc, map_val);
                if map_val < min_loc_2 {
                    min_loc_2 = map_val;
                }
            }
        }
        println!("Part 2 min location: {}", min_loc_2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}

fn xmap(map_table: &Vec<(i64, i64, i64)>, map_in: i64) -> i64 {
    for (map_trans, map_start, map_len) in map_table {
        if map_in >= *map_start && map_in < *map_start + *map_len {
            return *map_trans + map_in - *map_start;
        }
    }
    map_in
}