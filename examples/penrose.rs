extern crate lsystem;

// File & process manipulation.
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::{File, remove_file};
use std::process::Command;

use lsystem::LSystem;
use lsystem::turtle::{Turtle, TurtleInterpretation};

fn draw<T: TurtleInterpretation>(v: Vec<T>, index: usize) -> (String, String) {
    let filename = format!("penrose-{}.py", index);
    let eps_name = format!("penrose-{}.eps", index);

    // Create a flat text file to hold the Python script.
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    // This sets up some nice drawing parameters in Python, and also implements a stack for the
    // turtle graphics (since the Python implementation does not have one).
    file.write(b"import turtle

stack = []                   # A stack to handle Push/Pop commands.

turtle.tracer(10000, 0)      # Massively reduce screen updates = big speed-up.
turtle.hideturtle()          # Don't actually draw the turtle image.
turtle.screensize(400, 300)  # 4:3 Aspect Ratio
turtle.pensize(2)            # This makes the lines more visible in the `.gif`.

turtle.up()                  # Logo-style orientation.
turtle.setposition(0, 0)
turtle.setheading(90)
turtle.down()\n\n").unwrap();

    // Loop over the vector and turn each Turtle token into the correct command.
    for command in v.iter() {
        let iores = match command.to_turtle() {
            Turtle::Dummy        => file.write(b""),
            Turtle::Forward(val) => file.write(format!("turtle.forward({})", val).as_bytes()),
            Turtle::Left(val)    => file.write(format!("turtle.left({})", val).as_bytes()),
            Turtle::Right(val)   => file.write(format!("turtle.right({})", val).as_bytes()),
            Turtle::Push         => file.write(b"stack.append((turtle.pos(), turtle.heading()))"),
            Turtle::Pop          => file.write(b"position, head = stack.pop()
turtle.up()
turtle.setposition(position)
turtle.setheading(head)
turtle.down()"),
        };

        match iores {
            Ok(_)  => (),
            Err(e) => panic!("file writing failed with error: {}", e)
        }
    }

    // The now-finished Tkinter canvas can be converted to a Postscript file.
    match file.write(
        format!("turtle.update()
turtle.getcanvas().postscript(file = '{}', colormode = 'color')
", eps_name).as_bytes()) {
        
        Ok(_)  => (),
        Err(e) => panic!("file writing failed with error: {}", e)
    }

    // Return the name of the script and the Postscript file it will create.
    (filename, eps_name)
}

#[derive(Clone)]
enum Penrose {
    F, N, M, O, P, Q, LBrace, RBrace, Plus, Minus
}

impl TurtleInterpretation for Penrose {
    fn to_turtle(&self) -> Turtle {
        match *self {
            Penrose::F      => Turtle::Forward(25),
            Penrose::LBrace => Turtle::Push,
            Penrose::RBrace => Turtle::Pop,
            Penrose::Plus   => Turtle::Right(36.0),
            Penrose::Minus  => Turtle::Left(36.0),
            _               => Turtle::Dummy
        }
    }
}

fn main() {
    use Penrose::{F, N, M, O, P, Q, LBrace, RBrace, Plus, Minus};
    
    // Axiom: [N]++[N]++[N]++[N]++[N]
    let s = LSystem::new(vec!(LBrace, N, RBrace, Plus, Plus,
                              LBrace, N, RBrace, Plus, Plus,
                              LBrace, N, RBrace, Plus, Plus,
                              LBrace, N, RBrace, Plus, Plus,
                              LBrace, N, RBrace), |x| match x {
        // M -> OA++PA----NA[-OA----MA]++
        M => vec!(O, F, Plus, Plus, P, F, Minus, Minus, Minus, Minus, N, F, LBrace, Minus, O, F,
                  Minus, Minus, Minus, Minus, M, F, RBrace, Plus, Plus),
        // N -> +OA--PA[---MA--NA]+
        N => vec!(Plus, O, F, Minus, Minus, P, F, LBrace, Minus, Minus, Minus, M, F, Minus, Minus,
                  N, F, RBrace, Plus),
        // O -> -MA++NA[+++OA++PA]-
        O => vec!(Minus, M, F, Plus, Plus, N, F, LBrace, Plus, Plus, Plus, O, F, Plus, Plus, P, F,
                  RBrace, Minus),
        // P -> --OA++++MA[+PA++++NA]--NA
        P => vec!(Minus, Minus, O, F, Plus, Plus, Plus, Plus, M, F, LBrace, Plus, P, F, Plus, Plus,
                  Plus, Plus, N, F, RBrace, Minus, Minus, N, F),
        // F -> Q
        F => vec!(Q),
        // All others are constants.
        c => vec!(c)
    });

    for (index, i) in s.skip(1).take(7).enumerate() {
        // "Draw" the current iteration by writing the Python script to produce a Postscript file.
        let (filename, eps_name) = draw(i, index);
        let png_name = format!("penrose-{}.png", index);

        // Run the actual script through Python, creating a Postscript file.
        let mut python = match Command::new("python").arg(filename.clone()).spawn() {
            Ok(python) => python,
            Err(e) => panic!("failed to execute python: {}", e),
        };

        match python.wait() {
            Err(e) => panic!("python failed with status {:?}", e),
            Ok(_)  => ()
        }

        // Run the Postscript file through ImageMagick to create a `.png`.
        let mut convert = match Command::new("convert").arg(eps_name.clone())
            .arg("-resize").arg("512x512").arg("-background").arg("black")
            .arg(png_name.clone()).spawn() {
            Ok(convert) => convert,
            Err(e) => panic!("failed to execute convert: {}", e),
        };

        match convert.wait() {
            Err(e) => panic!("convert failed with status {:?}", e),
            Ok(_)  => ()
        }

        // Delete the intermediate files.
        remove_file(filename).unwrap();
        remove_file(eps_name).unwrap();
    }

    // Stitch all of the `.png` files into a `.gif` using ImageMagick. This is equivalent to:
    // $ convert -delay 10 -loop 0 -dispose 3 *.png animation.gif
    let mut convert = match Command::new("convert").arg("-delay").arg("75").arg("-loop").arg("0")
        .arg("-dispose").arg("3").arg("*.png").arg("penrose.gif").spawn() {
        Ok(convert) => convert,
        Err(e) => panic!("failed to execute convert: {}", e),
    };

    match convert.wait() {
        Err(e) => panic!("convert failed with status {:?}", e),
        Ok(_)  => ()
    }
}
