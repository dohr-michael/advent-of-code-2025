use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_char(c: char) -> Self {
        match c {
            '*' => Operator::Multiply,
            _ => Operator::Add,
        }
    }

    fn initial_value(&self) -> usize {
        match self {
            Operator::Add => 0,
            Operator::Multiply => 1,
        }
    }

    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

#[derive(Debug)]
struct Operation {
    values: Vec<String>,
    operator: Operator,
}

impl Operation {
    fn parse<R: BufRead>(reader: R) -> Result<Vec<Self>> {
        let mut cached: Vec<String> = Vec::new();
        for line in reader.lines() {
            cached.push(line?);
        }
        if cached.is_empty() {
            return Ok(Vec::new());
        }
        let mut cached = cached.iter().collect::<Vec<_>>();
        let operator_line = cached.pop().unwrap();
        let mut operator_mask = vec![];
        let mut current_operator: Option<Operator> = None;
        let mut space_count: usize = 0;
        for c in operator_line.chars() {
            if c.is_whitespace() {
                space_count += 1;
                continue;
            }
            let operator = Operator::from_char(c);
            if let Some(current_operator_instance) = current_operator {
                operator_mask.push((current_operator_instance, space_count));
                current_operator = Some(operator);
                space_count = 0;
            } else {
                current_operator = Some(operator);
            }
        }
        operator_mask.push((current_operator.unwrap(), space_count + 2));
        let mut result = Vec::new();
        let mut cumulated_mask = 0;
        for (operator, mask) in operator_mask.iter() {
            let mut values = Vec::new();
            for c in cached.iter() {
                let end = cumulated_mask + mask;
                let current = if c.len() < end {
                    format!("{:width$}", &c[cumulated_mask..], width = mask)
                } else {
                    format!("{}", &c[cumulated_mask..end])
                };
                values.push(current);
            }
            result.push(Operation {
                values,
                operator: operator.clone(),
            });
            cumulated_mask += mask + 1;
        }

        Ok(result)
    }

    fn internal_compute(values: &[String], operator: &Operator) -> usize {
        values.iter().fold(operator.initial_value(), |acc, value| {
            let value = usize::from_str(value.trim()).unwrap_or_default();
            operator.apply(acc, value)
        })
    }
    fn compute(&self) -> usize {
        Self::internal_compute(&self.values, &self.operator)
    }

    fn compute_right_to_left(&self) -> usize {
        let mut values = vec![];
        for i in 0..self.values.first().unwrap().len() {
            let mut current = String::new();
            for value in self.values.iter() {
                current = format!("{}{}", current, &value[i..i + 1]);
            }
            values.push(current)
        }
        Self::internal_compute(&values, &self.operator)
    }
}

fn main() -> Result<()> {
    start_day(DAY);
    let skip_part_2 = false;
    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let operations = Operation::parse(reader)?;
        let computed = operations.iter().map(Operation::compute);
        let result = computed.sum();
        Ok(result)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

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
        let operations = Operation::parse(reader)?;
        let result = operations
            .iter()
            .map(Operation::compute_right_to_left)
            .sum();
        Ok(result)
    }
    //
    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);
    //
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
