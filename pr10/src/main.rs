mod hackerrank;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use hackerrank::task6::diagonal_difference;

#[allow(non_snake_case)]
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    diagonal_difference(arr)
}

fn write_result(result: i32) {
    match env::var("OUTPUT_PATH") {
        Ok(path) if !path.is_empty() => {
            let mut fptr = File::create(path).unwrap();
            writeln!(&mut fptr, "{result}").ok();
        }
        _ => {
            println!("{result}");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<Vec<i32>> = (0..n)
        .map(|_| {
            stdin_iterator
                .next()
                .unwrap()
                .unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let result = diagonalDifference(&arr);

    write_result(result);
}
