use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

#[derive(Debug, Clone)]
struct Range {
    left: usize,
    right: usize,
}

impl FromStr for Range {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split('-');
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        Ok(Self {
            left: usize::from_str(left)?,
            right: usize::from_str(right)?,
        })
    }
}
impl Range {
    fn read_all(reader: impl BufRead) -> Result<Vec<Self>> {
        let items: Result<Vec<Vec<Self>>> = reader
            .lines()
            .map(|line| {
                line?
                    .split(',')
                    .map(Self::from_str)
                    .collect::<Result<Vec<Self>, Error>>()
            })
            .collect();
        Ok(items?.iter().flatten().cloned().collect::<Vec<_>>())
    }

    fn is_repeated(value: usize, count: usize) -> bool {
        let as_str = value.to_string();
        if as_str.len() % count != 0 {
            return false;
        }
        let parts = as_str.len() / count;
        let mut values = vec![];
        for i in 0..count {
            values.push(&as_str[i * parts..(i + 1) * parts]);
        }
        values.iter().all(|v| v == &values[0])
    }

    fn find_as_twice_sequence(&self) -> Vec<usize> {
        let items = self.left..=self.right;
        items.filter(|&x| Self::is_repeated(x, 2)).collect()
    }

    fn find_as_any_sequence(&self) -> Vec<usize> {
        let items = self.left..=self.right;
        items
            .filter(|&x| {
                for i in 2..=x.to_string().len() {
                    if Self::is_repeated(x, i) {
                        return true;
                    }
                }
                return false;
            })
            .collect()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let ranges = Range::read_all(reader)?;
        let invalids = ranges
            .iter()
            .map(|r| r.find_as_twice_sequence())
            .collect::<Vec<_>>();
        Ok(invalids.iter().flatten().sum())
    }

    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let ranges = Range::read_all(reader)?;
        let invalids = ranges
            .iter()
            .map(|r| r.find_as_any_sequence())
            .collect::<Vec<_>>();

        Ok(invalids.iter().flatten().sum())
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
