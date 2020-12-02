use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need to pass input filename");
    }
    let input_filename = &args[1];
    let input = fs::read_to_string(input_filename)
        .expect("Something went wrong loading input file");
    let mut numbers: Vec<i32> = input.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    numbers.sort_unstable();

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

    println!("{:?}", numbers);
}

