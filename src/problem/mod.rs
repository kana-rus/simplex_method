#![allow(non_snake_case)]

use crate::components::{polynomial::Polynomial, matrix::Matrix, scalor::Scalor, variable::Variable};


pub struct GeneralFormProblem {
    objective_function: Polynomial,
    condition:          GeneralFormCondition,
}

/// `A x <= b`
pub struct GeneralFormCondition {
    A: Matrix<Scalor>,
    x: Vec<Variable>,
    b: Vec<Scalor>,
}

pub struct StandarndFormProblem {
    pub objective_function: Polynomial,
    pub condition:          StandardFormCondition,
}

/// `A x = b`
pub struct StandardFormCondition {
    pub A: Matrix<Scalor>,
    pub x: Vec<Variable>,
    pub b: Vec<Scalor>,
}


impl GeneralFormProblem {
    pub(crate) fn into_standard_form(self) -> StandarndFormProblem {
        let GeneralFormProblem { objective_function, condition } = self;
        StandarndFormProblem { objective_function, condition:condition.into_standard_form() }
    }
}
impl GeneralFormCondition {
    pub(crate) fn into_standard_form(self) -> StandardFormCondition {
        let GeneralFormCondition { A, x, b } = self;

        let slack_variables = (1..=b.len()).into_iter()
            .map(|i| Variable::Slack { name: format!("s{i}") }).collect::<Vec<Variable>>();

        StandardFormCondition {
            A: A.try_concat(Matrix::identity(b.len())).unwrap(/* `b.len()` equals to `A.column_size` */),
            x: [x, slack_variables].concat(),
            b
        }
    }
}

impl GeneralFormProblem {
    pub fn maximize(objective_function: impl Into<Polynomial>, condition: GeneralFormCondition) -> Self {
        Self {
            objective_function: objective_function.into(),
            condition,
        }
    }
}
impl GeneralFormCondition {
    /// `\forall i, Ax_i <= b_i`
    pub fn each_le(A: Matrix<Scalor>, x: Vec<Variable>, b: Vec<impl Into<Scalor>>) -> Self {
        Self { A, x, b:b.into_iter().map(Into::into).collect() }
    }
}
