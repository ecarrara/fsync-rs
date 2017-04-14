#![feature(slice_patterns)]
extern crate fsync;

use std::process;
use std::env;
use std::iter::Iterator;

pub fn main () {
    let args : Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <source> <target>", args[0]);
        process::exit(1);
    }

    let (source, target) = (&args[1], &args[2]);

    fsync::sync(source, target).unwrap();
}
