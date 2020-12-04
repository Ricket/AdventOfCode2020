use std::env;
use std::fs;

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

    day1part2(input);
}

fn sorted_int_vec(input: String) -> Vec<i32> {
    let mut numbers: Vec<i32> = input.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    numbers
}

fn day1part1(input: String) {
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
    let numbers = sorted_int_vec(input);

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
