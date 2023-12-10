use std::io::BufRead;

use aoc::{execute, get_reader, Result};

mod direction {
    pub const UP: u8 = 0x01;
    pub const DOWN: u8 = 0x02;
    pub const RIGHT: u8 = 0x04;
    pub const LEFT: u8 = 0x08;
}

fn part1() -> Result<u64> {
    let mut reader = get_reader();
    let mut buffer = String::new();

    let mut chart: Vec<Vec<u8>> = vec![];
    let mut start = None;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let row: Vec<_> = buffer
            .trim()
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(x, &ch)| match ch {
                b'|' => direction::UP | direction::DOWN,
                b'-' => direction::LEFT | direction::RIGHT,
                b'L' => direction::UP | direction::RIGHT,
                b'J' => direction::UP | direction::LEFT,
                b'7' => direction::LEFT | direction::DOWN,
                b'F' => direction::RIGHT | direction::DOWN,
                b'S' => {
                    start = Some((x, chart.len()));
                    direction::UP | direction::DOWN | direction::RIGHT | direction::LEFT
                }
                _ => 0,
            })
            .collect();

        chart.push(row);
    }

    let mut sum = 0;

    'outer: for initial in [
        direction::UP,
        direction::RIGHT,
        direction::DOWN,
        direction::LEFT,
    ] {
        sum = 0;
        let mut next_dir = initial;
        let mut current_pos = start.unwrap();

        loop {
            sum += 1;
            let (next_pos, dir) = if (next_dir & direction::UP) != 0 {
                if current_pos.1 == 0
                    || (chart[current_pos.1 - 1][current_pos.0] & direction::DOWN) == 0
                {
                    break;
                } else {
                    let p = (current_pos.0, current_pos.1 - 1);
                    (p, chart[p.1][p.0] & (!direction::DOWN))
                }
            } else if (next_dir & direction::DOWN) != 0 {
                if current_pos.1 == chart.len() - 1
                    || (chart[current_pos.1 + 1][current_pos.0] & direction::UP) == 0
                {
                    break;
                } else {
                    let p = (current_pos.0, current_pos.1 + 1);
                    (p, chart[p.1][p.0] & (!direction::UP))
                }
            } else if (next_dir & direction::LEFT) != 0 {
                if current_pos.0 == 0
                    || (chart[current_pos.1][current_pos.0 - 1] & direction::RIGHT) == 0
                {
                    break;
                } else {
                    let p = (current_pos.0 - 1, current_pos.1);
                    (p, chart[p.1][p.0] & (!direction::RIGHT))
                }
            } else if (next_dir & direction::RIGHT) != 0 {
                if current_pos.0 == chart[0].len() - 1
                    && (chart[current_pos.1][current_pos.0 + 1] & direction::LEFT) == 0
                {
                    break;
                } else {
                    let p = (current_pos.0 + 1, current_pos.1);
                    (p, chart[p.1][p.0] & (!direction::LEFT))
                }
            } else {
                panic!("never!");
            };

            next_dir = dir;

            if Some(next_pos) == start {
                break 'outer;
            }

            current_pos = next_pos;
        }
    }

    Ok(sum / 2)
}

fn _print_path(chart: &[Vec<u8>]) {
    print!("\x1B[2J");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    chart.iter().for_each(|x| {
        x.iter().for_each(|&y| {
            let n = match y {
                0 => '.',
                w if w == direction::DOWN | direction::UP => '│',
                w if w == direction::LEFT | direction::RIGHT => '─',
                w if w == direction::DOWN | direction::RIGHT => '┌',
                w if w == direction::DOWN | direction::LEFT => '┐',
                w if w == direction::UP | direction::RIGHT => '└',
                w if w == direction::UP | direction::LEFT => '┘',
                16 => ' ',
                _ => '█',
            };
            print!("{}", n)
        });
        println!();
    })
}

fn part2() -> Result<u64> {
    let mut reader = get_reader();
    let mut buffer = String::new();

    let mut original_chart: Vec<Vec<u8>> = vec![];
    let mut start = None;

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let row: Vec<_> = buffer
            .trim()
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(x, &ch)| match ch {
                b'|' => direction::UP | direction::DOWN,
                b'-' => direction::LEFT | direction::RIGHT,
                b'L' => direction::UP | direction::RIGHT,
                b'J' => direction::UP | direction::LEFT,
                b'7' => direction::LEFT | direction::DOWN,
                b'F' => direction::RIGHT | direction::DOWN,
                b'S' => {
                    start = Some((x, original_chart.len()));
                    direction::UP | direction::DOWN | direction::RIGHT | direction::LEFT
                }
                _ => 0,
            })
            .collect();

        original_chart.push(row);
    }

    fn index(i: usize) -> usize {
        2 * i + 1
    }

    let mut chart: Vec<Vec<u8>> = vec![];

    'outer: for initial in [
        direction::UP,
        direction::RIGHT,
        direction::DOWN,
        direction::LEFT,
    ] {
        chart.clear();
        chart.push(vec![16; 2 * original_chart[0].len() + 1]);

        original_chart.iter().for_each(|v| {
            chart.push(v.iter().fold(vec![16], |mut acc, &q| {
                acc.push(q);
                acc.push(16);
                acc
            }));
            chart.push(vec![16; 2 * original_chart[0].len() + 1]);
        });

        let mut next_dir = initial;
        let mut current_pos = start.unwrap();

        loop {
            let (next_pos, dir) = if (next_dir & direction::UP) != 0 {
                if current_pos.1 == 0
                    || (chart[index(current_pos.1 - 1)][index(current_pos.0)] & direction::DOWN)
                        == 0
                {
                    break;
                } else {
                    let p = (current_pos.0, current_pos.1 - 1);
                    (p, chart[index(p.1)][index(p.0)] & (!direction::DOWN))
                }
            } else if (next_dir & direction::DOWN) != 0 {
                if current_pos.1 == chart.len() - 1
                    || (chart[index(current_pos.1 + 1)][index(current_pos.0)] & direction::UP) == 0
                {
                    break;
                } else {
                    let p = (current_pos.0, current_pos.1 + 1);
                    (p, chart[index(p.1)][index(p.0)] & (!direction::UP))
                }
            } else if (next_dir & direction::LEFT) != 0 {
                if current_pos.0 == 0
                    || (chart[index(current_pos.1)][index(current_pos.0 - 1)] & direction::RIGHT)
                        == 0
                {
                    break;
                } else {
                    let p = (current_pos.0 - 1, current_pos.1);
                    (p, chart[index(p.1)][index(p.0)] & (!direction::RIGHT))
                }
            } else if (next_dir & direction::RIGHT) != 0 {
                if current_pos.0 == chart[0].len() - 1
                    || (chart[index(current_pos.1)][index(current_pos.0 + 1)] & direction::LEFT)
                        == 0
                {
                    break;
                } else {
                    let p = (current_pos.0 + 1, current_pos.1);
                    (p, chart[index(p.1)][index(p.0)] & (!direction::LEFT))
                }
            } else {
                panic!("never!");
            };

            next_dir = dir;

            chart[index(current_pos.1)][index(current_pos.0)] = b'#';
            if next_pos.1 > current_pos.1 {
                chart[index(current_pos.1) + 1][index(current_pos.0)] = b'#';
            } else if current_pos.1 > next_pos.1 {
                chart[index(current_pos.1) - 1][index(current_pos.0)] = b'#';
            } else if next_pos.0 > current_pos.0 {
                chart[index(current_pos.1)][index(current_pos.0) + 1] = b'#';
            } else if current_pos.0 > next_pos.0 {
                chart[index(current_pos.1)][index(current_pos.0) - 1] = b'#';
            }

            if Some(next_pos) == start {
                // print_path(&chart);
                break 'outer;
            }

            current_pos = next_pos;
        }
    }

    let inside_pos = get_inside_position(&chart);

    let mut cursors = vec![inside_pos];
    let mut internal_tiles = 0;

    while let Some(n) = move_and_find(&mut chart, &mut cursors) {
        internal_tiles += n;
    }

    Ok(internal_tiles as u64)
}

fn get_adjacent_tiles(chart: &[Vec<u8>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
        .iter()
        .filter(|(a, b)| chart[*a][*b] != b'#')
        .copied()
        .collect()
}

fn move_and_find(chart: &mut [Vec<u8>], cursors: &mut Vec<(usize, usize)>) -> Option<u32> {
    let mut tiles = 0;
    let mut new_cursors = vec![];
    for cursor in cursors.iter() {
        for (y, x) in get_adjacent_tiles(chart, *cursor) {
            if chart[y][x] != 16 {
                tiles += 1;
            }
            chart[y][x] = b'#';
            new_cursors.push((x, y));
        }
    }
    cursors.clear();
    cursors.extend(new_cursors);
    if !cursors.is_empty() {
        Some(tiles)
    } else {
        None
    }
}

fn get_inside_position(chart: &[Vec<u8>]) -> (usize, usize) {
    for (y, v) in chart.iter().enumerate() {
        for x in 0..v.len() {
            if chart[y][x] == b'#' {
                return (x + 1, y + 1);
            }
        }
    }
    panic!("never")
}

fn main() -> Result<()> {
    execute(part1, part2)
}
