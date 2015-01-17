//! A library for creating and iterating over
//! [L-Systems](http://en.wikipedia.org/wiki/l_system) defined using types as
//! an alphabet.
//!
//! ## A Simple Example
//!
//! Consider the [algae theory of Lindenmayer]
//! (http://www.math.ubc.ca/~cass/courses/m308-03b/projects-03b/skinner/lindenmayer.htm).
//! He suggests that algae cells can be in one of two states: reproduction (A)
//! or growth (B). We can represent these states with a type:
//!
//! ```ignore
//! #[deriving(Clone, Show, Eq, PartialEq)]
//! enum Algae {
//!     #[doc = "Reproduction State"] A,
//!     #[doc = "Growth State"] B
//! }
//! ```
//!
//! Algae cells that are in the growth state transition to being in the
//! reproductive state after a period of time. In this time, a cell in the
//! reproductive state will birth a new cell, which starts in the growth state.
//! These two processes can be represented using a simple function:
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
//! This function actually has a very specific form: it matches the definition
//! of the [`ProductionRule<Algae>`](./type.ProductionRule.html) type. Notice
//! that it applies to each cell individually, regardless of the state of
//! affairs of the other cells around it. For this reason, this kind of
//! L-System can be called a "context-free" grammar.
//!
//! The <abbr>API</abbr> for creating and iterating over an L-System &mdash;
//! given the two components above &mdash; is actually quite simple. The fifth
//! iteration (i.e. `n = 4`) of Lindenmayer's algae L-System is ABAAB, which we
//! can confirm as follows:
//!
//! ```ignore
//! use lsystem::LSystemType;
//! # #[deriving(Clone, Show, Eq, PartialEq)]
//! # enum Algae { A, B }
//! # fn algae_rule(input: Algae) -> Vec<Algae> {
//! #     match input {
//! #         Algae::A => vec!(Algae::A, Algae::B),
//! #         Algae::B => vec!(Algae::A)
//! #     }
//! # }
//!
//! let algae_lsystem = LSystemType::new(vec!(Algae::B), algae_rule);
//!
//! // The iter() method returns a normal Rust iterator, so to get the fifth
//! // item (which is the n = 4 iteration) we use the following idiom:
//! let algae_lsystem_n4 = algae_lsystem.iter().nth(4).unwrap();
//!
//! // And confirm that it matches Lindenmayer's fifth iteration.
//! assert_eq!(algae_lsystem_n4,
//!            vec!(Algae::A, Algae::B, Algae::A, Algae::A, Algae::B))
//! ```

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
/// ## Iterating over the L-System
///
/// The [`iter()`](#method.iter) method returns a Rust iterator that yields successive iterations
/// of the L-System. This allows very idiomatic handling of iteration, but be warned: the iterator
/// will never be exhausted, so any loops must be broken manually.
pub struct LSystemType<T, F: FnMut(T) -> Vec<T>> {
    axiom: Vec<T>,
    rules: F
}

impl<T, F> LSystemType<T, F> where F: FnMut(T) -> Vec<T> {
    /// Creates a new representation of an L-system with the given axiom and production rules.
    pub fn new(axiom: Vec<T>, rules: F) -> LSystemType<T, F> {
        LSystemType { axiom: axiom, rules: rules }
    }

    /// Consumes the `LSystemType` to produce an iterator.
    pub fn iter(self) -> LSystemIterator<T, F> {
        LSystemIterator { current_state: self.axiom, rules: self.rules, zeroth: true }
    }
}

/// Defines an iterator over an L-System, where each successive iteration applies a series of rules
/// to the current axiom to produce a new axiom.
pub struct LSystemIterator<T, F: FnMut(T) -> Vec<T>> {
    current_state: Vec<T>,
    rules: F,
    zeroth: bool
}

impl<T, F> Iterator for LSystemIterator<T, F> where T: Clone, F: FnMut(T) -> Vec<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        // In order to ensure that the "n = 0" case returns the original axiom, store whether we
        // are in this state or not.
        if self.zeroth {
            self.zeroth = false;
            return Some(self.current_state.clone())
        }

        // Otherwise, apply the production rules to the axiom to produce a new axiom for the
        // iteration level.
        let mut new_state = Vec::new();
        for element in self.current_state.drain() {
            new_state.extend((self.rules)(element).into_iter());
        }
        self.current_state = new_state;
        Some(self.current_state.clone())
    }
}
