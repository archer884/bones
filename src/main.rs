use bangdice::Expression;
use std::env;

fn main() {
    let mut rng = rand::thread_rng();
    for expression in read_expressions() {
        println!("{}", expression.execute(&mut rng));
    }
}

fn read_expressions() -> impl Iterator<Item = Expression> {
    env::args().skip(1).filter_map(|s| s.parse().ok())
}
