extern crate lsystem;

use lsystem::LSystem;
use lsystem::turtle::{Turtle, TurtleInterpretation};

fn draw<T: TurtleInterpretation>(v: Vec<T>, animate: bool) {
    println!("import turtle\n");
    println!("stack = []\n");
    
    // Animation can really slow things down, so make it optional.
    if animate {
        println!("turtle.speed(0)");
    } else {
        println!("turtle.tracer(1000, 0)");
    }

    // Start the turtle so that the seaweed is about centered.
    println!("turtle.hideturtle()");
    println!("turtle.screensize(400, 300)");
    println!("turtle.up()");
    println!("turtle.setposition(0, -150)");
    println!("turtle.setheading(90)");
    println!("turtle.down()");

    for command in v.iter() {
        match command.to_turtle() {
            Turtle::Forward(val) => println!("turtle.forward({})", val),
            Turtle::Left(val)    => println!("turtle.left({})", val),
            Turtle::Right(val)   => println!("turtle.right({})", val),
            Turtle::Push         => println!("stack.append((turtle.pos(), turtle.heading()))"),
            Turtle::Pop          => {
                println!("position, head = stack.pop()");
                println!("turtle.up()");
                println!("turtle.setposition(position)");
                println!("turtle.setheading(head)");
                println!("turtle.down()");
            },
            Turtle::Dummy        => ()
        }
    }

    if !animate {
        println!("turtle.update()");
    }

    println!("\nturtle.exitonclick()\n");
}

#[derive(Clone)]
enum Seaweed {
    F, LBrace, RBrace, Plus, Minus
}

impl TurtleInterpretation for Seaweed {
    fn to_turtle(&self) -> Turtle {
        match *self {
            Seaweed::F      => Turtle::Forward(10),
            Seaweed::LBrace => Turtle::Push,
            Seaweed::RBrace => Turtle::Pop,
            Seaweed::Plus   => Turtle::Right(22.5),
            Seaweed::Minus  => Turtle::Left(22.5),
        }
    }
}

fn main() {
    let s = LSystem::new(vec!(Seaweed::F), |x| match x {
        // F -> FF-[-F+F+F]+[+F-F-F]
        Seaweed::F => vec!(Seaweed::F, Seaweed::F, Seaweed::Minus,
                           Seaweed::LBrace, Seaweed::Minus, Seaweed::F,
                           Seaweed::Plus, Seaweed::F, Seaweed::Plus,
                           Seaweed::F, Seaweed::RBrace, Seaweed::Plus,
                           Seaweed::LBrace, Seaweed::Plus, Seaweed::F,
                           Seaweed::Minus, Seaweed::F, Seaweed::Minus,
                           Seaweed::F, Seaweed::RBrace),
        c => vec!(c)
    }).nth(3).unwrap();

    draw(s, false);
}
