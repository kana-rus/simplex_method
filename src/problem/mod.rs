#![allow(non_snake_case)]
#[cfg(test)] mod test;

use crate::components::{polynomial::Polynomial, matrix::Matrix, scalor::Scalor, variable::Variable};


#[derive(Debug, PartialEq)]
pub struct Problem {
    pub(crate) objective_function: Polynomial,
    pub(crate) condition:          Condition,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub(crate) A:    Matrix<Scalor>,
    pub(crate) x:    Vec<Variable>,
    pub(crate) sign: Sign,
    pub(crate) b:    Vec<Scalor>,
}

#[derive(PartialEq)]
pub(crate) enum Sign { EQ, LE }
impl std::fmt::Debug for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EQ => "=",
            Self::LE => "≤",
        })
    }
}

impl Problem {
    pub(crate) fn into_standard_form(self) -> Problem {
        if self.is_standard_form() {return self}

        let Problem { objective_function, condition } = self;
        Problem { objective_function, condition:condition.into_standard_form() }
    }

    fn is_standard_form(&self) -> bool {
        matches!(self.condition.sign, Sign::EQ)
    }
}
impl Condition {
    pub(crate) fn into_standard_form(self) -> Condition {
        let Condition { A, x,  b, .. } = self;

        let slack_variables = (1..=b.len()).into_iter()
            .map(|i| Variable::Slack { id: i }).collect::<Vec<Variable>>();

        Condition {
            A: A.try_concat(Matrix::identity(b.len())).unwrap(/* `b.len()` equals to `A.column_size` */),
            x: [x, slack_variables].concat(),
            sign: Sign::EQ,
            b
        }
    }
}

impl Problem {
    pub fn maximize(objective_function: impl Into<Polynomial>, condition: Condition) -> Self {
        Self {
            objective_function: objective_function.into(),
            condition,
        }
    }
}
impl Condition {
    /// `\forall i, Ax_i ≤ b_i`
    pub fn each_le(A: Matrix<Scalor>, x: Vec<Variable>, b: Vec<impl Into<Scalor>>) -> Self {
        Self {
            A, x,
            sign: Sign::LE,
            b:b.into_iter().map(Into::into).collect(),
        }
    }
}
