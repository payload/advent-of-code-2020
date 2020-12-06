use regex::{self, Regex};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day1_1()?;
    day1_2()?;
    day2()?;
    day3()?;
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

#[derive(Debug, Clone)]
struct PasswordEntry(usize, usize, char, String);

fn day2() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "src/passwords.input";
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)")?;
    let entries: Vec<_> = fs::read_to_string(&filepath)?
        .split('\n')
        .map(|line| {
            regex.captures(line).map(|caps| {
                PasswordEntry(
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                    caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                    caps.get(4).unwrap().as_str().to_string(),
                )
            })
        })
        .filter_map(|o| o)
        .collect();

    let part1_hits = entries
        .iter()
        .filter(|PasswordEntry(min, max, letter, password)| {
            let char_count = &password.chars().filter(|c| letter == c).count();
            return min <= char_count && char_count <= max;
        })
        .count();

    let part2_hits: Vec<_> = entries
        .iter()
        .filter(|&e| day2_2_check(e.clone()))
        .collect();
    println!("day 2 part 1: {}", part1_hits);
    println!("day 2 part 2: {}", part2_hits.len());
    Ok(())
}

fn day2_2_check(PasswordEntry(pos1, pos2, letter, password): PasswordEntry) -> bool {
    let at1 = password.chars().nth(pos1 - 1) == Some(letter);
    let at2 = password.chars().nth(pos2 - 1) == Some(letter);
    (at1 && !at2) || (!at1 && at2)
}

#[test]
fn day2_2() {
    let password = "ddddmldddzddgnk".into();
    assert!(!day2_2_check(PasswordEntry(10, 15, 'd', password)));
}

fn day3() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "src/forest.input";
    let map = fs::read_to_string(&filepath)?.trim().to_string();
    let trees = day3_count_trees(&map, 3, 1);
    println!("day 3 part 1: {}", trees);
    let product: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| day3_count_trees(&map, *x, *y))
        .product();
    println!("day 3 part 2: {}", product);
    Ok(())
}

fn day3_count_trees(input: &str, x_dir: usize, y_dir: usize) -> usize {
    let map: Vec<char> = input.chars().collect();
    let width = map.iter().position(|&c| c == '\n').unwrap();
    let height = map.len() / (width + 1);
    let rows = height / y_dir;
    // let mut map2 = map.clone();
    // for index in 1..=rows {
    //     let row = dbg!(index) * y_dir;
    //     let col = dbg!(index * x_dir) % dbg!(width);
    //     let pos = dbg!(row * (width + 1)) + dbg!(col);
    //     let tree = map[pos] == '#';
    //     map2[pos] = if tree { 'X' } else { 'O' };
    // }
    // println!("{}\n", map2.into_iter().collect::<String>());
    (1..=rows)
        .filter(|index| {
            let row = index * y_dir;
            let col = (index * x_dir) % width;
            let pos = row * (width + 1) + col;
            map[pos] == '#'
        })
        .count()
}

#[test]
fn day3_1() {
    assert_eq!(3, day3_count_trees("....\n...#\n##.#\n.#..\n#...", 3, 1));
    assert_eq!(1, day3_count_trees("....\n....\n....\n...#", 1, 1));
}

#[test]
fn day3_2() {
    let map = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    assert_eq!(2, day3_count_trees(&map, 1, 1));
    assert_eq!(7, day3_count_trees(&map, 3, 1));
    assert_eq!(3, day3_count_trees(&map, 5, 1));
    assert_eq!(4, day3_count_trees(&map, 7, 1));
    assert_eq!(2, day3_count_trees(&map, 1, 2));
}
