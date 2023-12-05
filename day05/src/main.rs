use std::{io::{self, BufRead}, error, cmp::{max, min}, collections::{HashMap, BTreeMap}};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn read_seeds(input: &str) -> Vec<u64> {
    let values = input.split(":").skip(1).take(1).next().unwrap();
    values.trim().split(" ").map(|v| v.parse::<u64>().unwrap()).collect()
}

fn read_map<R>(reader: &mut R) -> Result<Vec<(u64, u64, u64)>>
where
    R: BufRead
{
    let mut map = vec![];

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    loop {
        buffer.clear();

        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        if buffer.trim().is_empty() {
            break;
        }

        let xs: Vec<_> = buffer.split(" ").map(|v| v.trim().parse::<u64>().unwrap()).collect();
        map.push((xs[0], xs[1], xs[2]));
    }

    Ok(map)
}

fn read_map2<R>(reader: &mut R) -> Result<BTreeMap<u64, (u64, u64)>>
where
    R: BufRead
{
    let mut map = BTreeMap::<u64, (u64, u64)>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    loop {
        buffer.clear();

        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        if buffer.trim().is_empty() {
            break;
        }

        let xs: Vec<_> = buffer.split(" ").map(|v| v.trim().parse::<u64>().unwrap()).collect();
        map.insert(xs[1], (xs[0], xs[2]));
    }

    Ok(map)
}

fn part1<R>(mut reader: R) -> Result<u64>
where
    R: BufRead
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut seeds = read_seeds(&buffer);

    reader.read_line(&mut buffer)?;

    loop {
        let map = read_map(&mut reader)?;
        if map.len() == 0 {
            break;
        }

        seeds = seeds.iter().map(|v| {
            if let Some((destination, source, _)) = map.iter().find(|(_, source, length)| *v >= *source && *v < *source + *length) {
                *destination + (*v - *source)
            } else {
                *v
            }
        }).collect();
    }

    Ok(*seeds.iter().min().unwrap())
}

fn part2<R>(mut reader: R) -> Result<u64>
where
    R: BufRead
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let seeds = read_seeds(&buffer);
    let mut seed_ranges: Vec<_> = seeds.chunks(2).map(|x| (x[0], x[1])).collect();

    reader.read_line(&mut buffer)?;

    loop {
        let map = read_map2(&mut reader)?;
        if map.len() == 0 {
            break;
        }

        seed_ranges = seed_ranges.iter().fold(vec![], |mut acc, (origin, count)| {
            let mut o = *origin;
            let mut n = *count;

            while n > 0 {
                if let Some((k, (d, w))) = map.iter().find(|(k, (_, w))| o >= **k && o < **k + *w) {
                    if o < *k {
                        acc.push((o, *k - o));
                    }
                    match min(o + n - *k, *w) {
                        0 => {
                            n = o + n - *k;
                            o = *k;
                        },
                        r => {
                            acc.push((*d + (o - *k), r - (o - *k)));
                            n = o + n - (*k + r);
                            o = *k + r;
                        },
                    }
                } else {
                    acc.push((o, n));
                    break;
                }
            }
            acc
        });
    }

    Ok(*seed_ranges.iter().map(|(x, _)| x).min().unwrap())
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    println!("RESULT: {}", part2(input)?);

    Ok(())
}
