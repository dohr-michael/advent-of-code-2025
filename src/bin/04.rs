use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

const DAY: &str = "04"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<usize>>,
}

impl Grid {
    fn new<R: BufRead>(reader: R) -> Result<Self> {
        let rows = reader.lines().try_fold(Vec::new(), |mut rows, line| {
            let line = line?;
            let mut cols = vec![];
            for i in 0..line.len() {
                cols.push(if line[i..=i].eq("@") { 1 } else { 0 });
            }
            rows.push(cols);
            Ok(rows)
        })?;

        Ok(Self { rows })
    }

    fn nb_adjacent(&self, row: usize, col: usize) -> usize {
        let cols = self.rows.get(row).unwrap();
        let min_row = if row == 0 { 0 } else { row - 1 };
        let max_row = min(self.rows.len() - 1, row + 1);
        let min_cols = if col == 0 { 0 } else { col - 1 };
        let max_cols = min(cols.len() - 1, col + 1);
        let mut result = 0;
        for r in min_row..=max_row {
            for c in min_cols..=max_cols {
                if r == row && c == col {
                    continue;
                }
                result += self.rows[r][c];
            }
        }
        result
    }

    fn forklift(&self) -> Vec<Vec<usize>> {
        let mut result = vec![];
        for (row_number, row) in self.rows.iter().enumerate() {
            let mut new_row = vec![];
            for (col_number, col) in row.iter().enumerate() {
                if *col == 0 {
                    new_row.push(0);
                } else {
                    let adjacent = self.nb_adjacent(row_number, col_number);
                    new_row.push(if adjacent < 4 { 1 } else { 0 })
                }
            }
            result.push(new_row);
        }
        result
    }

    fn removed(&mut self) -> usize {
        let mut result = self.rows.clone();
        let mut removed = 0;
        let forklifted = self.forklift();
        for (row_number, row) in self.rows.iter().enumerate() {
            for (col_number, _col) in row.iter().enumerate() {
                if forklifted[row_number][col_number] == 1 {
                    result[row_number][col_number] = 0;
                    removed += 1;
                }
            }
        }
        self.rows = result;
        removed
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = Grid::new(reader)?;
        let forklifted = grid.forklift();

        Ok(forklifted.iter().flatten().sum())
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut grid = Grid::new(reader)?;
        let mut removed = 0;
        while let nb_removed = grid.removed()
            && nb_removed > 0
        {
            removed += nb_removed;
        }

        Ok(removed)
    }
    //
    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
