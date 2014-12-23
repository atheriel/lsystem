//! A simple crate for creating L-System iterators using types as an alphabet.
//! See the documentation for the [`lsystem_iter()`](fn.lsystem_iter.html)
//! function for details.

/// Create an iterator yielding successive iterations over the Lindenmayer
/// System defined by an axiom of type `T`, a rule function which maps values
/// of type `T` to vectors of values of type `T`, and the set of all possible
/// values of type `T`.
///
/// Formally, an [L-System](http://en.wikipedia.org/wiki/l_system) consists of
/// three things:
///
/// 1. An alphabet of letters
/// 2. An axiom composed of letters of this alphabet; and
/// 3. A set of "production" rules for transforming sets of letters into one
///    another.
///
/// This iterator satisfies these requirements by taking the universe of the
/// values of type `T` as the alphabet, one specific value of type `T` as the
/// axiom, and a function `T -> Vec<T>` as handling any transformations. This
/// is really just a way of using Rust's type system to express a formal
/// grammar in a very concise way. And while there's no reason one could not
/// use regular types (like `int` or `&str`) here, this method really comes
/// into its own through the use of `enum`s.
///
/// For example, consider the [algae theory of Lindenmayer]
/// (http://www.math.ubc.ca/~cass/courses/m308-03b/projects-03b/skinner/lindenmayer.htm),
/// which can be expressed as follows:
///
/// ```rust
/// use lsystem::lsystem_iter; 
///
/// // We represent Lindenmayer's states of algae cells with a type.
/// #[deriving(Clone, Show, Eq, PartialEq)]
/// enum Algae {
///     A, // Reproduction State
///     B  // Growth State
/// }
///
/// // Then define all of the production rules in a single function.
/// fn algae_rule(input: Algae) -> Vec<Algae> {
///     match input {
///         Algae::A => vec![Algae::A, Algae::B],
///         Algae::B => vec![Algae::A]
///     }
/// }
///
/// // The object returned by this function is just a normal iterator, so we
/// // can get the fifth item (which is the n = 4 iteration) in an intuitive
/// // and idiomatic way:
/// let algae_lsystem_n4 = lsystem_iter(Algae::B, algae_rule).skip(4).next();
///
/// // And confirm that it matches Lindenmayer's fourth iteration.
/// assert_eq!(algae_lsystem_n4,
///            Some(vec!(Algae::A, Algae::B, Algae::A, Algae::A, Algae::B)))
/// ```
// pub fn lsystem_iter<T>(axiom: T, rules: fn(T) -> Vec<T>) -> LSystemIterator<T> {
//     LSystemIterator { current_state: vec!(axiom), rules: rules, zeroth: true }
// }

pub type ProductionRule<T> = fn(T) -> Vec<T>;

pub struct LSystemType<'a, T: Clone> {
    axiom: T,
    rules: &'a ProductionRule<T>
}

impl<'a, T: Clone> LSystemType<'a, T> {
    pub fn new(axiom: T, rules: &'a ProductionRule<T>) -> LSystemType<'a, T> {
        LSystemType { axiom: axiom, rules: rules }
    }

    pub fn iter(&self) -> LSystemIterator<'a, T> {
        LSystemIterator {
            current_state: vec!(self.axiom.clone()),
            rules: self.rules,
            zeroth: true
        }
    }
}

/// Defines an iterator over an L-System, where each successive iteration
/// applies a series of rules to the current axiom to produce a new axiom.
pub struct LSystemIterator<'a, T: Clone> {
    current_state: Vec<T>,
    rules: &'a ProductionRule<T>,
    zeroth: bool
}

impl<'a, T: Clone> Iterator<Vec<T>> for LSystemIterator<'a, T> {
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
            let rules = *self.rules;
            let entry = rules(element);
            new_state.push_all(entry.as_slice());
        }
        self.current_state = new_state;
        Some(self.current_state.clone())
    }
}
