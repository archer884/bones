use bangdice::Expression;

fn main() {
    let mut rng = rand::thread_rng();
    for expression in read_expressions() {
        println!("{}", expression.execute(&mut rng));
    }
}

fn read_expressions() -> impl Iterator<Item = Expression> {
    use std::env;
    env::args().skip(1).filter_map(|s| s.parse().ok())
}
