mod task1;

fn main() {
    let result = task1::staircase(4);

    for line in result {
        println!("{line}");
    }
}