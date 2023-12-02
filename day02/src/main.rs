use std::{io::{self, BufRead}, error, cmp::max};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Draw(u32, u32, u32);

fn tokenize(values: &str) -> Vec<Draw> {
    values.split(";")
        .map(|v| v.trim())
        .map(|draw_str| {
            draw_str.split(",").map(|v| v.trim())
                .fold(Draw(0, 0, 0), |draw, x| {
                    let q: Vec<&str> = x.split(" ").map(|v| v.trim()).collect();
                    if q.len() == 2 {
                        let n = q[0].parse::<u32>().unwrap();
                        match q[1] {
                            "red" => Draw(n, draw.1, draw.2),
                            "green" => Draw(draw.0, n, draw.2),
                            "blue" => Draw(draw.0, draw.1, n),
                            _ => draw,
                        }
                    } else {
                        draw
                    }
                })
        })
        .collect()
}

fn cube_conundrum<R>(mut reader: R) -> Result<(u32, u32)>
where
    R: BufRead
{
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer)? == 0 {
            break;
        }

        let id_and_values: Vec<&str> = buffer.split(":").map(|v| v.trim()).collect();
        match id_and_values[..] {
            [game_id_str, values] => {
                let xs: Vec<&str> = game_id_str.split(" ").map(|v| v.trim()).collect();
                let game_id =
                    if xs.len() == 2 {
                        xs[1].parse::<u32>().unwrap()
                    } else {
                        0
                    };
                let draws = tokenize(values);
                if draws.iter().all(|d| d.0 <= RED && d.1 <= GREEN && d.2 <= BLUE) {
                    sum_part1 += game_id;
                }
                let min_cubes = draws.iter().fold(Draw(0, 0, 0), |acc, d| {
                    Draw(max(acc.0, d.0), max(acc.1, d.1), max(acc.2, d.2))
                });
                sum_part2 += min_cubes.0 * min_cubes.1 * min_cubes.2;
            },
            _ => {}
        }
    }

    Ok((sum_part1, sum_part2))
}

fn main() -> Result<()> {
    let stdio = io::stdin();
    let input = stdio.lock();

    let (part1, part2) = cube_conundrum(input)?;
    println!("PART1: {}", part1);
    println!("PART2: {}", part2);

    Ok(())
}
