use aoc_driver::*;
use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
	let cookie : String = env::var("COOKIE").unwrap();
	aoc_magic!(&cookie, 2022:1:1, day01::solve).unwrap();
	aoc_magic!(&cookie, 2022:2:1, day02::solve).unwrap();
	aoc_magic!(&cookie, 2022:3:1, day03::solve).unwrap();
}
