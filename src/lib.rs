//! A simple crate for creating L-System iterators using types as an alphabet.
//! See the documentation for the [`lsystem_iter()`](fn.lsystem_iter.html)
//! function for details.

/// Create an iterator yielding successive iterations over the Lindenmayer
/// Gammar defined by an axiom of type `T`, a rule function which maps values
/// of type `T` to vectors of values of type `T`, and the set of all possible
/// values of type `T`.
///
/// Formally, an L-System consists of three things: an alphabet, an axiom
/// composed of letters of this alphabet, and a set of rules for transforming
/// one set of letters into another. This iterator satisfies these requirements
/// by taking the universe of the values of type `T` as the alphabet, one
/// specific value of type `T` as the axiom, and a function `T -> Vec<T>` as
/// handling any transformations. This is really just a way of using Rust's
/// type system to express a formal grammar in a very concise way. For example,
/// consider the [algae theory of Lindenmayer]
/// (http://www.math.ubc.ca/~cass/courses/m308-03b/projects-03b/skinner/lindenmayer.htm),
/// which can be expressed as follows:
///
/// ```rust
/// extern crate lsystem;
///
/// use lsystem::lsystem_iter;
///
/// #[deriving(Clone)]
/// enum AlgeaState {
///     Reproduction,
///     Growth,
/// }
///
/// fn algae_rule(input: AlgeaState) -> Vec<AlgeaState> {
///     match input {
///         AlgeaState::Reproduction => vec![AlgeaState::Reproduction,
///                                          AlgeaState::Growth],
///         AlgeaState::Growth => vec![AlgeaState::Reproduction]
///     }
/// }
///
/// fn main() {
///     // Print out the first eight levels of the Algae sequence in the same
///     // format as in the Wikipedia article.
///     for (index, n) in lsystem_iter(AlgeaState::Growth, algae_rule).
///                       take(8).enumerate() {
///         let mut printed = format!("n = {}: ", index);
///         for i in n.iter() {
///             match i {
///                 &AlgeaState::Reproduction => printed.push_str("A"),
///                 &AlgeaState::Growth => printed.push_str("B")
///             }
///         }
///         println!("{}", printed)
///     }
/// }
/// ```
pub fn lsystem_iter<T>(axiom: T, rules: fn(T) -> Vec<T>) -> LSystemIterator<T> {
    LSystemIterator { current_state: vec!(axiom), rules: rules, zeroth: true }
}

/// Defines an iterator over an L-System, where each successive iteration
/// applies a series of rules to the current axiom to produce a new axiom.
pub struct LSystemIterator<T> {
    current_state: Vec<T>,
    rules: fn(T) -> Vec<T>,
    zeroth: bool
}

impl<T: Clone> Iterator<Vec<T>> for LSystemIterator<T> {
    fn next(&mut self) -> Option<Vec<T>> {
        // In order to ensure that the "n = 0" case returns the original axiom,
        // store whether we are in this state or not.
        if self.zeroth {
            self.zeroth = false;
            return Some(self.current_state.clone())
        }

        // Otherwise, apply the production rules to the axiom to produce a new
        // axiom for the iteration level.
        let mut new_state: Vec<T> = Vec::new();
        for element in self.current_state.iter().cloned() {
            let rules = self.rules;
            let entry = rules(element);
            new_state.push_all(entry.as_slice());
        }
        self.current_state = new_state;
        Some(self.current_state.clone())
    }
}
