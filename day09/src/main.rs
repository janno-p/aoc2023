use std::io::BufRead;

use aoc::{execute, get_reader, Result};

fn part1() -> Result<i32> {
    let mut reader = get_reader();
    let mut buffer = String::new();

    let mut sum = 0i32;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let numbers: Vec<_> = buffer
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut differences = Vec::new();
        for number in numbers {
            differences.push(number);
            let mut last = *differences.last().unwrap();
            for difference in differences.iter_mut().rev().skip(1) {
                last -= *difference;
                *difference = last;
            }
        }

        sum += differences.iter().sum::<i32>();
    }

    Ok(sum)
}

fn part2() -> Result<i32> {
    let mut reader = get_reader();
    let mut buffer = String::new();

    let mut sum = 0i32;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let numbers: Vec<_> = buffer
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut differences = Vec::new();
        for &number in numbers.iter().rev() {
            differences.push(number);
            let mut last = *differences.last().unwrap();
            for difference in differences.iter_mut().rev().skip(1) {
                last = *difference - last;
                *difference = last;
            }
        }

        sum += differences.iter().fold(0, |acc, &x| x - acc);
    }

    Ok(sum)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
