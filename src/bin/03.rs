use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

struct Battery {
    power_level: usize,
}

impl Battery {
    fn find_power_level(chain: &str, nb_battery: usize) -> usize {
        let mut result = vec![0; nb_battery];
        result.fill(0);
        let len = chain.len();
        for i in 0..len {
            let sub_chain = usize::from_str(&chain[i..i + 1]).unwrap();
            let from = if i < len - nb_battery {
                0
            } else {
                i - (len - nb_battery)
            };
            for j in from..nb_battery {
                if sub_chain > result[j] {
                    result[j] = sub_chain;
                    for k in j + 1..nb_battery {
                        result[k] = 0;
                    }
                    break;
                }
            }
        }
        result
            .iter()
            .fold("".to_string(), |acc, v| format!("{acc}{v}"))
            .parse::<usize>()
            .unwrap()
    }

    fn new(chain: &str, nb_battery: usize) -> Self {
        let power_level = Self::find_power_level(chain, nb_battery);
        Self { power_level }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|l| Ok(Battery::new(&l?, 2)))
            .collect::<Result<Vec<_>, _>>()?;
        let answer = answer.iter().map(|b| b.power_level).sum::<usize>();
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    //
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|l| Ok(Battery::new(&l?, 12)))
            .collect::<Result<Vec<_>, _>>()?;
        let answer = answer.iter().map(|b| b.power_level).sum::<usize>();
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
