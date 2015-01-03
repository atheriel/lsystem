extern crate lsystem;

use lsystem::LSystemType;

use self::Polarity::{L, R};
use self::Anabaena::{A, B};

#[deriving(Clone, Show, Eq, PartialEq)]
enum Polarity { L, R }

#[deriving(Clone, Eq, PartialEq)]
enum Anabaena {
    A(Polarity),
    B(Polarity)
}

impl std::fmt::Show for Anabaena {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &A(R) => write!(f, "-->"),
            &A(L) => write!(f, "<--"),
            &B(R) => write!(f, "->"),
            &B(L) => write!(f, "<-")
        }
    }
}

fn anabaena_rule(input: Anabaena) -> Vec<Anabaena> {
    match input {
        A(R) => vec!(A(L), B(R)),
        A(L) => vec!(B(L), A(R)),
        B(R) => vec!(A(R)),
        B(L) => vec!(A(L)),
    }
}

fn main() {
    let anabaena_lsystem = LSystemType::new(vec!(A(R)), anabaena_rule);

    for item in anabaena_lsystem.iter().skip(4).next().unwrap().iter() {
        print!("{} ", item)
    }
    
    println!("")
}
