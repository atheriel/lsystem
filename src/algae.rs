extern crate lsystem;

use lsystem::lsystem_iter;

#[deriving(Clone)]
pub enum AlgeaState {
    Reproduction,
    Growth,
}

fn algae_rule(input: AlgeaState) -> Vec<AlgeaState> {
    match input {
        AlgeaState::Reproduction => vec![AlgeaState::Reproduction, AlgeaState::Growth],
        AlgeaState::Growth => vec![AlgeaState::Reproduction]
    }
}

fn main() {
    // Print out the first eight levels of the Algae sequence in the same
    // format as in the Wikipedia article.
    for (index, n) in lsystem_iter(AlgeaState::Growth, algae_rule).
                      take(8).enumerate() {
        let mut printed = format!("n = {}: ", index);
        for i in n.iter() {
            match i {
                &AlgeaState::Reproduction => printed.push_str("A"),
                &AlgeaState::Growth => printed.push_str("B")
            }
        }
        println!("{}", printed)
    }
}
