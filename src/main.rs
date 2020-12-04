use std::fs;

fn main() -> std::io::Result<()> {
    day1_1()?;
    day1_2()?;
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
        };
        None
    };
    let hit = find_hit().expect("no hit found");
    println!("day1 2: {}", hit);
    Ok(())
}

