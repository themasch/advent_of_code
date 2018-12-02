const INPUT: &'static str = include_str!("../../input/2018/day02.txt");

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
