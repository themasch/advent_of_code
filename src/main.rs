mod year_2018 {
	pub mod day_01 {
		const INPUT: &str = include_str!("../input/2018/day01.txt");

		fn parse_input(input: &str) -> Vec<i32> {
			input.lines()
				.map(| line | {
					let val: i32 = line[1..].parse().unwrap();
					if &line[0..1] == "-" {
						-val
					} else {
						val
					}
				})
				.collect()
		}

		fn solve(input: &[i32]) -> i32 {
			input.iter().sum()
		}

		pub fn run() {
			println!("day  1, part 1: {:?}", solve(&parse_input(INPUT)));
		}

		#[cfg(test)]
		mod test {
			use super::*;
			#[test]
			pub fn test_part1() {
				assert_eq!(590, solve(&parse_input(INPUT)));
			}
		}
	}
}

fn main() {
    year_2018::day_01::run();
}

