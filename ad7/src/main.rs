use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

fn main() {

    let mut hands: Vec<(String, i32)> = Vec::new();

    if let Ok(lines_raw) = read_lines("C:\\rust\\DevelopmentFolder\\ad7input.txt") {
        let lines: Vec<_> = lines_raw.collect();
        for line in lines {
            if let Ok((this_hand, this_value)) = scan_fmt!(line.unwrap().as_str(), "{5s} {}i32", String, i32) {
                hands.push((this_hand, this_value));
            }
        }
        hands.sort_by(poker_cmp);

        // println!("{:?}", hands);
        let mut score: i64 = 0;
        for (ind, han) in hands.iter().enumerate() {
            score += ((ind as i64) + 1) * (han.1 as i64);
        }
        println!("Score: {}", score);

        hands.sort_by(poker_joker_cmp);
        // println!("{:?}", hands);
        let mut jscore: i64 = 0;
        for (ind, han) in hands.iter().enumerate() {
            jscore += ((ind as i64) + 1) * (han.1 as i64);
        }
        println!("Joker Score: {}", jscore);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let my_file = File::open(filename)?;
    Ok(io::BufReader::new(my_file).lines())
}

fn poker_cmp(a: &(String, i32), b: &(String, i32)) -> std::cmp::Ordering {
    let hand_a = &a.0;
    let hand_b = &b.0;
    let hand_a_val = poker_eval(hand_a);
    let hand_b_val = poker_eval(hand_b);
    return hand_a_val.cmp(&hand_b_val);
}

fn poker_eval(a: &String) -> i64 {
    let cha: Vec<char> = a.chars().collect();
    let mut cnt: Vec<i32> = vec![0, 0, 0, 0, 0];
    let mut tres: i64 = 0;
    let mut tfra: i64 = 100000000;
    for i in 0..5 {
        cnt[i] = cha.iter().filter(|&t| *t==cha[i]).count() as i32;
        // put the value of the cards into cardinal order based on 2 digits per card
        tres += cval(cha[i]) * tfra;
        tfra /= 100;
    }
    // I expect sums of
    // 5 of a kind: 25
    // 4 of a kind: 17
    // full house: 13
    // 3 of a kind: 11
    // two pair: 9
    // one pair: 5
    // all 5 different: 4
    (cnt.iter().sum::<i32>() as i64) * 10000000000 + tres // hands will be ranked higher when more cards match
}

fn cval(a: char) -> i64 {
    match a {
        '2'..='9' => a.to_digit(10).unwrap() as i64,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card {}", a),
    }
}

fn poker_joker_cmp(a: &(String, i32), b: &(String, i32)) -> std::cmp::Ordering {
    let hand_a = &a.0;
    let hand_b = &b.0;
    let hand_a_val = poker_joker_eval(hand_a);
    let hand_b_val = poker_joker_eval(hand_b);
    return hand_a_val.cmp(&hand_b_val);
}

fn poker_joker_eval(a: &String) -> i64 {
    let mut cha: Vec<char> = a.chars().collect();
    let mut cnt: Vec<i32> = vec![0, 0, 0, 0, 0];
    let mut tres: i64 = 0;
    let mut tfra: i64 = 100000000;
    let mut maxc: i32 = 0;
    let mut chac: char = 'J';

    for i in 0..5 {
        cnt[i] = cha.iter().filter(|&t| *t==cha[i]).count() as i32;
        if cnt[i]>maxc && cha[i]!='J'{
            maxc = cnt[i];
            chac = cha[i];
        }
        // put the value of the cards into cardinal order based on 2 digits per card
        tres += jcval(cha[i]) * tfra;
        tfra /= 100;
    }
    for i in 0..5 {
        if cha[i]=='J' {
            cha[i]=chac;
        }
    }
    for i in 0..5 {
        cnt[i] = cha.iter().filter(|&t| *t==cha[i]).count() as i32;
        // put the value of the cards into cardinal order based on 2 digits per card
    }
    // I expect sums of
    // 5 of a kind: 25
    // 4 of a kind: 17
    // full house: 13
    // 3 of a kind: 11
    // two pair: 9
    // one pair: 5
    // all 5 different: 4
    (cnt.iter().sum::<i32>() as i64) * 10000000000 + tres // hands will be ranked higher when more cards match
}

fn jcval(a: char) -> i64 {
    match a {
        '2'..='9' => a.to_digit(10).unwrap() as i64,
        'T' => 10,
        'J' => 1, // Jokers are low
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card {}", a),
    }
}
