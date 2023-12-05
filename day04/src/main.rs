use std::{
    collections::{BTreeSet, HashMap},
    io::BufRead,
};

use aoc::{execute, get_reader, Result};

struct Card {
    winning: BTreeSet<u32>,
    existing: BTreeSet<u32>,
}

impl Card {
    fn parse(row: &str) -> Self {
        let split = row
            .split(':')
            .map(|v| v.trim())
            .skip(1)
            .take(1)
            .next()
            .unwrap();

        let numbers: Vec<&str> = split.split('|').map(|v| v.trim()).take(2).collect();

        Self {
            existing: numbers
                .get(1)
                .unwrap()
                .split(' ')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect(),
            winning: numbers
                .first()
                .unwrap()
                .split(' ')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect(),
        }
    }
}

fn part1() -> Result<u32> {
    let mut reader = get_reader();

    let mut sum = 0;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let card = Card::parse(&buffer);

        let power = card
            .winning
            .intersection(&card.existing)
            .collect::<Vec<&u32>>()
            .len();

        if power > 0 {
            sum += 1 << (power - 1);
        }
    }

    Ok(sum)
}

fn part2() -> Result<u32> {
    let mut reader = get_reader();

    let mut cards = HashMap::<usize, usize>::new();
    let mut card_n = 0usize;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let card = Card::parse(&buffer);
        card_n += 1;
        let v = cards.entry(card_n).or_insert(0);
        *v += 1;

        let m = *v;

        let power = card
            .winning
            .intersection(&card.existing)
            .collect::<Vec<&u32>>()
            .len();

        if power > 0 {
            for i in 1..=power {
                *cards.entry(card_n + i).or_insert(0) += m;
            }
        }
    }

    Ok(cards.values().map(|v| *v as u32).sum())
}

fn main() -> Result<()> {
    execute(part1, part2)
}
