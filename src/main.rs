mod year_2018 {
    pub mod day_01 {
        use std::collections::HashSet;

        const INPUT: &str = include_str!("../input/2018/day01.txt");

        fn parse_input(input: &str) -> Vec<i32> {
            input
                .lines()
                .map(|line| {
                    let val: i32 = line[1..].parse().unwrap();
                    if &line[0..1] == "-" {
                        -val
                    } else {
                        val
                    }
                })
                .collect()
        }

        fn solve_part1(input: &[i32]) -> i32 {
            input.iter().sum()
        }

        /// finds the first intermediate result that appears twice
        fn solve_part2(input: &[i32]) -> i32 {
            let mut found = HashSet::new();
            let mut accu = 0;

            loop {
                for number in input {
                    accu = accu + *number;
                    if found.contains(&accu) {
                        return accu;
                    };

                    found.insert(accu);
                }
            }
        }

        pub fn run() {
            println!("day  1, part 1: {:?}", solve_part1(&parse_input(INPUT)));
            println!("day  1, part 2: {:?}", solve_part2(&parse_input(INPUT)));
        }

        #[cfg(test)]
        mod test {
            use super::*;
            #[test]
            pub fn test_part1() {
                assert_eq!(590, solve_part1(&parse_input(INPUT)));
            }

            #[test]
            pub fn test_part2() {
                assert_eq!(83445, solve_part2(&parse_input(INPUT)));
            }

        }
    }
}

fn main() {
    year_2018::day_01::run();
}
