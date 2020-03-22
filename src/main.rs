use bangdice::Expression;
use std::env;

fn main() {
    let mut rng = rand::thread_rng();
    let mut sum = 0;

    for expression in read_expressions() {
        let result = expression.execute(&mut rng);
        sum += result.total();
        println!("{}", result);
    }

    println!("===\n{:>3}", sum);
}

fn read_expressions() -> impl Iterator<Item = Expression> {
    env::args().skip(1).filter_map(|s| s.parse().ok())
}
