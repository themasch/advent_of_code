use std::str::FromStr;

const COLUMNS: usize = 1000;
const ROWS: usize = 1000;

const INPUT: &'static str = include_str!("../../input/2018/day03.txt");

#[derive(Debug)]
struct Claim {
    id: usize,
    pos: (u16, u16),
    size: (u16, u16),
}

impl Claim {
    fn end(&self) -> (u16, u16) {
        (self.pos.0 + self.size.0, self.pos.1 + self.size.1)
    }

    fn add_to_field(&self, field: &mut [u8]) {
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let offset = ((self.pos.1 + y) as usize * COLUMNS + (x + self.pos.0) as usize);
                let value = field[offset] + 1;
                field[offset] = value;
            }
        }
    }

    fn overlaps(&self, other: &Claim) -> bool {
        if self.pos.0 >= other.end().0 || other.pos.0 >= self.end().0 {
            return false;
        }

        if self.pos.1 >= other.end().1 || other.pos.1 >= self.end().1 {
            return false;
        }

        return true;
    }
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let at_pos = input.find('@').expect("find @");
        let com_pos = input.find(',').expect("find ,");
        let col_pos = input.find(':').expect("find :");
        let x_pos = input.find('x').expect("find x");

        Ok(Claim {
            id: input[1..at_pos - 1].parse().expect("id"),
            pos: (
                input[at_pos + 2..com_pos].parse().expect("pos.x"),
                input[com_pos + 1..col_pos].parse().expect("pos.y"),
            ),
            size: (
                input[col_pos + 2..x_pos].parse().expect("size.x"),
                input[x_pos + 1..].parse().expect("size.y"),
            ),
        })
    }
}

fn read_input(input: &str) -> Vec<Claim> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_part1(input: &Vec<Claim>) -> usize {
    let mut field: [u8; COLUMNS * ROWS] = [0; COLUMNS * ROWS];
    input
        .iter()
        .for_each(|claim| claim.add_to_field(&mut field));

    field.iter().filter(|&&cnt| cnt >= 2).count()
}

fn solve_part2(input: &Vec<Claim>) -> usize {
    for claim in input {
        if !input
            .iter()
            .filter(|other_claim| other_claim.id != claim.id)
            .any(|other_claim| other_claim.overlaps(claim))
        {
            return claim.id;
        }
    }

    0
}

pub fn run() {
    println!("day  3, part 1: {:?}", solve_part1(&read_input(INPUT)));
    println!("day  3, part 2: {:?}", solve_part2(&read_input(INPUT)));
}

#[test]
fn test_part1() {
    assert_eq!(96569, solve_part1(&read_input(INPUT)));
}

#[test]
fn test_part2() {
    assert_eq!(1023, solve_part2(&read_input(INPUT)));
}

#[test]
fn test_overlap() {
    let c1: Claim = "#1 @ 1,3: 4x4".parse().unwrap();
    let c2: Claim = "#2 @ 3,1: 4x4".parse().unwrap();
    let c3: Claim = "#3 @ 5,5: 2x2".parse().unwrap();

    assert_eq!(true, c1.overlaps(&c2));
    assert_eq!(true, c1.overlaps(&c1));
    assert_eq!(true, c2.overlaps(&c1));
    assert_eq!(true, c2.overlaps(&c2));
    assert_eq!(false, c3.overlaps(&c1));
    assert_eq!(false, c3.overlaps(&c2));
    assert_eq!(true, c3.overlaps(&c3));
}
