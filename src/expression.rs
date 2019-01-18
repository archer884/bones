use crate::{
    spec::{ParseError, Specification},
    tally::Tally,
};
use rand::Rng;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Expression {
    base: Specification,
    modifier: Option<Modifier>,
}

impl Expression {
    pub fn sample(self, rng: &mut impl Rng) -> Tally {
        match self.modifier.map(|modifier| modifier.sample(rng)) {
            None => self.base.sample(rng),
            Some(modifier) => {
                let mut tally = self.base.sample(rng);
                tally.modify(modifier);
                tally
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Modifier {
    Constant(u8),
    Specification(Specification),
}

impl Modifier {
    fn sample(&self, rng: &mut impl Rng) -> u32 {
        match self {
            Modifier::Specification(spec) => spec.sample(rng).sum(),
            Modifier::Constant(n) => u32::from(*n),
        }
    }
}

impl FromStr for Expression {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Expression, ParseError> {
        fn parse_specification(s: &str) -> Result<Specification, ParseError> {
            s.parse()
        }

        // FIXME: Allow for multiple modifiers? Will require different representation.
        let mut parts = s.split('+');

        let base = parts
            .next()
            .map(parse_specification)
            .ok_or(ParseError::SegmentCount)??;

        let modifier = match parts.next() {
            None => None,
            Some(modifier) => {
                if modifier.contains('d') {
                    Some(Modifier::Specification(modifier.parse()?))
                } else {
                    Some(Modifier::Constant(modifier.parse()?))
                }
            }
        };

        Ok(Expression { base, modifier })
    }
}
