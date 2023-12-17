mod condition;

use std::collections::HashSet;

pub use condition::*;
use crate::{Expression, expression::Variable};


pub struct Problem {
    pub(crate) optimize_type:      OptimizeType,
    pub(crate) objective_function: Expression,
    pub(crate) conditions:         Vec<Condition>,
}
#[derive(Debug, PartialEq)]
pub(crate) enum OptimizeType {
    Max,
    Min,
}

impl Problem {
    pub fn max(objective_function: Expression) -> Self {
        Self { optimize_type: OptimizeType::Max, objective_function, conditions: Vec::new() }
    }
    pub fn min(objective_function: Expression) -> Self {
        Self { optimize_type: OptimizeType::Min, objective_function, conditions: Vec::new() }
    }
    pub fn st(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn solve(self) {
        todo!()
    }
}

impl Problem {
    pub(crate) fn into_standard_form(mut self) -> Self {
        let mut conditions = Vec::with_capacity(self.conditions.len());
        for cond in self.conditions {
            let (standarized, slack) = cond.into_standarized();
            conditions.push(standarized);
            if let Some(slack) = slack {
                conditions.push(Condition::Inequality(
                    inequality(vec![(1., slack)], Sign::GE, 0.)
                ))
            }
        }

        if matches!(&self.optimize_type, OptimizeType::Min) {
            for (scalor, _) in &mut self.objective_function.terms {
                *scalor *= -1.;
            }
            self.optimize_type = OptimizeType::Max;
        }

        Self { conditions, ..self }
    }

    pub(crate) fn all_variables(&self) -> Vec<Variable> {
        let mut all_variables = HashSet::new();
        for condition in &self.conditions {
            for (_, var) in &condition.left().terms {
                if !all_variables.contains(var) {
                    all_variables.insert(var.clone());
                }
            }
        }
        all_variables.into_iter().collect()
    }
}
