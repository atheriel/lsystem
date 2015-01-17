extern crate lsystem;

use lsystem::LSystemType;

use self::Anabaena::{Ar, Al, Br, Bl};

macro_rules! derive_rulefn{
    ($T:ty, $ruleset:ident, { $($pred:pat => $succ:expr),+ }) => (
        fn $ruleset(input: $T) -> Vec<$T> {
            match input {
                $(
                $pred => $succ,
                )+
            }
        }
    );
}

#[derive(Clone)]
enum Anabaena {
    Ar, Al,
    Br, Bl
}

impl std::fmt::Show for Anabaena {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Ar => write!(f, "-->"),
            &Al => write!(f, "<--"),
            &Br => write!(f, "->"),
            &Bl => write!(f, "<-")
        }
    }
}

derive_rulefn!(Anabaena, anabaena_rule,
    {
        Ar => vec!(Al, Br),
        Al => vec!(Bl, Ar),
        Br => vec!(Ar),
        Bl => vec!(Al)
    }
);

fn main() {
    let anabaena_lsystem = LSystemType::new(vec!(Ar), anabaena_rule);

    for item in anabaena_lsystem.iter().skip(4).next().unwrap().iter() {
        print!("{:?} ", item)
    }
    
    println!("")
}
