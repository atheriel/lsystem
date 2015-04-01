extern crate lsystem;

use lsystem::LSystem;
use lsystem::turtle::{Turtle, TurtleInterpretation, draw};

#[derive(Clone)]
enum Koch {
    F, Plus, Minus
}

impl TurtleInterpretation for Koch {
    fn to_turtle(&self) -> Turtle {
        match *self {
            Koch::F     => Turtle::Forward(10),
            Koch::Plus  => Turtle::Left(90.0),
            Koch::Minus => Turtle::Right(90.0)
        }
    }
}

fn main() {
    // Gets the third iteration of the Koch curve.
    let koch = LSystem::new(vec!(Koch::F), |x| match x {
        // The only production is F -> F+F−F−F+F.
        Koch::F => vec!(Koch::F, Koch::Plus, Koch::F, Koch::Minus, Koch::F, Koch::Minus, Koch::F,
                        Koch::Plus, Koch::F),
        // + and - are constants.
        c => vec!(c)
    }).nth(3).unwrap();

    draw(koch);
}
