use std::{collections::HashMap, io::BufRead};

use aoc::{execute, get_reader, Result};

#[derive(Clone, Copy, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type(hand_str: &str) -> HandType {
    let map = hand_str
        .trim()
        .as_bytes()
        .iter()
        .fold(HashMap::<u8, u8>::new(), |mut acc, &b| {
            let e = acc.entry(b).or_insert(0);
            *e += 1;
            acc
        });
    if map.len() == 1 {
        HandType::FiveOfAKind
    } else if map.len() == 2 {
        if map.values().any(|&x| x == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else if map.len() == 3 {
        if map.values().any(|&x| x == 3) {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if map.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn get_hand_type_alt(hand_str: &str) -> HandType {
    let mut map =
        hand_str
            .trim()
            .as_bytes()
            .iter()
            .fold(HashMap::<u8, u8>::new(), |mut acc, &b| {
                let e = acc.entry(b).or_insert(0);
                *e += 1;
                acc
            });

    let num_jokers = map.remove(&b'J').unwrap_or_default();

    if map.len() < 2 {
        HandType::FiveOfAKind
    } else if map.len() == 2 {
        if map.values().any(|&x| x + num_jokers == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        }
    } else if map.len() == 3 {
        if map.values().any(|&x| x + num_jokers == 3) {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        }
    } else if map.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn card_rank(b: u8) -> u32 {
    match b {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        v => (v - b'0') as u32,
    }
}

fn get_hand_rank(hand_str: &str) -> u32 {
    hand_str
        .trim()
        .as_bytes()
        .iter()
        .fold(0u32, |acc, &b| (acc << 4) | card_rank(b))
}

fn get_hand_rank_alt(hand_str: &str) -> u32 {
    hand_str.trim().as_bytes().iter().fold(0u32, |acc, &b| {
        if b == b'J' {
            (acc << 4) | 1
        } else {
            (acc << 4) | card_rank(b)
        }
    })
}

fn part1() -> Result<u64> {
    let mut reader = get_reader();

    let mut hands: Vec<(HandType, u32, u32)> = vec![];

    let mut buffer = String::new();
    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let mut split = buffer.trim().split(' ');
        let hand = split.next().unwrap();
        let bet = split.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

        hands.push((get_hand_type(hand), get_hand_rank(hand), bet));
    }

    hands.sort_by(|a, b| match (a.0 as u32).cmp(&(b.0 as u32)) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
    });

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * (x.2 as u64))
        .sum();

    Ok(result)
}

fn part2() -> Result<u64> {
    let mut reader = get_reader();

    let mut hands: Vec<(HandType, u32, u32)> = vec![];

    let mut buffer = String::new();
    loop {
        buffer.clear();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let mut split = buffer.trim().split(' ');
        let hand = split.next().unwrap();
        let bet = split.next().and_then(|x| x.parse::<u32>().ok()).unwrap();

        hands.push((get_hand_type_alt(hand), get_hand_rank_alt(hand), bet));
    }

    hands.sort_by(|a, b| match (a.0 as u32).cmp(&(b.0 as u32)) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
    });

    let result = hands
        .iter()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1) * (x.2 as u64))
        .sum();

    Ok(result)
}

fn main() -> Result<()> {
    execute(part1, part2)
}
