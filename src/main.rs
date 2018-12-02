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

    pub mod day_02 {
        const INPUT: &'static str = include_str!("../input/2018/day02.txt");

        fn parse_input(input: &str) -> Vec<&str> {
            input.lines().collect()
        }

        fn solve_part1(input: &[&str]) -> usize {
            let counts = input
                .iter()
                .map(|line| {
                    let mut counts: [u8; 26] = [0; 26];
                    line.chars().for_each(|chr| {
                        let idx = (chr as u8 - 'a' as u8) as usize;
                        counts[idx] = counts[idx] + 1;
                    });

                    counts
                })
                .map(|counts| {
                    (
                        counts.iter().find(|&&x| x == 2).is_some(),
                        counts.iter().find(|&&x| x == 3).is_some(),
                    )
                })
                .fold((0, 0), |acc, (has_twos, has_threes)| {
                    return (
                        acc.0 + if has_twos { 1 } else { 0 },
                        acc.1 + if has_threes { 1 } else { 0 },
                    );
                });

            return counts.0 * counts.1;
        }

        pub fn run() {
            println!("day  2, part 1: {:?}", solve_part1(&parse_input(INPUT)));
        }

        #[test]
        fn test_part1() {
            assert_eq!(3952, solve_part1(&parse_input(INPUT)));
        }
    }
}

fn main() {
    year_2018::day_01::run();
    year_2018::day_02::run();
}
