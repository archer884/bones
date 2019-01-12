mod spec;
mod tally;

use crate::spec::Specification;

fn main() {
    let mut rng = rand::thread_rng();
    for spec in read_specifications() {
        println!("{}", spec.sample(&mut rng));
    }
}

fn read_specifications() -> impl Iterator<Item = Specification> {
    use std::env;

    fn parse_specification(s: impl AsRef<str>) -> Option<Specification> {
        s.as_ref().parse().ok()
    }

    env::args().skip(1).filter_map(parse_specification)
}
