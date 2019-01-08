use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
/// A struct specifying a roll of dice of a given size
pub struct Specification {
    size: u8,
    count: u8,
}

impl Specification {
    fn new(count: u8, size: u8) -> Specification {
        Specification {
            size,
            count,
        }
    }
    
    fn size(size: u8) -> Specification {
        Specification {
            size,
            count: 1,
        }
    }
}

impl FromStr for Specification {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_int(s: &str) -> Result<u8, ParseIntError> {
            s.parse()
        }
        
        let mut parts = s.split('d').map(parse_int);

        let left = parts.next().ok_or(ParseError::SegmentCount)??;
        let right = parts.next();

        if parts.next().is_some() {
            return Err(ParseError::SegmentCount);
        }

        match right {
            Some(right) => Ok(Specification::new(left, right?)),
            None => Ok(Specification::size(left)),
        }
    }
}

#[derive(Debug)]
/// An enum describing a failure to parse
pub enum ParseError {
    Int(ParseIntError),
    SegmentCount,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Int(e) => write!(f, "Parsing error: {}", e),
            ParseError::SegmentCount => f.write_str("Bad format"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::Int(e) => Some(e),
            ParseError::SegmentCount => None,
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        ParseError::Int(e)
    }
}
