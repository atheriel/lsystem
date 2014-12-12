extern crate lsystem;

use lsystem::LSystemType;

#[deriving(Show, Clone)]
pub enum AlgeaAlphabet {
    A,
    B,
}

fn algae_rule(input: AlgeaAlphabet) -> Vec<AlgeaAlphabet> {
    match input {
        AlgeaAlphabet::A => vec![AlgeaAlphabet::A, AlgeaAlphabet::B],
        AlgeaAlphabet::B => vec![AlgeaAlphabet::A]
    }
}

fn main() {
    let algae_lsystem = LSystemType::from_fn(algae_rule);
    let algae_n7 = algae_lsystem.recurse(AlgeaAlphabet::A).take(8);

    // Print out the first eight levels of the Algae sequence in the same
    // format as in the Wikipedia article.
    for (index, n) in algae_n7.enumerate() {
        let mut printed = format!("n = {}: ", index);
        for i in n.iter() {
            match i {
                &AlgeaAlphabet::A => printed.push_str("A"),
                &AlgeaAlphabet::B => printed.push_str("B")
            }
        }
        println!("{}", printed)
    }
}
