use std::fmt::{self, Display};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Tally {
    sum: u32,
    values: Vec<u32>,
    modifier: Option<u32>,
}

impl Tally {
    pub fn modify(&mut self, modifier: u32) {
        self.modifier = Some(modifier);
    }

    pub fn sum(&self) -> u32 {
        self.sum
    }
}

impl Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.values.len() {
            0 => return f.write_str("0 []"),
            _ => {
                write!(f, "{} [", self.sum + self.modifier.unwrap_or_default())?;
                let mut values = self.values.iter();
                for item in (&mut values).take(self.values.len() - 1) {
                    write!(f, "{}, ", item)?;
                }
                write!(f, "{}]", values.next().expect("Impossibru"))?;
            }
        }

        match self.modifier {
            Some(modifier) => write!(f, " (+{})", modifier),
            None => Ok(())
        }
    }
}

impl FromIterator<u32> for Tally {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Tally {
        let mut sum = 0;
        let mut values = Vec::new();

        for n in iter {
            sum += n;
            values.push(n);
        }

        Tally { sum, values, modifier: None }
    }
}
