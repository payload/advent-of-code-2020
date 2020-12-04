use regex::{self, Regex};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1_1()?;
    day1_2()?;
    day2()?;
    Ok(())
}

fn day1_1() -> std::io::Result<()> {
    let filepath = "src/expense report.input";
    let numbers: Vec<i32> = fs::read_to_string(&filepath)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();
    let hit = numbers
        .iter()
        .find(|&n| numbers.contains(&(2020 - n)))
        .expect("no numbers found adding up to 2020");
    println!("day1 1: {}", hit * (2020 - hit));
    Ok(())
}

fn day1_2() -> std::io::Result<()> {
    let filepath = "src/expense report.input";
    let numbers: Vec<i32> = fs::read_to_string(&filepath)?
        .split('\n')
        .filter_map(|s| s.parse().ok())
        .collect();
    let find_hit = || {
        for a in numbers.iter() {
            for b in numbers.iter().filter(|&&n| n < 2020 - a) {
                let c: i32 = 2020 - a - b;
                if numbers.contains(&c) {
                    return Some(a * b * c);
                }
            }
        }
        None
    };
    let hit = find_hit().expect("no hit found");
    println!("day1 2: {}", hit);
    Ok(())
}

fn day2() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "src/passwords.input";
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)")?;
    let hits = fs::read_to_string(&filepath)?
        .split('\n')
        .filter(|s| {
            if let Some(caps) = regex.captures(s) {
                let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
                let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
                let letter = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
                let password = caps.get(4).unwrap().as_str();
                let char_count = password.chars().filter(|&c| letter == c).count();
                return min <= char_count && char_count <= max;
            }
            false
        })
        .count();
    println!("day2: {}", hits);
    Ok(())
}
