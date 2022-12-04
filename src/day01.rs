// https://adventofcode.com/2022/day/1

fn aggregate<'a>(acc: &'a mut Vec<u32>, v:&str) -> &'a mut Vec<u32> {
	let last = acc.len()-1;
	if v.is_empty() {
		acc.push(0);
	} else {
		let calories: u32  = v.parse().unwrap();
		acc[last] += calories;
	}
	acc
}

pub fn solve(i: &str) -> String {
	let lines = i.lines();
	let mut acc:Vec<u32> = vec![0];
	// aggregate
	let stock = lines.fold(&mut acc, aggregate);
	stock.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
	use crate::day01::solve;
	const EXPECTED : &str = "24000";
	const INPUT : &'static str =
r###"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"###;

	#[test]
	fn test() {
		let actual = solve(INPUT);
		assert_eq!(actual, EXPECTED);
	}
}
