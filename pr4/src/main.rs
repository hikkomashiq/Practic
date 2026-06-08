mod hackerrank;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use hackerrank::task2::count_apples_and_oranges;

#[allow(non_snake_case)]
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let (apple_count, orange_count) = count_apples_and_oranges(s, t, a, b, apples, oranges);

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    writeln!(&mut fptr, "{apple_count}").ok();
    writeln!(&mut fptr, "{orange_count}").ok();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let third_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();
    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    let apples: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let oranges: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples[..m as usize], &oranges[..n as usize]);
}
