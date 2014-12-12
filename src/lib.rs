pub struct Rule<T> {
    predecessor: Vec<T>,
    successor: Vec<T>
}

pub struct LSystem<T> {
    alphabet: Vec<T>,
    axiom: T,
    rules: Vec<Rule<T>>
}

#[test]
fn it_works() {
}
