//! A library for creating and iterating over [L-Systems](http://en.wikipedia.org/wiki/l_system)
//! using types as alphabets.
//!
//! ## A Simple Example
//!
//! Consider the [algae theory of Lindenmayer]
//! (http://www.math.ubc.ca/~cass/courses/m308-03b/projects-03b/skinner/lindenmayer.htm). He
//! suggests that algae cells can be in one of two states: reproduction (usually `A`) or growth
//! (usually `B`). We can represent these states with a type:
//!
//! ```ignore
//! #[derive(Clone, Eq, Debug, PartialEq)]
//! enum Algae {
//!     #[doc = "Reproduction State"] A,
//!     #[doc = "Growth State"] B
//! }
//! ```
//!
//! Algae cells that are in the growth state transition to being in the reproductive state after a
//! period of time. In this time, a cell in the reproductive state will birth a new cell, which
//! starts in the growth state.  These two processes can be represented using a simple function:
//!
//! ```ignore
//! fn algae_rule(input: Algae) -> Vec<Algae> {
//!     match input {
//!         Algae::A => vec!(Algae::A, Algae::B),
//!         Algae::B => vec!(Algae::A)
//!     }
//! }
//! ```
//!
//! This function actually has a very specific signature, which is worth keeping in mind.  Also,
//! notice that it applies to each cell individually, regardless of the state of affairs of the
//! other cells around it. For this reason, this kind of L-System can be called a "context-free"
//! grammar.
//!
//! The <abbr>API</abbr> for creating and iterating over an L-System &mdash; given the two
//! components above &mdash; is actually quite simple. The fifth iteration (i.e. `n = 4`) of
//! Lindenmayer's algae L-System is ABAAB, which we can confirm as follows:
//!
//! ```rust
//! use lsystem::LSystem;
//! # #[derive(Clone, Eq, Debug, PartialEq)]
//! # enum Algae { A, B }
//! # fn algae_rule(input: Algae) -> Vec<Algae> {
//! #     match input {
//! #         Algae::A => vec!(Algae::A, Algae::B),
//! #         Algae::B => vec!(Algae::A)
//! #     }
//! # }
//!
//! let mut algae_lsystem = LSystem::new(vec!(Algae::B), algae_rule);
//!
//! // LSystem implements the normal iterator trait, so to get the fifth
//! // item (which is the n = 4 iteration) we use the following idiom:
//! let algae_lsystem_n4 = algae_lsystem.nth(4).unwrap();
//!
//! // And confirm that it matches Lindenmayer's fifth iteration.
//! assert_eq!(algae_lsystem_n4,
//!            vec!(Algae::A, Algae::B, Algae::A, Algae::A, Algae::B))
//! ```

// Currently required for using `drain`.
#![feature(collections)]

/// Create the Lindenmayer System defined by an axiom of type `Vec<T>`, a rule function (or
/// closure) which maps values of type `T` to vectors of values of type `T`, and the set of all
/// possible values of type `T`.
///
/// Formally, an [L-system](http://en.wikipedia.org/wiki/l_system) consists of three things:
///
/// 1. An alphabet of letters
/// 2. An axiom composed of letters of this alphabet; and
/// 3. A set of "production" rules for transforming sets of letters into one another.
///
/// This definition satisfies these requirements by taking the universe of the values of type `T`
/// as the alphabet, one specific vector of values of type `T` as the axiom, and a function or
/// closure of type `FnMut(T) -> Vec<T>` for handling any transformations. This is really just a
/// way of using Rust's type system to express a formal grammar in a very concise way. And while
/// there's no reason one could not use regular types (like `int` or `&str`) here, this method
/// really comes into its own through the use of `enum`s.
///
/// Since this type implements the normal iterator trait, it can be used in many idiomatic ways.
/// But be warned: the iterator will never be exhausted, so any loops must be broken manually.
pub struct LSystem<T, F: FnMut(T) -> Vec<T>> {
    axiom: Vec<T>,
    rules: F,
    zeroth: bool
}

impl<T, F> LSystem<T, F> where F: FnMut(T) -> Vec<T> {
    /// Creates a new representation of an L-system with the given axiom and production rules.
    pub fn new(axiom: Vec<T>, rules: F) -> LSystem<T, F> {
        LSystem { axiom: axiom, rules: rules, zeroth: true }
    }
}

impl<T, F> Iterator for LSystem<T, F> where T: Clone, F: FnMut(T) -> Vec<T> {
    type Item = Vec<T>;

    /// Yield the next iteration of the L-system by rewriting the current axiom's contents using
    /// the production rules.
    fn next(&mut self) -> Option<Vec<T>> {
        // In order to ensure that the "n = 0" case returns the original axiom, store whether we
        // are in this state or not.
        if self.zeroth {
            self.zeroth = false;
            return Some(self.axiom.clone())
        }

        // Otherwise, apply the production rules to the axiom to produce a new axiom for the
        // iteration level.
        let mut new_axiom = Vec::new();
        for element in self.axiom.drain() {
            new_axiom.extend((self.rules)(element).into_iter());
        }
        self.axiom = new_axiom;
        Some(self.axiom.clone())
    }
}

pub mod turtle {

    /// Note that this is only a small subset of more complete Turtle graphics implementations.
    #[derive(Copy, Clone)]
    pub enum Turtle {
        Forward(u32), Left(f32), Right(f32), Push, Pop, Dummy
    }

    pub trait TurtleInterpretation {
        fn to_turtle(&self) -> Turtle;
    }

    pub fn draw<T: TurtleInterpretation>(v: Vec<T>) {
        println!("import turtle\n\nturtle.speed(0)\n");

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
}
