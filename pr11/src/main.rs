mod hackerrank;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use hackerrank::task6::page_count;

#[allow(non_snake_case)]
fn pageCount(n: i32, p: i32) -> i32 {
    page_count(n, p)
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

    let p = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let result = pageCount(n, p);

    write_result(result);
}
