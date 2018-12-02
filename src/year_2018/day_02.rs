const INPUT: &'static str = include_str!("../../input/2018/day02.txt");

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn solve_part1(input: &[&str]) -> usize {
    let counts = input
        .iter()
        .map(|line| {
            line.chars().fold([0; 26], |mut counts, chr| {
                let idx = (chr as u8 - 'a' as u8) as usize;
                counts[idx] = counts[idx] + 1;
                counts
            })
        })
        .map(|counts| {
            (
                if counts.iter().any(|&x| x == 2) { 1 } else { 0 },
                if counts.iter().any(|&x| x == 3) { 1 } else { 0 },
            )
        })
        .fold((0, 0), |acc, (twos, threes)| {
            return (acc.0 + twos, acc.1 + threes);
        });

    return counts.0 * counts.1;
}

use itertools::Itertools;

fn solve_part2(input: &[&str]) -> String {
    input
        .iter()
        .cartesian_product(input.iter())
        .filter(|(left, right)| {
            left.chars()
                .zip(right.chars())
                .filter(|(lchr, rchr)| lchr != rchr)
                .take(2)
                .count()
                == 1
        })
        .map(|(left, right)| {
            left.chars()
                .zip(right.chars())
                .filter(|(lchr, rchr)| lchr == rchr)
                .map(|(lchr, _)| lchr)
                .collect::<String>()
        })
        .nth(1)
        .unwrap()
}

pub fn run() {
    println!("day  2, part 1: {:?}", solve_part1(&parse_input(INPUT)));
    println!("day  2, part 2: {:?}", solve_part2(&parse_input(INPUT)));
}

#[test]
fn test_part1() {
    assert_eq!(3952, solve_part1(&parse_input(INPUT)));
}

#[test]
fn test_part2() {
    assert_eq!(
        "vtnikorkulbfejvyznqgdxpaw",
        solve_part2(&parse_input(INPUT))
    );
}
