mod variable;

pub use variable::{new_slack, Variable};
use crate::Scalor;


#[derive(Debug, PartialEq)]
pub struct Expression {
    pub terms: Vec<(Scalor, Variable)>,
} impl Expression {
    pub fn from_iter(terms_iter: impl IntoIterator<Item = (Scalor, Variable)>) -> Self {
        Self { terms: terms_iter.into_iter()
            .map(|(i, var)| (i.into(), var))
            .collect() }
    }
} const _: () = {
    impl From<(Scalor, Variable)> for Expression {
        fn from(value: (Scalor, Variable)) -> Self {
            Self { terms: vec![value] }
        }
    }
};
