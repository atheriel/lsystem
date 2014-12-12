// pub struct Rule<T> {
//     predecessor: Vec<T>,
//     successor: Vec<T>
// }

// pub struct LSystem<T> {
//     alphabet: Vec<T>,
//     axiom: T,
//     rules: Vec<Rule<T>>
// }

pub struct LSystemType<T> {
    rules: fn(T) -> Vec<T>
}

impl<T> LSystemType<T> {
    pub fn from_fn(func: fn(T) -> Vec<T>) -> LSystemType<T> {
        LSystemType { rules: func }
    }

    pub fn recurse(&self, axiom: T) -> LSystemTypeIterator<T> {
        LSystemTypeIterator { current_state: vec!(axiom), rules: self.rules }
    }
}

pub struct LSystemTypeIterator<T> {
    current_state: Vec<T>,
    rules: fn(T) -> Vec<T>
}

impl<T: Clone> Iterator<Vec<T>> for LSystemTypeIterator<T> {
    fn next(&mut self) -> Option<Vec<T>> {
        let mut new_state: Vec<T> = Vec::new();
        for element in self.current_state.clone().into_iter() {
            let rules = self.rules;
            let entry = rules(element);
            new_state.push_all(entry.as_slice());
        }
        self.current_state = new_state;
        Some(self.current_state.clone())
    }
}

#[test]
fn it_works() {
}
