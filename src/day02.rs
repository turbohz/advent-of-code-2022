#[derive(PartialEq,Clone,Copy)]
enum Hand {
	Rock     = 1,
	Paper    = 2,
	Scissors = 3,
}

#[derive(Clone)]
enum Outcome {
	Loss = 0,
	Draw = 3,
	Win  = 6,
}

#[derive(PartialEq,Clone)]
struct Round(Hand, Hand);

fn read_hand(hand:&char) -> Hand {
	match hand {
		'A' | 'X' => Hand::Rock,
		'B' | 'Y' => Hand::Paper,
		'C' | 'Z' => Hand::Scissors,
		_ => panic!("Invalid hand: {}",hand.to_string())
	}
}

fn read_round(line: &str) -> Round {
	let chars = line.chars().collect::<Vec<_>>();
	let theirs:Hand = read_hand(chars.first().unwrap());
	let ours:Hand   = read_hand(chars.last().unwrap());
	Round(theirs,ours)
}

fn grade_round(line: &str) -> u32 {

	let round = read_round(line);

	let Round(theirs,ours) = round;

	let outcome =
		if theirs == ours {
			Outcome::Draw
		} else {

			match round {
				// match the three possible winning combinations
				Round(Hand::Paper   , Hand::Scissors) |
				Round(Hand::Rock    , Hand::Paper   ) |
				Round(Hand::Scissors, Hand::Rock    ) => Outcome::Win,

				_ => Outcome::Loss
			}
		};
	let grade = (ours as u32, outcome as u32);
	let total = grade.0.checked_add(grade.1).unwrap();
	total
}

pub fn solve(input:&str) -> String {
	let solution:u32 =
		input
		.lines()
		.filter(|l| !l.is_empty())
		.map(grade_round)
		.into_iter()
		.sum::<u32>();
	solution.to_string()
}

#[cfg(test)]
mod tests {

	use crate::day02::*;

	const INPUT: &'static str = r###"
A Y
B X
C Z
"###;

	#[test]
	fn test() {
		const EXPECTED : &str = "15";
		let actual:String = solve(INPUT);
		assert_eq!(actual, EXPECTED);
	}
}
