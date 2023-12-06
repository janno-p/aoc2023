use std::{cmp::max, io::BufRead};

use aoc::{execute, get_reader, Result};

fn get_values(buffer: &str) -> Vec<u64> {
    buffer
        .trim()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|v| match v {
            "" => None,
            _ => v.parse::<u64>().ok(),
        })
        .collect()
}

fn get_value_fixed(buffer: &str) -> u64 {
    buffer
        .trim()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|v| !v.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn number_of_ways(t: u64, d: u64) -> u64 {
    let q = ((t * t - 4 * d) as f64).sqrt();
    let mut min_v = (((t as f64) - q) / 2f64).floor() as u64;
    if min_v * (t - min_v) <= d {
        min_v += 1;
    }
    let mut max_v = (((t as f64) + q) / 2f64).ceil() as u64;
    if max_v * (t - max_v) <= d {
        max_v -= 1;
    }
    max(max_v - min_v + 1, 0)
}

fn part1() -> Result<u64> {
    let mut reader = get_reader();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let time_values = get_values(&buffer);

    buffer.clear();
    reader.read_line(&mut buffer)?;
    let distance_values = get_values(&buffer);

    let result = time_values
        .iter()
        .zip(distance_values)
        .map(|(t, d)| number_of_ways(*t, d))
        .reduce(|acc, e| acc * e)
        .unwrap_or(0);

    Ok(result as u64)
}

fn part2() -> Result<u64> {
    let mut reader = get_reader();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let time_value = get_value_fixed(&buffer);

    buffer.clear();
    reader.read_line(&mut buffer)?;
    let distance_value = get_value_fixed(&buffer);

    let result = number_of_ways(time_value, distance_value);

    Ok(result)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
