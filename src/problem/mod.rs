mod condition;

use condition::*;
use crate::{Expression};


pub struct Problem {
    optimize_type:      OptimizeType,
    objective_function: Expression,
    conditions:         Vec<Condition>,
}
enum OptimizeType {
    Max,
    Min,
}

impl Problem {
    pub fn into_standard_form(self) -> Self {
        let mut conditions = Vec::with_capacity(self.conditions.len());
        for cond in self.conditions {
            let (standarized, slack) = cond.standarized();
            conditions.push(standarized);
            if let Some(slack) = slack {
                conditions.push(Condition::Inequality(
                    inequality(vec![(1, slack)], Sign::GE, 0)
                ))
            }
        }

        let (mut optimize_type, mut objective_function) = (self.optimize_type, self.objective_function);
        if matches!(optimize_type, OptimizeType::Min) {
            for (scalor, _) in &mut objective_function.terms {
                *scalor *= -1;
            }
            optimize_type = OptimizeType::Max;
        }

        __TODO__

        Self { optimize_type, objective_function, conditions }
    }
}
