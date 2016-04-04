extern crate getopts;
extern crate iron;

mod ironserver;
mod qsort;

use getopts::Options;
use std::env;
use std::io;
use std::io::BufRead;

use ironserver::ironserver;
use qsort::qsort;

fn halp(opts: Options, prog_name: &String) {
    println!("{}", opts.usage(&opts.short_usage(prog_name)));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = args[0].clone();  // Grab prog name before args are consumed

    let mut opts = Options::new();
    opts.optflag("", "stdin", "Read values from stdin to sort");
    opts.optflag("h", "help", "Print usage");
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(_) => {
            return halp(opts, &prog_name);
        }
    };
    if matches.opt_present("h") {
        return halp(opts, &prog_name);
    }

    if matches.opt_present("stdin") {
        sort_from_stdin();
    } else {
        ironserver();
    }
}

fn sort_from_stdin() {
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
