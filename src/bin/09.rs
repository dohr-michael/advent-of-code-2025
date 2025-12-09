use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::{max, min};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct Rectangle {
    points: (Coord, Coord),
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    pixels: i64,
}

impl Rectangle {
    fn new(points: (Coord, Coord)) -> Self {
        let x_min = min(points.0.0, points.1.0);
        let x_max = max(points.0.0, points.1.0);
        let y_min = min(points.0.1, points.1.1);
        let y_max = max(points.0.1, points.1.1);
        Self {
            pixels: Self::compute_pixels(x_min, x_max, y_min, y_max),
            points,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn compute_pixels(left: usize, right: usize, bottom: usize, top: usize) -> i64 {
        ((right as i64 - left as i64).abs() + 1) * ((top as i64 - bottom as i64).abs() + 1)
    }
}

struct Grid {
    points: Vec<Coord>,
}

impl Grid {
    fn new<R: BufRead>(reader: R) -> Result<Self> {
        let mut points = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let current = line.split(',').collect_vec();
            let [x, y] = current[..] else {
                return Err(anyhow!("Invalid input: {}", line));
            };
            let x: usize = x.parse()?;
            let y: usize = y.parse()?;
            points.push((x, y));
        }

        Ok(Self { points })
    }

    fn part_1(&self) -> Vec<Rectangle> {
        let mut result = Vec::new();
        for (i, p1) in self.points.iter().enumerate() {
            for p2 in self.points.iter().skip(i + 1) {
                result.push(Rectangle::new((*p1, *p2)));
            }
        }
        result.sort_by(|r1, r2| r1.pixels.partial_cmp(&r2.pixels).unwrap());
        result
    }

    fn part_2(&self) -> Vec<Rectangle> {
        let rects = self.part_1();
        let fold = |mut acc: Vec<Rectangle>, r: &Rectangle| {
            let mut is_cut = false;
            for (i, p1) in self.points.iter().enumerate() {
                let p2 = &self.points[(i + 1) % self.points.len()];
                let x3 = min(p1.0, p2.0);
                let x4 = max(p1.0, p2.0);
                let y3 = min(p1.1, p2.1);
                let y4 = max(p1.1, p2.1);
                if r.x_min < x4 && r.x_max > x3 && r.y_min < y4 && y3 < r.y_max {
                    is_cut = true;
                    break;
                }
            }
            if !is_cut {
                acc.push(r.clone());
            }
            acc
        };

        rects.iter().fold(Vec::new(), fold)
    }
}

fn main() -> Result<()> {
    start_day(DAY);
    let skip_part_2 = false;
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let grid = Grid::new(reader)?;
        let rectangles = grid.part_1();
        let largest_area = rectangles.last().unwrap().pixels;
        Ok(largest_area)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion
    if skip_part_2 {
        return Ok(());
    }
    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i64> {
        let grid = Grid::new(reader)?;
        let rectangles = grid.part_2();
        let largest_area = rectangles.last().unwrap().pixels;
        Ok(largest_area)
    }

    assert_eq!(24, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
