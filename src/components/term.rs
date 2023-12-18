use std::ops::Mul;

use super::{scalor::Scalor, variable::Variable};


#[derive(Debug, PartialEq)]
pub struct Term {
    pub coefficient: Scalor,
    pub variable:    Variable,
}

impl Mul<Variable> for Scalor {
    type Output = Term;
    fn mul(self, variable: Variable) -> Self::Output {
        Term { coefficient:self, variable }
    }
}
impl Mul<&Variable> for Scalor {
    type Output = Term;
    fn mul(self, variable: &Variable) -> Self::Output {
        Term { coefficient:self, variable:variable.clone() }
    }
}
