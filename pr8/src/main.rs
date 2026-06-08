mod hackerrank;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use hackerrank::task6::migratory_birds;

#[allow(non_snake_case)]
fn migratoryBirds(arr: &[i32]) -> i32 {
    migratory_birds(arr)
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

    let arr: Vec<i32> = stdin_iterator
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

    let result = migratoryBirds(&arr[..n as usize]);

    write_result(result);
}
