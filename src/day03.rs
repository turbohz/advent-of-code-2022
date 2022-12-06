use std::fs;

const LOWERCASE_OFFSET:u8 = 'a' as u8;
const UPPERCASE_OFFSET:u8 = 'A' as u8;

fn compute_priority(i:char) -> u8 {
	if i.is_lowercase() {
		01 + (i as u8) - LOWERCASE_OFFSET
	} else if i.is_uppercase() {
		27 + (i as u8) - UPPERCASE_OFFSET
	} else {
		panic!("Expecting a letter, but got '{}'",i)
	}
}

fn find_duplicate_item(compartment:(&str,&str)) -> char {
	let mut dupe:Option<char> = None;
	let (a,b) = compartment;
	a.chars().for_each(|c| if b.contains(c) { dupe = Some(c) });
	dupe.unwrap()
}

pub fn solve(input:&str) -> u32 {
	input
	.lines()
	.map(|l| l.split_at(l.len()/2))
	.map(find_duplicate_item)
	.map(compute_priority)
	.fold(0 as u32, |acc, p| acc.checked_add(p as u32).unwrap())
}

#[cfg(test)]
mod tests {

	use crate::day03::*;

	const EXPECTED:u32 = 157;
	const INPUT: &'static str =
r###"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"###;

	#[test]
	#[should_panic]
	fn test_invalid_item() {
		compute_priority('1');
		compute_priority('$');
		()
	}
	#[test]
	fn test_priorities() {
		assert_eq!(compute_priority('a'),01);
		assert_eq!(compute_priority('A'),27);
	}
	#[test]
	fn test() {
		let actual:u32 = solve(INPUT);
		assert_eq!(actual,EXPECTED);
	}
}
