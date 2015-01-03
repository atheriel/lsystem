//! An example of a Parametric OL-system from _The Algorithmic Beauty of
//! Plants_, by Przemyslaw Prusinkiewicz and Aristid Lindenmayer.
//!
//! See example 1.7, pages 42-43 in the text for details. The book is available
//! online at <http://algorithmicbotany.org/papers/>.

#![feature(macro_rules)]

extern crate lsystem;

use lsystem::LSystemType;

// Re-import the enum variants for more concise code.
use self::Parametric::{A, B, C};

// Define a simple procedural macro to generate a function from more
// conventional rule syntax.
macro_rules! derive_rulefn(
    ($T:ty, $ruleset:ident, { $($pred:pat => $succ:expr),+ }) => (
        fn $ruleset(input: $T) -> Vec<$T> {
            match input {
                $(
                $pred => $succ,
                )+
            }
        }
    );
)

// This type parameterizes two of its variants with real numbers, allowing
// much more flexibility in the definition of growth rules.
#[deriving(Clone, Show)]
enum Parametric {
    A(f32, f32), B(f32), C
}

derive_rulefn!(Parametric, parametric_rule,
    {
        // The conditional substitution rules can be captured with `if-else`
        // constructions.
        A(x, y) =>
            if y <= 3.0 {
                vec!(A(x * 2.0, x + y))
            } else {
                vec!(B(x), A(x / y, 0.0))
            },
        B(x) =>
            if x < 1.0 {
                vec!(C)
            } else {
                vec!(B(x - 1.0))
            },
        // C is a constant in this L-system.
        C => vec!(C)
    }
)

fn main() {
    let parametric_lsystem =
        LSystemType::new(vec!(B(2.0), A(4.0, 4.0)), parametric_rule);
    
    println!("{}", parametric_lsystem.iter().skip(4).next().unwrap())
}
