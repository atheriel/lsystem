extern crate lsystem;

use lsystem::LSystemType;

#[deriving(Show, Clone)]
pub enum AlgeaAlphabet {
	AlgaeA,
	AlgaeB,
}

fn algae_rule(input: AlgeaAlphabet) -> Vec<AlgeaAlphabet> {
	match input {
		AlgeaAlphabet::AlgaeA => vec![AlgeaAlphabet::AlgaeA, AlgeaAlphabet::AlgaeB],
		AlgeaAlphabet::AlgaeB => vec![AlgeaAlphabet::AlgaeA]
	}
}

fn main() {
	let algae_lsystem = LSystemType::from_fn(algae_rule);
	for n in algae_lsystem.recurse(AlgeaAlphabet::AlgaeA) {
		println!("{}", n)
		break;
	}
	// println!("{}", algae_lsystem.recurse(AlgeaAlphabet::AlgaeA).take(2).collect());
}
