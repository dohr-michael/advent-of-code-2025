use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Start,
    Splitter,
    Beam,
    None,
}
#[derive(Debug, Clone)]
struct Beam {
    rows: Vec<Vec<Item>>,
    nb_splitting: usize,
}

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for c in row {
                write!(
                    f,
                    "{}",
                    match c {
                        Item::Start => 'S',
                        Item::Splitter => '^',
                        Item::Beam => '|',
                        Item::None => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}

impl Beam {
    fn init<R: BufRead>(reader: R) -> Result<Self> {
        let mut rows = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    'S' => Item::Start,
                    '^' => Item::Splitter,
                    _ => Item::None,
                });
            }
            rows.push(row);
        }
        Ok(Self {
            rows,
            nb_splitting: 0,
        })
    }

    fn next(&self, row: usize) -> Self {
        let prev = self.rows[row - 1].clone();
        let mut current_row = self.rows[row].clone();
        let mut nb_splitting = self.nb_splitting;
        for (idx, c) in self.rows[row].iter().enumerate() {
            let is_none = matches!(c, Item::None);
            let prev_row_needs_beam = matches!(prev.get(idx), Some(Item::Beam) | Some(Item::Start));
            if is_none && prev_row_needs_beam {
                current_row[idx] = Item::Beam;
            } else if matches!(c, Item::Splitter) && prev_row_needs_beam {
                let mut splitting = 0;
                if idx > 0 && matches!(current_row[idx - 1], Item::None) {
                    splitting += 1;
                    current_row[idx - 1] = Item::Beam;
                }
                if idx < current_row.len() - 1 && matches!(current_row[idx + 1], Item::None) {
                    splitting += 1;
                    current_row[idx + 1] = Item::Beam;
                }
                if splitting > 0 {
                    nb_splitting += 1;
                }
            }
        }
        let mut rows = self.clone();
        rows.rows[row] = current_row;
        rows.nb_splitting = nb_splitting;
        rows
    }

    fn count_paths_from_start(
        &self,
        row: usize,
        col: usize,
        mut cache: HashMap<(usize, usize), usize>,
    ) -> (usize, HashMap<(usize, usize), usize>) {
        if cache.contains_key(&(row, col)) {
            return (cache[&(row, col)], cache);
        }
        let value = &self.rows[row][col];
        let is_beam = value == &Item::Beam || value == &Item::Start;
        let is_splitter = value == &Item::Splitter;
        if !is_beam && !is_splitter {
            cache.insert((row, col), 0);
            return (0, cache);
        }
        if row == self.rows.len() - 1 {
            let value = if is_beam { 1 } else { 0 };
            cache.insert((row, col), value);
            return (value, cache);
        }
        let (value, mut cache) = if is_splitter {
            let (left, cache) = if col >= 1 {
                self.count_paths_from_start(row + 1, col - 1, cache)
            } else {
                (0, cache)
            };
            let (right, cache) = if col <= self.rows[row].len() - 1 {
                self.count_paths_from_start(row + 1, col + 1, cache)
            } else {
                (0, cache)
            };
            (left + right, cache)
        } else {
            self.count_paths_from_start(row + 1, col, cache)
        };
        cache.insert((row, col), value);
        (value, cache)
    }

    fn final_state(&self) -> Self {
        let mut result = self.clone();
        for i in 1..self.rows.len() {
            let next = result.next(i);
            result = next;
        }
        result
    }

    fn final_quantum_states(&self) -> usize {
        let final_state = self.final_state();
        let mut result = 0;
        let mut cache = HashMap::new();
        for i in 0..final_state.rows.first().unwrap().len() {
            let (value, c_cache) = final_state.count_paths_from_start(0, i, cache);
            result += value;
            cache = c_cache;
        }
        result
    }
}

fn main() -> Result<()> {
    start_day(DAY);
    let skip_part_2 = false;
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let initial_state = Beam::init(reader)?;
        let final_state = initial_state.final_state();
        Ok(final_state.nb_splitting)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion
    if skip_part_2 {
        return Ok(());
    }
    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let initial_state = Beam::init(reader)?;
        let final_states = initial_state.final_quantum_states();
        Ok(final_states)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
