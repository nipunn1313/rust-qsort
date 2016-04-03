mod qsort;

use std::io;
use std::io::BufRead;
use qsort::qsort;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut vect: Vec<i32> = Vec::new();

    for line in lines {
        let line = line.expect("Failed to read line");
        let val = line.parse().expect("Must enter an integer");
        vect.push(val);
    }

    for val in &vect {
        println!("before: {}", val);
    }

    qsort(&mut vect[..]);

    for val in &vect {
        println!("after: {}", val);
    }
}
