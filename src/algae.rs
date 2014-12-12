extern crate lsystem;

use lsystem::LSystemType;

#[deriving(Clone)]
pub enum AlgeaAlphabet {
	AlgaeA,
	AlgaeB,
}

impl std::fmt::Show for AlgeaAlphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &AlgeaAlphabet::AlgaeA => write!(f, "{}", "A"),
            &AlgeaAlphabet::AlgaeB => write!(f, "{}", "B")
        }
    }
}

fn algae_rule(input: AlgeaAlphabet) -> Vec<AlgeaAlphabet> {
	match input {
		AlgeaAlphabet::AlgaeA => vec![AlgeaAlphabet::AlgaeA, AlgeaAlphabet::AlgaeB],
		AlgeaAlphabet::AlgaeB => vec![AlgeaAlphabet::AlgaeA]
	}
}

fn main() {
	let algae_lsystem = LSystemType::from_fn(algae_rule);
	for n in algae_lsystem.recurse(AlgeaAlphabet::AlgaeA).take(5) {
		println!("{}", n)
	}
}
