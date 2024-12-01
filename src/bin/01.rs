use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (mut col1, mut col2) = reader.lines().fold((vec![], vec![]), |(mut col1, mut col2), str| {
            let bind = str.unwrap();
            let nums = bind.split("   ").collect::<Vec<&str>>();
            col1.push(nums[0].parse::<i32>().unwrap());
            col2.push(nums[1].parse::<i32>().unwrap());
            return (col1, col2);
        });

        Ok(col1.iter().sorted().zip(col2.iter().sorted()).fold(0, |val, (a, b)| {val + (a - b).abs() as usize}))
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (mut col1, mut col2) = reader.lines().fold((vec![], vec![]), |(mut col1, mut col2), str| {
            let bind = str.unwrap();
            let nums = bind.split("   ").collect::<Vec<&str>>();
            col1.push(nums[0].parse::<i32>().unwrap());
            col2.push(nums[1].parse::<i32>().unwrap());
            return (col1, col2);
        });

        let mut map = col2.iter().fold(HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });

        Ok(col1.iter().fold(0, |acc, elem| {
            acc + (elem * *map.entry(elem).or_insert(0)) as usize
        }))
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
