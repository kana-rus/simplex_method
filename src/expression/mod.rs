mod variable;
mod scalor;

pub use variable::{var, new_slack, Variable};
pub use scalor  ::{Scalor};


pub struct Expression {
    pub terms: Vec<(Scalor, Variable)>,
} impl Expression {
    pub fn from_iter(terms_iter: impl IntoIterator<Item = (Scalor, Variable)>) -> Self {
        Self { terms: terms_iter.into_iter()
            .map(|(i, var)| (i.into(), var))
            .collect() }
    }
}
