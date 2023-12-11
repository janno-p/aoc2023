use std::{
    cmp::{max, min},
    collections::BTreeSet,
    io::BufRead,
};

use aoc::{execute, get_reader, Result};

fn calculate_distances(multiplier: usize) -> Result<u64> {
    let mut reader = get_reader();
    let mut buffer = String::new();

    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut empty_rows = BTreeSet::<usize>::new();
    let mut empty_cols = BTreeSet::<usize>::new();

    let mut n = 0;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let bytes = buffer.trim().as_bytes();

        if n == 0 {
            empty_cols = (0..bytes.len()).collect();
        }

        let is_empty = bytes.iter().enumerate().fold(true, |is_empty, (i, &b)| {
            if b == b'#' {
                galaxies.push((i, n));
                empty_cols.remove(&i);
                false
            } else {
                is_empty
            }
        });

        if is_empty {
            empty_rows.insert(n);
        }

        n += 1;
    }

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (max_x, min_x) = (
                max(galaxies[i].0, galaxies[j].0),
                min(galaxies[i].0, galaxies[j].0),
            );
            let (max_y, min_y) = (
                max(galaxies[i].1, galaxies[j].1),
                min(galaxies[i].1, galaxies[j].1),
            );
            let dx = max_x - min_x;
            let dy = max_y - min_y;
            sum += (dx
                + dy
                + ((multiplier - 1)
                    * empty_cols
                        .iter()
                        .skip_while(|&&x| x < min_x)
                        .take_while(|&&x| x < max_x)
                        .count())
                + ((multiplier - 1)
                    * empty_rows
                        .iter()
                        .skip_while(|&&y| y < min_y)
                        .take_while(|&&y| y < max_y)
                        .count())) as u64;
        }
    }

    Ok(sum)
}

fn part1() -> Result<u64> {
    calculate_distances(2)
}

fn part2() -> Result<u64> {
    calculate_distances(1000000)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
