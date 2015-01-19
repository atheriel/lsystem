extern crate lsystem;

use lsystem::LSystem;

enum Turtle {
    Forward(u32), Left(u32), Right(u32)
}

trait TurtleInterpretation {
    fn to_turtle(&self) -> Turtle;
}

fn draw<T: TurtleInterpretation>(v: Vec<T>) {
    println!("import turtle\n\nturtle.speed(0)\n");

    for command in v.iter() {
        match command.to_turtle() {
            Turtle::Forward(val) => println!("turtle.forward({})", val),
            Turtle::Left(val) => println!("turtle.left({})", val),
            Turtle::Right(val) => println!("turtle.right({})", val)
        }
    }

    println!("\nturtle.exitonclick()\n");
}

#[derive(Clone)]
enum Koch {
    F, Plus, Minus
}

impl TurtleInterpretation for Koch {
    fn to_turtle(&self) -> Turtle {
        match *self {
            Koch::F => Turtle::Forward(10),
            Koch::Plus => Turtle::Left(90),
            Koch::Minus => Turtle::Right(90)
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
