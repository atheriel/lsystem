extern crate lsystem;

use lsystem::LSystem;
use lsystem::turtle::{Turtle, TurtleInterpretation};

fn draw<T: TurtleInterpretation>(v: Vec<T>) {
    println!("import turtle\n");
    println!("turtle.speed(0)");
    println!("turtle.hideturtle()");
    println!("turtle.screensize(400, 400)");
    println!("turtle.up()");
    println!("turtle.setposition(-320, -260)");
    println!("turtle.down()");

    for command in v.iter() {
        match command.to_turtle() {
            Turtle::Forward(val) => println!("turtle.forward({})", val),
            Turtle::Left(val)    => println!("turtle.left({})", val),
            Turtle::Right(val)   => println!("turtle.right({})", val),
            _ => ()
        }
    }

    println!("\nturtle.exitonclick()\n");
}

#[derive(Clone)]
enum Sierpinski {
    A, B, Plus, Minus
}

impl TurtleInterpretation for Sierpinski {
    fn to_turtle(&self) -> Turtle {
        match *self {
            Sierpinski::A     => Turtle::Forward(10),
            Sierpinski::B     => Turtle::Forward(10),
            Sierpinski::Plus  => Turtle::Left(60.0),
            Sierpinski::Minus => Turtle::Right(60.0)
        }
    }
}

fn main() {
    let mut s = LSystem::new(vec!(Sierpinski::A), |x| match x {
        // A -> B-A-B
        Sierpinski::A => vec!(Sierpinski::B, Sierpinski::Minus, Sierpinski::A,
                              Sierpinski::Minus, Sierpinski::B),
        // B -> A+B+A
        Sierpinski::B => vec!(Sierpinski::A, Sierpinski::Plus, Sierpinski::B,
                              Sierpinski::Plus, Sierpinski::A),
        // + and - are constants.
        c => vec!(c)
    });

    draw(s.nth(6).unwrap());
}
