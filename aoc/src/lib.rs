use core::fmt;
use std::{
    env::args,
    error,
    io::{self, StdinLock},
};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidPart,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPart => write!(
                f,
                "invalid part number: accepted values are 'part1' and 'part2'"
            ),
        }
    }
}

impl error::Error for Error {}

pub type FnPart<T> = fn() -> Result<T>;

pub fn get_reader() -> StdinLock<'static> {
    let stdio = io::stdin();
    stdio.lock()
}

pub fn execute<T: fmt::Display>(part1: FnPart<T>, part2: FnPart<T>) -> Result<()> {
    if let Some(part) = args().nth(1) {
        let result = match part.as_str() {
            "part1" => part1()?,
            "part2" => part2()?,
            _ => return Err(Error::InvalidPart.into()),
        };
        println!("RESULT: {}", result);
        return Ok(());
    }

    Err(Error::InvalidPart.into())
}
