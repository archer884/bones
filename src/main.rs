mod expression;
mod spec;
mod tally;

use crate::expression::Expression;

fn main() {
    let mut rng = rand::thread_rng();
    for spec in read_expressions() {
        println!("{}", spec.sample(&mut rng));
    }
}

fn read_expressions() -> impl Iterator<Item = Expression> {
    use std::env;

    fn parse_specification(s: impl AsRef<str>) -> Option<Expression> {
        s.as_ref().parse().ok()
    }

    env::args().skip(1).filter_map(parse_specification)
}
