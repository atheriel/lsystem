extern crate lsystem;

use lsystem::LSystem;

// Re-import the enum variants for more concise code.
use self::AlgeaState::{A, B};

#[derive(Clone, Copy)]
enum AlgeaState {
    #[doc = "Reproduction State"] A,
    #[doc = "Growth State"] B
}

fn main() {
    // Use a closure to express the growth patterns of the algae cells.
    let mut algae = LSystem::new(vec!(AlgeaState::B), |x| match x {
        A => vec![A, B],
        B => vec![A]
    });

    // Print out the first eight levels of the Algae sequence in the same
    // format as in the Wikipedia article.
    for (index, n) in algae.take(8).enumerate() {
        let mut printed = format!("n = {}: ", index);
        for i in n.iter() {
            match i {
                &AlgeaState::A => printed.push_str("A"),
                &AlgeaState::B => printed.push_str("B")
            }
        }
        println!("{}", printed)
    }
}
