use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use aoc::{execute, get_reader, Result};

fn get_adjacent_items(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut items = vec![];

    let min_x = if x == 0 { 0 } else { x - 1 };
    let max_x = x + w + 1;

    if y > 0 {
        let j = y - 1;
        items.extend((min_x..max_x).map(|dx| (j, dx)));
    }

    if y + 1 < h {
        let j = y + 1;
        items.extend((min_x..max_x).map(|dx| (j, dx)));
    }

    if x > 0 {
        items.push((y, x - 1));
    }

    items.push((y, x + w));

    items
}

fn part1() -> Result<u32> {
    let mut reader = get_reader();

    let mut numbers = Vec::<(usize, usize, usize, u32)>::new();
    let mut symbols = HashSet::<(usize, usize)>::new();

    let mut line_index = 0usize;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let mut number_start = None;

        let bytes = buffer.trim().as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if b.is_ascii_digit() {
                if number_start.is_none() {
                    number_start = Some(i);
                }
            } else {
                if let Some(start_index) = number_start {
                    let val = String::from_utf8_lossy(&bytes[start_index..i])
                        .parse::<u32>()
                        .unwrap();
                    numbers.push((line_index, start_index, i - start_index, val));
                    number_start = None;
                }
                if *b != b'.' {
                    symbols.insert((line_index, i));
                }
            }
        }

        if let Some(start_index) = number_start {
            let val = String::from_utf8_lossy(&bytes[start_index..])
                .parse::<u32>()
                .unwrap();
            numbers.push((line_index, start_index, bytes.len() - start_index, val));
        }

        line_index += 1;
    }

    let sum = numbers
        .iter()
        .filter(|(y, x, w, _)| {
            get_adjacent_items(*x, *y, *w, line_index)
                .iter()
                .any(|adr| symbols.contains(adr))
        })
        .map(|(_, _, _, val)| *val)
        .sum();

    Ok(sum)
}

fn part2() -> Result<u32> {
    let mut reader = get_reader();

    let mut numbers = Vec::<(usize, usize, usize, u32)>::new();
    let mut symbols = HashSet::<(usize, usize)>::new();
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    let mut line_index = 0usize;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let mut number_start = None;

        let bytes = buffer.trim().as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            if b.is_ascii_digit() {
                if number_start.is_none() {
                    number_start = Some(i);
                }
            } else {
                if let Some(start_index) = number_start {
                    let val = String::from_utf8_lossy(&bytes[start_index..i])
                        .parse::<u32>()
                        .unwrap();
                    numbers.push((line_index, start_index, i - start_index, val));
                    number_start = None;
                }
                if *b != b'.' {
                    symbols.insert((line_index, i));
                }
                if *b == b'*' {
                    gears.insert((line_index, i), vec![]);
                }
            }
        }

        if let Some(start_index) = number_start {
            let val = String::from_utf8_lossy(&bytes[start_index..])
                .parse::<u32>()
                .unwrap();
            numbers.push((line_index, start_index, bytes.len() - start_index, val));
        }

        line_index += 1;
    }

    numbers.iter().for_each(|(y, x, w, num)| {
        get_adjacent_items(*x, *y, *w, line_index)
            .iter()
            .for_each(|adr| {
                if let Some(u) = gears.get_mut(adr) {
                    u.push(*num);
                }
            });
    });

    let sum = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    Ok(sum)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
