use std::{cmp::max, io::BufRead};

use aoc::{execute, get_reader, Result};

#[derive(Debug)]
struct Draw(u32, u32, u32);

fn tokenize(values: &str) -> Vec<Draw> {
    values
        .split(';')
        .map(|v| v.trim())
        .map(|draw_str| {
            draw_str
                .split(',')
                .map(|v| v.trim())
                .fold(Draw(0, 0, 0), |draw, x| {
                    let q: Vec<&str> = x.split(' ').map(|v| v.trim()).collect();
                    if q.len() == 2 {
                        let n = q[0].parse::<u32>().unwrap();
                        match q[1] {
                            "red" => Draw(n, draw.1, draw.2),
                            "green" => Draw(draw.0, n, draw.2),
                            "blue" => Draw(draw.0, draw.1, n),
                            _ => draw,
                        }
                    } else {
                        draw
                    }
                })
        })
        .collect()
}

fn part1() -> Result<u32> {
    let mut reader = get_reader();

    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let mut sum = 0;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let id_and_values: Vec<&str> = buffer.split(':').map(|v| v.trim()).collect();
        if let [game_id_str, values] = id_and_values[..] {
            let xs: Vec<&str> = game_id_str.split(' ').map(|v| v.trim()).collect();
            let game_id = if xs.len() == 2 {
                xs[1].parse::<u32>().unwrap()
            } else {
                0
            };
            let draws = tokenize(values);
            if draws
                .iter()
                .all(|d| d.0 <= RED && d.1 <= GREEN && d.2 <= BLUE)
            {
                sum += game_id;
            }
        }
    }

    Ok(sum)
}

fn part2() -> Result<u32> {
    let mut reader = get_reader();

    let mut sum = 0;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let id_and_values: Vec<&str> = buffer.split(':').map(|v| v.trim()).collect();
        if let [_, values] = id_and_values[..] {
            let draws = tokenize(values);
            let min_cubes = draws.iter().fold(Draw(0, 0, 0), |acc, d| {
                Draw(max(acc.0, d.0), max(acc.1, d.1), max(acc.2, d.2))
            });
            sum += min_cubes.0 * min_cubes.1 * min_cubes.2;
        }
    }

    Ok(sum)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
