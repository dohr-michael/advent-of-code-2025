use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn count(&self) -> usize {
        self.end - self.start + 1
    }
    fn contains(&self, x: usize) -> bool {
        self.start <= x && x <= self.end
    }

    fn included(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.start && other.start <= self.end
            || self.end <= other.end && other.end <= self.start
    }
}

#[derive(Debug)]
struct ListOfRanges {
    ranges: Vec<Range>,
}

impl ListOfRanges {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn append(&mut self, other: &Range) {
        if self.ranges.is_empty() {
            self.ranges.push(other.clone());
            return;
        } else if self.ranges.iter().any(|r| r.included(other)) {
            return;
        }
        let mut copy = self.ranges.clone();
        let mut changed = false;

        for (index, r) in self.ranges.iter().enumerate() {
            if r.included(other) {
                continue;
            }
            if r.overlaps(other) {
                changed = true;
                copy[index] = Range {
                    start: min(r.start, other.start),
                    end: max(r.end, other.end),
                };
            }
        }
        if !changed {
            copy.push(other.clone());
        }
        self.ranges = copy;
    }
}

struct Fridge {
    fresh: Vec<Range>,
    available: Vec<usize>,
}

impl Fridge {
    fn new<R: BufRead>(reader: R) -> Result<Self> {
        let mut fresh: Vec<Range> = Vec::new();
        let mut available: Vec<usize> = Vec::new();
        let mut is_fresh = true;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                is_fresh = !is_fresh;
                continue;
            }
            if is_fresh {
                let (start, end) = line
                    .split_once('-')
                    .ok_or_else(|| anyhow!("Invalid range format: {}", line))?;
                let start = start.parse::<usize>()?;
                let end = end.parse::<usize>()?;
                fresh.push(Range { start, end });
            } else {
                available.push(line.parse::<usize>()?);
            }
        }
        fresh.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(Self { fresh, available })
    }

    fn fresh_available(&self) -> Vec<usize> {
        self.available
            .iter()
            .copied()
            .filter(|&x| self.fresh.iter().any(|r| r.contains(x)))
            .collect()
    }

    fn really_fresh(&self) -> usize {
        let mut result = ListOfRanges::new(vec![]);
        // loop {
        for r in &self.fresh {
            result.append(r);
        }
        // }
        result.ranges.iter().map(|r| r.count()).sum()
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let fridge = Fridge::new(reader)?;
        let fresh = fridge.fresh_available();
        Ok(fresh.len())
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let fridge = Fridge::new(reader)?;
        let fresh = fridge.really_fresh();
        Ok(fresh)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
