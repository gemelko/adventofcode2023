use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {

    let mut net_paths: Vec<(String, (String, String))> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();

    if let Ok(lines_raw) = read_lines("C:\\rust\\DevelopmentFolder\\ad8input.txt") {
        let lines: Vec<_> = lines_raw.collect();
        for line in lines {
            if instructions.len()==0 {
                instructions = line.unwrap().chars().collect();
            } else {
                if let Ok((this_node, left_node, right_node)) = scan_fmt!(line.unwrap().as_str(), "{3s} = ({3s} {3s})", String, String, String) {
                    net_paths.push((this_node, (left_node[0..3].to_string(), right_node[0..3].to_string())));
                }
            }
        }
    }
    
    let mut done: bool = false;
    let mut search_node: String = String::new();
    let mut inst_index: usize = 0;
    let mut count: i32 = 0;
    search_node = "AAA".to_string();
    println!("{:?}", search_node);

    while !done {
        if let Some((_, (l_val, r_val))) = net_paths.iter().find(|(node, _)| node == &search_node) {
            // println!("{}: {} {}", search_node, l_val, r_val);
            if instructions[inst_index]=='L' {
                search_node = l_val.to_string();
            } else {
                search_node = r_val.to_string();
            }
            inst_index += 1;
            if inst_index == instructions.len() {
                inst_index = 0;
            }
            count += 1;
        }
        if search_node == "ZZZ" {
            done = true;
        }
    }
    println!("Part 1 steps: {}", count);

    // let mut done: bool = false;
    let mut search_nodes: Vec<String> = Vec::new();
    let mut new_search_nodes: Vec<String> = Vec::new();
    // let mut inst_index: usize = 0;
    // let mut count: i32 = 0;
    let mut show: i32 = 0;

    count = 0;
    inst_index = 0;
    done = false;

    search_nodes = net_paths.iter().map(|(node, _)| node).filter(|node| node.chars().nth(2) == Some('A')).cloned().collect();
    // println!("{:?}", search_nodes);
    
    while !done {
        new_search_nodes.clear();
        done = true;
        // println!("{:?}", search_nodes);
        show = 0;
        for search_node in search_nodes {
            if let Some((_, (l_val, r_val))) = net_paths.iter().find(|(node, _)| node == &search_node) {
                // println!("{}: {} {}", search_node, l_val, r_val);
                if instructions[inst_index]=='L' {
                    new_search_nodes.push(l_val.to_string());
                    if l_val.chars().nth(2)!=Some('Z') {
                        done=false;
                    } else {
                        show+=1;
                    }
                } else {
                    new_search_nodes.push(r_val.to_string());
                    if r_val.chars().nth(2)!=Some('Z') {
                        done=false;
                    } else {
                        show+=1;
                    }
                }
            }
        }
        if show>2 {
            println!("{} {} {:?}", count+1, show, new_search_nodes);
        }
        inst_index += 1;
        if inst_index == instructions.len() {
            inst_index = 0;
        }
        count += 1;
        search_nodes = new_search_nodes.clone();
        // if count > 3 {
        //     done = true;
        // }
    }
    println!("Part 2 steps: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}
