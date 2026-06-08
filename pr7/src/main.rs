mod hackerrank;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use hackerrank::task5::breaking_records;

#[allow(non_snake_case)]
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let (max_breaks, min_breaks) = breaking_records(scores);
    vec![max_breaks, min_breaks]
}

fn write_result(max_breaks: i32, min_breaks: i32) {
    match env::var("OUTPUT_PATH") {
        Ok(path) if !path.is_empty() => {
            let mut fptr = File::create(path).unwrap();
            writeln!(&mut fptr, "{max_breaks} {min_breaks}").ok();
        }
        _ => {
            println!("{max_breaks} {min_breaks}");
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

    let scores: Vec<i32> = stdin_iterator
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

    let result = breakingRecords(&scores[..n as usize]);

    write_result(result[0], result[1]);
}
