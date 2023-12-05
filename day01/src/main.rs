use std::io::BufRead;

use aoc::{execute, get_reader, Result};

fn get_value_part1(bytes: &Vec<u8>) -> u32 {
    let mut a = None;
    let mut b = None;

    for i in 0..bytes.len() {
        if a.is_none() {
            let x = bytes[i];
            if x.is_ascii_digit() {
                a = Some((x - b'0') as u32);
            }
        }
        if b.is_none() {
            let x = bytes[bytes.len() - i - 1];
            if x.is_ascii_digit() {
                b = Some((x - b'0') as u32);
            }
        }
        if a.is_some() && b.is_some() {
            break;
        }
    }

    a.unwrap_or_default() * 10 + b.unwrap_or_default()
}

fn get_spelled_digit(slice: &[u8]) -> Option<u32> {
    if slice.len() < 3 {
        return None;
    }
    match slice[0..3] {
        [b'o', b'n', b'e'] => Some(1),
        [b't', b'w', b'o'] => Some(2),
        [b't', b'h', b'r'] => {
            if slice.len() > 4 && slice[3..5] == [b'e', b'e'] {
                Some(3)
            } else {
                None
            }
        }
        [b'f', b'o', b'u'] => {
            if slice.len() > 3 && slice[3] == b'r' {
                Some(4)
            } else {
                None
            }
        }
        [b'f', b'i', b'v'] => {
            if slice.len() > 3 && slice[3] == b'e' {
                Some(5)
            } else {
                None
            }
        }
        [b's', b'i', b'x'] => Some(6),
        [b's', b'e', b'v'] => {
            if slice.len() > 4 && slice[3..5] == [b'e', b'n'] {
                Some(7)
            } else {
                None
            }
        }
        [b'e', b'i', b'g'] => {
            if slice.len() > 4 && slice[3..5] == [b'h', b't'] {
                Some(8)
            } else {
                None
            }
        }
        [b'n', b'i', b'n'] => {
            if slice.len() > 3 && slice[3] == b'e' {
                Some(9)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_spelled_digit_rev(slice: &[u8]) -> Option<u32> {
    if slice.len() < 3 {
        return None;
    }
    match slice[(slice.len() - 3)..slice.len()] {
        [b'o', b'n', b'e'] => Some(1),
        [b't', b'w', b'o'] => Some(2),
        [b'r', b'e', b'e'] => {
            if slice.len() > 4 && slice[(slice.len() - 5)..(slice.len() - 3)] == [b't', b'h'] {
                Some(3)
            } else {
                None
            }
        }
        [b'o', b'u', b'r'] => {
            if slice.len() > 3 && slice[slice.len() - 4] == b'f' {
                Some(4)
            } else {
                None
            }
        }
        [b'i', b'v', b'e'] => {
            if slice.len() > 3 && slice[slice.len() - 4] == b'f' {
                Some(5)
            } else {
                None
            }
        }
        [b's', b'i', b'x'] => Some(6),
        [b'v', b'e', b'n'] => {
            if slice.len() > 4 && slice[(slice.len() - 5)..(slice.len() - 3)] == [b's', b'e'] {
                Some(7)
            } else {
                None
            }
        }
        [b'g', b'h', b't'] => {
            if slice.len() > 4 && slice[(slice.len() - 5)..(slice.len() - 3)] == [b'e', b'i'] {
                Some(8)
            } else {
                None
            }
        }
        [b'i', b'n', b'e'] => {
            if slice.len() > 3 && slice[slice.len() - 4] == b'n' {
                Some(9)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn get_value_part2(bytes: &Vec<u8>) -> u32 {
    let mut a = None;
    let mut b = None;

    for i in 0..bytes.len() {
        if a.is_none() {
            let x = bytes[i];
            if x.is_ascii_digit() {
                a = Some((x - b'0') as u32);
            } else if let Some(v) = get_spelled_digit(&bytes[i..]) {
                a = Some(v)
            }
        }
        if b.is_none() {
            let x = bytes[bytes.len() - i - 1];
            if x.is_ascii_digit() {
                b = Some((x - b'0') as u32);
            } else if let Some(v) = get_spelled_digit_rev(&bytes[0..(bytes.len() - i)]) {
                b = Some(v)
            }
        }
        if a.is_some() && b.is_some() {
            break;
        }
    }

    a.unwrap_or_default() * 10 + b.unwrap_or_default()
}

fn part1() -> Result<u32> {
    let mut reader = get_reader();

    let mut sum = 0;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let bytes: Vec<u8> = buffer.bytes().collect();
        sum += get_value_part1(&bytes);
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

        let bytes: Vec<u8> = buffer.bytes().collect();
        sum += get_value_part2(&bytes);
    }

    Ok(sum)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
