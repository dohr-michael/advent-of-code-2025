use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

#[derive(Debug)]
struct Rotate {
    direction: i32,
    steps: usize,
}

impl Rotate {
    fn list<R: BufRead>(reader: R) -> Result<Vec<Self>> {
        reader.lines().map(|line| Self::from_str(&line?)).collect()
    }
    fn rotate(&self, current: usize) -> (usize, usize) {
        let natural_turn = self.steps / 100;
        let steps = self.steps % 100;

        let delta = self.direction * steps as i32;
        let computed = (current as i32) + delta;
        let local_steps = computed.abs() % 100;
        let result = if computed < 0 {
            100 - local_steps
        } else if computed > 99 {
            local_steps
        } else {
            computed
        };

        let zero_count = match self.direction {
            1 => usize::from(computed > 100),
            -1 => usize::from(current == 0 && steps > 99 || current != 0 && steps > current),
            _ => 0,
        };
        (
            result as usize,
            natural_turn + usize::from(result == 0) + zero_count,
        )
    }
}

impl FromStr for Rotate {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let steps = usize::from_str(&s[1..])?;
        let direction = match &s[0..1] {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };
        Ok(Self { direction, steps })
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let items = Rotate::list(reader)?;
        let result = items.iter().fold((50, 0), |(pos, counts), item| {
            let current = item.rotate(pos);
            (current.0, counts + usize::from(current.0 == 0))
        });
        Ok(result.1)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let items = Rotate::list(reader)?;
        let result = items.iter().fold((50, 0), |(pos, counts), item| {
            let current = item.rotate(pos);
            (current.0, counts + current.1)
        });
        Ok(result.1)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
