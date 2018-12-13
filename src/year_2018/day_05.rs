const INPUT: &'static str = include_str!("../../input/2018/day05.txt");

use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Polymer {
    Uppercase(char),
    Lowercase(char)
}

fn read_input(input: &str) -> Vec<Polymer> {
    input
        .chars()
        .map( | chr | {
            if chr.is_uppercase() {
                Polymer::Uppercase(chr.to_ascii_lowercase())
            } else {
                Polymer::Lowercase(chr)
            }
        })
        .collect()
}

trait ChemicalReaction {
    fn reduce(&self) -> Self;

    fn filter_type(&self, chr: char) -> Self;

    fn fully_reduce(&self) -> Self;
}

impl ChemicalReaction for Vec<Polymer> {
    // idea for performance improvement: backtracking reducer, after eliminating two units, go back one element and check resume. should work in one pass.
    fn reduce(&self) -> Vec<Polymer> {
        let mut reduced = Vec::new();
        let (_, rest) = self.iter()
            .skip(1)
            .fold((&mut reduced, self.get(0)), | (acc, tail), element | {
                match (tail, element) {
                    (Some(Polymer::Uppercase(x)), Polymer::Lowercase(y)) if x == y => {
                        (acc, None)
                    },
                    (Some(Polymer::Lowercase(x)), Polymer::Uppercase(y)) if x == y => {
                        (acc, None)
                    },
                    (None, _) => {
                        (acc, Some(element))
                    }
                    (Some(curr), _) => {
                        acc.push(curr.clone());
                        (acc, Some(element))
                    }
                }
            });

        if let Some(x) = rest {
            reduced.push(x.clone());
        }

        return reduced;
    }

    fn filter_type(&self, chr: char) -> Vec<Polymer> {
        self.iter().filter( | &&poly | match poly {
            Polymer::Lowercase(x) if x == chr => false,
            Polymer::Uppercase(x) if x == chr => false,
            _ => true
        }).map(| c | c.clone()).collect()
    }

    fn fully_reduce(&self) -> Vec<Polymer> {
        let mut old_count = self.len();
        let mut reduced = self.reduce();
        while reduced.len() != old_count {
            old_count = reduced.len();
            reduced = reduced.reduce();
        }

        reduced
    }
}

fn solve_part1(polymers: &Vec<Polymer>) -> usize {
    let reduced = polymers.fully_reduce();

    return reduced.len();
}

fn solve_part2(polymers: &Vec<Polymer>) -> usize {
    // this is an unessesary optimization since all 26 chars are part of the input.
    // but we do not have to iterate over a range of u8 and cast them to char this way
    let mut  chars: Vec<char> = polymers.iter().map( | p | match p {
        Polymer::Lowercase(x) => *x,
        Polymer::Uppercase(x) => *x
    }).collect();
    chars.sort();
    chars.dedup();

    chars.into_par_iter()
        .map( | chr: char | polymers.filter_type(chr).fully_reduce().len())
        .min()
        .unwrap()
}


pub fn run() {
    println!("day  5, part 1: {}", solve_part1(&read_input(INPUT)));
    println!("day  5, part 2: {}", solve_part2(&read_input(INPUT)));
}

#[test]
fn test_simple_input() {
    assert_eq!(10, solve_part1(&read_input("dabAcCaCBAcCcaDA")));
}
#[test]
fn test_full_input() {
    assert_eq!(11264, solve_part1(&read_input(INPUT)));
}
#[test]
fn test_simple_input2() {
    assert_eq!(4, solve_part2(&read_input("dabAcCaCBAcCcaDA")));
}
#[test]
fn test_full_input2() {
    assert_eq!(4552, solve_part2(&read_input(INPUT)));
}