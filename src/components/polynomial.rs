use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::term::Term;


pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }
}

const _: () = {
    impl From<Term> for Polynomial {
        fn from(term: Term) -> Self {
            Self { terms: vec![term] }
        }
    }

    impl Add<Term> for Term {
        type Output = Polynomial;
        fn add(self, another: Term) -> Self::Output {
            Polynomial { terms: vec![self] } + another
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

    impl Sub<Term> for Term {
        type Output = Polynomial;
        fn sub(self, another: Term) -> Self::Output {
            Polynomial { terms: vec![self] } - another
        }
    }
    impl Sub<Term> for Polynomial {
        type Output = Polynomial;
        fn sub(mut self, Term { coefficient, variable }: Term) -> Self::Output {
            self.terms.push(Term { coefficient:-coefficient, variable });
            self
        }
    }
    impl SubAssign<Term> for Polynomial {
        fn sub_assign(&mut self, Term { coefficient, variable }: Term) {
            self.terms.push(Term { coefficient:-coefficient, variable });
        }
    }
};
