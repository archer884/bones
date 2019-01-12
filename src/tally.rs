use std::fmt::{self, Display};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct Tally {
    sum: u32,
    values: Vec<u32>,
}

impl Display for Tally {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.values.len() {
            0 => f.write_str("0 []"),
            _ => {
                write!(f, "{} [", self.sum)?;
                let mut values = self.values.iter();
                for item in (&mut values).take(self.values.len() - 1) {
                    write!(f, "{}, ", item)?;
                }
                write!(f, "{}]", values.next().expect("Impossibru"))
            }
        }
    }
}

impl FromIterator<u32> for Tally {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Tally {
        let mut sum = 0;
        let mut values = Vec::with_capacity(4);

        iter.into_iter().for_each(|n| {
            sum += n;
            values.push(n);
        });

        Tally { sum, values }
    }
}
