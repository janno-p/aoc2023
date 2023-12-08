use std::{collections::HashMap, io::BufRead, ops::ControlFlow};

use aoc::{execute, get_reader, Result};

enum Direction {
    Left,
    Right,
}

fn read_path<R: BufRead>(reader: &mut R) -> Result<Vec<Direction>> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let result = buffer
        .trim()
        .as_bytes()
        .iter()
        .filter_map(|&b| match b {
            b'L' => Some(Direction::Left),
            b'R' => Some(Direction::Right),
            _ => None,
        })
        .collect();
    Ok(result)
}

fn read_map<R: BufRead>(reader: &mut R) -> Result<HashMap<String, (String, String)>> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut map = HashMap::<String, (String, String)>::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let mut split = buffer.split('=').map(|x| x.trim());
        let key = split.next().unwrap();
        let values = split.next().unwrap();

        let mut value_split = values
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .map(|x| x.trim());

        let left_value = value_split.next().unwrap();
        let right_value = value_split.next().unwrap();

        map.insert(
            key.to_string(),
            (left_value.to_string(), right_value.to_string()),
        );
    }

    Ok(map)
}

fn get_path_length(
    start_pos: &str,
    end_pos: &str,
    path: &[Direction],
    map: &HashMap<String, (String, String)>,
) -> u64 {
    let direction = path
        .iter()
        .cycle()
        .try_fold((0, start_pos), |(sum, pos), dir| {
            let (left_value, right_value) = map.get(pos).unwrap();
            let next_pos = match dir {
                Direction::Left => left_value,
                Direction::Right => right_value,
            };
            if next_pos == end_pos {
                ControlFlow::Break((sum + 1, next_pos.as_str()))
            } else {
                ControlFlow::Continue((sum + 1, next_pos.as_str()))
            }
        });

    if let ControlFlow::Break((sum, _)) = direction {
        sum
    } else {
        0
    }
}

fn get_cycle(start_pos: &str, path: &[Direction], map: &HashMap<String, (String, String)>) -> u64 {
    let direction = path.iter().enumerate().cycle().try_fold(
        (0, start_pos, HashMap::new()),
        |(sum, pos, mut set), (i, dir)| {
            let (left_value, right_value) = map.get(pos).unwrap();
            let next_pos = match dir {
                Direction::Left => left_value,
                Direction::Right => right_value,
            };
            if next_pos.ends_with('Z') {
                if let Some(it) = set.get(&(i, next_pos)) {
                    ControlFlow::Break((*it, next_pos.as_str(), set))
                } else {
                    set.insert((i, next_pos), sum + 1);
                    ControlFlow::Continue((sum + 1, next_pos.as_str(), set))
                }
            } else {
                ControlFlow::Continue((sum + 1, next_pos.as_str(), set))
            }
        },
    );

    if let ControlFlow::Break((sum, _, _)) = direction {
        sum
    } else {
        0
    }
}

fn part1() -> Result<u64> {
    let mut reader = get_reader();

    let path = read_path(&mut reader)?;
    let map = read_map(&mut reader)?;

    Ok(get_path_length("AAA", "ZZZ", &path, &map))
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn part2() -> Result<u64> {
    let mut reader = get_reader();

    let path = read_path(&mut reader)?;
    let map = read_map(&mut reader)?;

    let result = map
        .keys()
        .filter(|&x| x.ends_with('A'))
        .map(|x| get_cycle(x, &path, &map))
        .reduce(lcm)
        .unwrap();

    Ok(result)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
