use std::{io::{self, BufRead}, error, collections::{HashSet, HashMap}};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

fn gear_ratios<R>(mut reader: R) -> Result<(u32, u32)>
where
    R: BufRead
{
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
                    let val = String::from_utf8_lossy(&bytes[start_index..i]).parse::<u32>().unwrap();
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
            let val = String::from_utf8_lossy(&bytes[start_index..]).parse::<u32>().unwrap();
            numbers.push((line_index, start_index, bytes.len() - start_index, val));
        }

        line_index += 1;
    }

    let sum_part1 = numbers.iter().filter(|(y, x, w, _)|
        get_adjacent_items(*x, *y, *w, line_index)
            .iter()
            .find(|adr| symbols.contains(&adr))
            .is_some()
    ).map(|(_, _, _, val)| val).sum();

    numbers.iter().for_each(|(y, x, w, num)| {
        get_adjacent_items(*x, *y, *w, line_index).iter().for_each(|adr| {
            if let Some(u) = gears.get_mut(adr) {
                u.push(*num);
            }
        });
    });

    let sum_part2 = gears.iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    Ok((sum_part1, sum_part2))
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let (part1, part2) = gear_ratios(input)?;
    println!("PART1: {}", part1);
    println!("PART2: {}", part2);

    Ok(())
}
