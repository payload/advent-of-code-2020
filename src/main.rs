#![feature(str_split_once)]
#![feature(or_patterns)]
#![feature(iterator_fold_self)]

use regex::{self, Regex};
use std::{collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    day1_1()?;
    day1_2()?;
    day2()?;
    day3()?;
    day4()?;
    day5()?;
    day6()?;
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

fn day2() -> Result<(), Box<dyn Error>> {
    let filepath = "src/passwords.input";
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)")?;
    let entries: Vec<_> = fs::read_to_string(&filepath)?
        .split('\n')
        .map(|line| {
            regex.captures(line).map(|caps| {
                PasswordEntry(
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                    caps.get(3).unwrap().as_str().chars().next().unwrap(),
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
            min <= char_count && char_count <= max
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

fn day3() -> Result<(), Box<dyn Error>> {
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

fn day4() -> Result<(), Box<dyn Error>> {
    let filepath = "src/passports.input";
    let content = fs::read_to_string(&filepath)?;
    let entries = parse_passports(&content);
    println!(
        "day 4 part 1: {}",
        entries.iter().filter(|e| has_mandatory_fields(e)).count()
    );
    println!(
        "day 4 part 2: {}",
        entries.iter().filter(|e| is_valid_passport(e)).count()
    );
    Ok(())
}

fn parse_passports(content: &str) -> Vec<Vec<(&str, &str)>> {
    content
        .trim()
        .split("\n\n")
        .map(|entry| parse_passport(entry))
        .collect()
}

fn parse_passport(entry: &str) -> Vec<(&str, &str)> {
    entry
        .split_whitespace()
        .map(|kv| kv.split_once(':').expect("kv pair needs : separator"))
        .collect::<Vec<_>>()
}

fn is_valid_passport(entry: &[(&str, &str)]) -> bool {
    has_mandatory_fields(&entry) && entry.iter().all(field_is_valid)
}

fn field_is_valid(field: &(&str, &str)) -> bool {
    match field {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        ("byr", v) => is_year(v, 1920, 2002),
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        ("iyr", v) => is_year(v, 2010, 2020),
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        ("eyr", v) => is_year(v, 2020, 2030),
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        ("hgt", v) => is_height(v, "cm", 150, 193) || is_height(v, "in", 59, 76),
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        ("hcl", v) => is_colorcode(v),
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        ("ecl", "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") => true,
        ("ecl", _) => false,
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        ("pid", v) => v.len() == 9 && v.chars().all(char::is_numeric),
        // cid (Country ID) - ignored, missing or not.
        ("cid", _) => true,
        _ => true,
    }
}

fn has_mandatory_fields(entry: &[(&str, &str)]) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|mandatory| entry.iter().any(|(key, _)| mandatory == key))
}

fn is_year(s: &str, min: usize, max: usize) -> bool {
    let n = s.parse::<usize>().expect("must be a number");
    n >= min && n <= max && s.chars().all(char::is_numeric) && s.len() == 4
}

fn is_height(s: &str, unit: &str, min: usize, max: usize) -> bool {
    if s.ends_with(unit) {
        let digits = s.trim_end_matches(unit);
        let n = digits.parse::<usize>().expect("must be a number");
        n >= min && n <= max
    } else {
        false
    }
}

fn is_colorcode(s: &str) -> bool {
    s.starts_with('#') && s[1..].chars().all(|c| c.is_digit(16))
}

#[test]
fn day4_invalid_passports() {
    let content = "
    eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

    iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946

    hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

    hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007";
    for passport in parse_passports(content) {
        assert!(!is_valid_passport(&passport));
    }
}

#[test]
fn day4_valid_passports() {
    let content = "
    pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f

    eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

    hcl:#888785
    hgt:164cm byr:2001 iyr:2015 cid:88
    pid:545766238 ecl:hzl
    eyr:2022

    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    for passport in parse_passports(content) {
        assert!(is_valid_passport(&passport));
    }
}

fn day5() -> Result<(), Box<dyn Error>> {
    let filepath = "src/boardingpasses.input";
    let content = fs::read_to_string(&filepath)?;
    let seats: Vec<_> = content
        .lines()
        .filter_map(parse_boardingpass)
        .map(|(id, _, _)| id)
        .collect();
    let max = seats.iter().max();
    println!("day 5 part 1: {:?}", max);

    let mut seats = seats.clone();
    seats.sort_unstable();
    let gaps: Vec<_> = seats
        .iter()
        .zip(&seats[1..])
        .filter(|(&a, &b)| b - a > 1)
        .collect();
    println!("day 5 part 2: {:?}", gaps);
    Ok(())
}

type DecodedPass = (usize, usize, usize);

fn parse_boardingpass(pass: &str) -> Option<DecodedPass> {
    if pass.len() == 10 {
        let row = binpart(128, 'F', 'B', &pass[..7]);
        let col = binpart(8, 'L', 'R', &pass[7..]);
        let id = row * 8 + col;
        Some((id, row, col))
    } else {
        None
    }
}

#[test]
fn day5_parse_boardingpass() {
    assert_eq!((567, 70, 7), parse_boardingpass("BFFFBBFRRR").unwrap());
    assert_eq!((119, 14, 7), parse_boardingpass("FFFBBBFRRR").unwrap());
    assert_eq!((820, 102, 4), parse_boardingpass("BBFFBBFRLL").unwrap());
}

fn binpart(space: usize, lo: char, hi: char, instructions: &str) -> usize {
    let mut a = 0;
    let mut b = space;
    for c in instructions.chars() {
        if c == lo {
            b /= 2;
        } else if c == hi {
            b /= 2;
            a += b;
        }
    }
    a
}

#[test]
fn day5_binpart() {
    assert_eq!(44, binpart(128, 'F', 'B', "FBFBBFF"));
    assert_eq!(5, binpart(8, 'L', 'R', "RLR"));
}

fn day6() -> Result<(), Box<dyn Error>> {
    let filepath = "src/customs.input";
    let content = fs::read_to_string(&filepath)?;
    let sum: usize = content.split("\n\n").map(count_any_yes).sum();
    println!("day 6 part 1: {}", sum);

    let sum: usize = content.split("\n\n").map(count_all_yes).sum();
    println!("day 6 part 2: {}", sum);
    Ok(())
}

fn count_any_yes(group: &str) -> usize {
    group
        .chars()
        .filter(char::is_ascii_lowercase)
        .collect::<HashSet<_>>()
        .len()
}

fn count_all_yes(group: &str) -> usize {
    group
        .lines()
        .map(|l| {
            l.chars()
                .filter(char::is_ascii_lowercase)
                .collect::<HashSet<_>>()
        })
        .fold_first(|a, b| a.intersection(&b).map(Clone::clone).collect())
        .unwrap()
        .len()
}
