// https://adventofcode.com/2022/day/1

fn aggregate(acc: Vec<u32>, v:&str) -> Vec<u32> {
	let mut result = acc.clone();
	let last = result.len()-1;
	if v.is_empty() {
		result.push(0);
	} else {
		let calories: u32  = v.parse().unwrap();
		result[last] = calories + acc[last];
	}
	result
}

pub fn solution(i: &str) -> String {
	let lines = i.lines();
	let acc:Vec<u32> = vec![0];
	// aggregate
	let stock = lines.fold(acc, aggregate);
	stock.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
	use crate::day01::solution;
	const EXPECTED : &str = "24000";
	const INPUT : &'static str = r###"
1000
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
		let actual = solution(INPUT);
		assert_eq!(actual, EXPECTED);
	}
}
