use std::{io::{self, BufRead}, error, collections::{BTreeSet, HashMap}};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct Card {
    winning: BTreeSet<u32>,
    existing: BTreeSet<u32>,
}

impl Card {
    fn parse(row: &str) -> Self {
        let split = row.split(':')
            .map(|v| v.trim())
            .skip(1)
            .take(1)
            .next()
            .unwrap();

        let numbers: Vec<&str> = split.split('|')
            .map(|v| v.trim())
            .take(2)
            .collect();

        Self {
            existing: numbers.get(1).unwrap().split(' ').map(|v| v.trim()).filter(|v| !v.is_empty()).map(|v| v.parse::<u32>().unwrap()).collect(),
            winning: numbers.get(0).unwrap().split(' ').map(|v| v.trim()).filter(|v| !v.is_empty()).map(|v| v.parse::<u32>().unwrap()).collect(),
        }
    }
}

fn scratchcards<R>(mut reader: R) -> Result<(u32, u32)>
where
    R: BufRead
{
    let mut sum_part1 = 0;

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

        let m = v.clone();

        let power = card.winning.intersection(&card.existing).collect::<Vec<&u32>>().len();

        if power > 0 {
            sum_part1 += 1 << (power - 1);

            for i in 1..=power {
                *cards.entry(card_n + i).or_insert(0) += m;
            }
        }
    }

    let sum_part2 = cards.values().map(|v| *v as u32).sum();

    Ok((sum_part1, sum_part2))
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let (part1, part2) = scratchcards(input)?;
    println!("PART1: {}", part1);
    println!("PART2: {}", part2);

    Ok(())
}
