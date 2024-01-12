use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {

    let mut net_paths_i: Vec<String> = Vec::new();
    let mut net_paths_l: Vec<String> = Vec::new();
    let mut net_paths_r: Vec<String> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();

    if let Ok(lines_raw) = read_lines("C:\\rust\\DevelopmentFolder\\ad8input.txt") {
        let lines: Vec<_> = lines_raw.collect();
        for line in lines {
            if instructions.len()==0 {
                instructions = line.unwrap().chars().collect();
            } else {
                if let Ok((this_node, left_node, right_node)) = scan_fmt!(line.unwrap().as_str(), "{3s} = ({3s} {3s})", String, String, String) {
                    net_paths_i.push(this_node);
                    net_paths_l.push(left_node[0..3].to_string());
                    net_paths_r.push(right_node[0..3].to_string());
                }
            }
        }
    }

    let mut t_table: Vec<(usize, usize)> = Vec::new();

    let left_entries: Vec<usize> = net_paths_l.iter().map(|path| net_paths_i.iter().position(|p| p == path).unwrap()).collect();
    let right_entries: Vec<usize> = net_paths_r.iter().map(|path| net_paths_i.iter().position(|p| p == path).unwrap()).collect();
    let is_zzz: Vec<bool> = net_paths_i.iter().map(|path| path=="ZZZ").collect();
    let inst_bool: Vec<bool> = instructions.iter().map(|inst| *inst=='R').collect();
    let mut done: bool = false;
    let mut search_node: usize;
    let mut inst_index: usize = 0;
    let mut count: i64 = 0;
    let search_node_list: Vec<usize> = net_paths_i.iter().enumerate().filter(|(_, path)| *path=="AAA").map(|(ind, _)| ind).collect();
    search_node = search_node_list[0];
    println!("{:?}", search_node);

    while !is_zzz[search_node] {
        if inst_bool[inst_index] {
            search_node = right_entries[search_node];
        } else {
            search_node = left_entries[search_node];
        }
        inst_index += 1;
        if inst_index == instructions.len() {
            inst_index = 0;
        }
        count += 1;
    }
    println!("Part 1 steps: {}", count);

    let mut done: bool = false;
    let mut search_nodes: Vec<String> = Vec::new();
    let mut new_search_nodes: Vec<String> = Vec::new();
    // let mut inst_index: usize = 0;
    // let mut count: i32 = 0;
    let mut show: i32 = 0;

    count = 0;
    inst_index = 0;
    let mut end_count: usize = 0;

    let mut search_nodes: Vec<usize> = net_paths_i.iter().enumerate().filter(|(_, path)| path.chars().nth(2)==Some('A')).map(|(ind, _)| ind).collect();
    let is_xxz: Vec<bool> = net_paths_i.iter().map(|path| path.chars().nth(2)==Some('Z')).collect();
    // println!("{:?}", search_nodes);
    
    while end_count<6 {
        end_count = 0;
        for search_node_i in 0..search_nodes.len() {
            if inst_bool[inst_index] {
                search_nodes[search_node_i] = right_entries[search_nodes[search_node_i]];
            } else {
                search_nodes[search_node_i] = left_entries[search_nodes[search_node_i]];
            }
            if is_xxz[search_nodes[search_node_i]] {
                end_count += 1;
            }
        }
        if end_count>2 {
            if let Some(output) = search_nodes.get(0..6) {
                let out_vals: Vec<String> = output.iter().filter_map(|&ind| net_paths_i.get(ind)).cloned().collect();
                println!("{} {} {:?}", count+1, end_count, out_vals);
            }
        }
        inst_index += 1;
        if inst_index == instructions.len() {
            inst_index = 0;
        }
        count += 1;
    }
    println!("Part 2 steps: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}
