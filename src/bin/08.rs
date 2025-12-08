use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

type Coord = (usize, usize, usize);

fn distance(a: &Coord, b: &Coord) -> f64 {
    let square = |l: usize, r: usize| -> f64 { (l as f64 - r as f64).powf(2.0) };
    let square_root = |v: f64| v.powf(0.5);
    square_root(square(a.0, b.0) + square(a.1, b.1) + square(a.2, b.2))
}

#[derive(Debug)]
struct DSU {
    parents: HashMap<Coord, Coord>,
    ranks: HashMap<Coord, usize>,
}

impl DSU {
    fn new(items: Vec<Coord>) -> Self {
        Self {
            parents: items.iter().map(|c| (*c, *c)).collect(),
            ranks: items.iter().map(|c| (*c, 1)).collect(),
        }
    }

    fn find(&mut self, coord: Coord) -> Coord {
        let parent_coord = *self.parents.get(&coord).unwrap_or(&coord);
        if parent_coord == coord {
            coord
        } else {
            let parent = self.find(self.parents[&coord]);
            self.parents.insert(coord, parent);
            parent
        }
    }

    fn union(&mut self, coord1: Coord, coord2: Coord) -> bool {
        let parent1 = self.find(coord1);
        let parent2 = self.find(coord2);
        if parent1 == parent2 {
            return false;
        }
        let rank1 = self.ranks.remove(&parent1).unwrap_or(1);
        let rank2 = self.ranks.remove(&parent2).unwrap_or(1);
        if rank1 < rank2 {
            self.parents.insert(parent1, parent2);
            self.ranks.insert(parent2, rank1 + rank2);
            self.ranks.insert(parent1, 0);
        } else {
            self.parents.insert(parent2, parent1);
            self.ranks.insert(parent1, rank1 + rank2);
            self.ranks.insert(parent2, 0);
        }
        true
    }

    fn ranks(&self) -> Vec<(Coord, usize)> {
        self.ranks
            .iter()
            .filter(|&(_, v)| *v > 0)
            .map(|(c, v)| (c.clone(), v.clone()))
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .collect()
    }
}

struct Results {
    boxes: Vec<Coord>,
}

impl Results {
    fn new<R: BufRead>(reader: R) -> Result<Self> {
        let mut boxes = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts = line.split(',').collect_vec();
            if parts.len() != 3 {
                return Err(anyhow!("Expected 3 parts, got {}", parts.len()));
            }
            boxes.push((parts[0].parse()?, parts[1].parse()?, parts[2].parse()?));
        }

        Ok(Self { boxes })
    }

    fn jonctions(&self) -> Vec<(&Coord, &Coord, f64)> {
        let mut jonctions = Vec::new();
        let size = self.boxes.len();
        for (idx, l) in self.boxes.iter().enumerate() {
            for r_i in idx + 1..size {
                let r = &self.boxes[r_i];
                if l == r {
                    continue;
                }
                jonctions.push((l, r, distance(l, r)));
            }
        }
        jonctions.sort_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));
        jonctions
    }

    fn dsu(&self, limit: Option<usize>) -> (DSU, (&Coord, &Coord)) {
        let mut dsu = DSU::new(self.boxes.iter().map(|b| b.clone()).collect_vec());
        let jonctions = self.jonctions();
        let mut count = 0;
        let limit = limit.unwrap_or(jonctions.len());
        let mut coord_pairs = (jonctions[0].0, jonctions[0].1);
        for &(l, r, _) in &jonctions {
            coord_pairs = (&l, &r);
            if count >= limit {
                continue;
            }
            dsu.union(l.clone(), r.clone());
            let parent = dsu.find(l.clone());
            if dsu.ranks[&parent] == dsu.ranks.len() {
                break;
            }
            count += 1;
        }

        (dsu, coord_pairs)
    }
}

fn main() -> Result<()> {
    start_day(DAY);
    let skip_part_2 = false;
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, limit: Option<usize>) -> Result<usize> {
        let results = Results::new(reader)?;
        let (dsu, _) = results.dsu(limit);
        let ranks = dsu.ranks();
        let result = ranks
            .iter()
            .map(|c| c.1)
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, v| acc * v);
        Ok(result)
    }

    assert_eq!(40, part1(BufReader::new(TEST.as_bytes()), Some(10))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, Some(1000))?);
    println!("Result = {}", result);
    //endregion
    if skip_part_2 {
        return Ok(());
    }
    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let results = Results::new(reader)?;
        let (_, latest) = results.dsu(None);
        Ok(latest.0.0 * latest.1.0)
    }

    assert_eq!(25272, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
