#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Args: day part inputfile");
    }
    let day = &args[1];
    let part = &args[2];
    let input_filename = &args[3];
    let input = fs::read_to_string(input_filename)
        .expect("Something went wrong loading input file");

    day2part1(&input);
}

fn sorted_int_vec(input: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = input.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    numbers
}

fn day1part1(input: &str) {
    let numbers = sorted_int_vec(input);
    // println!("{:?}", numbers);

    let mut low_idx = 0;
    let mut high_idx = numbers.len() - 1;

    while low_idx < high_idx {
        let sum = numbers[low_idx] + numbers[high_idx];

        if sum == 2020 {
            println!("{:?}", numbers[low_idx] * numbers[high_idx]);
            std::process::exit(0);
        } else if sum < 2020 {
            low_idx += 1;
        } else /* if sum > 2020 */ {
            high_idx -= 1;
        }
    }

}

fn day1part2(input: String) {
    let numbers = sorted_int_vec(&input);

    if numbers.len() < 3 {
        panic!("Less than 3 input numbers");
    }

    for low in 0..numbers.len() {
        for med in low..numbers.len() {
            for high in med..numbers.len() {
                let sum = numbers[low] + numbers[med] + numbers[high];
                if sum == 2020 {
                    println!("{:?}", numbers[low] * numbers[med] * numbers[high]);
                    std::process::exit(0);
                }
            }
        }
    }
}

fn count_occurrences(input: &str, desired: char) -> usize {
    input.chars()
        .map(|c| if c == desired { Some(1) } else { None })
        .filter(|x| x.is_some())
        .count()
}

fn day2part1(input: &str) {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();
    }

    let matching_pws: usize = input.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| RE.captures(s).and_then(|cap| {
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let c: char = cap[3].parse().unwrap();
            let password = &cap[4];
            let count = count_occurrences(password, c);
            if count >= min && count <= max {
                Some(1)
            } else {
                None
            }
        }))
        .filter(|x| x.is_some())
        .count();
    println!("{:?}", matching_pws);

}
