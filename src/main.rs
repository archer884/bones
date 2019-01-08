mod spec;

fn main() {    
    use crate::spec::Specification;

    let _ = dbg!("2d256".parse::<Specification>());
}
