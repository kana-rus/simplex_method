use std::ops::{Add, AddAssign};

use super::term::Term;


pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }
}

impl Add<Term> for Polynomial {
    type Output = Polynomial;
    fn add(mut self, new_term: Term) -> Self::Output {
        self.terms.push(new_term);
        self
    }
}
impl AddAssign<Term> for Polynomial {
    fn add_assign(&mut self, new_term: Term) {
        self.terms.push(new_term);
    }
}
