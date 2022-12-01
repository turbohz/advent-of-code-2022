use aoc_driver::*;
use std::env;
mod day01;

fn main() {
	let cookie : String = env::var("COOKIE").unwrap();
	aoc_magic!(&cookie, 2022:1:1, day01::solution).unwrap();
}
