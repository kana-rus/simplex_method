use std::collections::HashMap;

use crate::expression::{Variable, Expression};
use crate::problem::{Problem, OptimizeType, Condition, Inequality, Equation};
use crate::{Scalor};


pub struct SimplexTable {
    slack_rows:   Vec<SimplexRow>,
    object_row:   SimplexObjectRow,
    inequalities: Vec<Inequality>,
}

struct SimplexRow {
    base:         Variable,
    base_value:   Scalor,
    coefficients: HashMap<Variable, Scalor>,
    inc_limit:    Scalor,
}
struct SimplexObjectRow {
    value:        Scalor,
    coefficients: HashMap<Variable, Scalor>,
}

impl SimplexTable {
    pub(crate) fn from_problem(problem: Problem) -> Self {
        /* assert that `problem` is pre-proccessed */
        assert_eq!(problem.optimize_type, OptimizeType::Max);

        #[allow(non_snake_case)]
        let ALL_VARIABLES = problem.all_variables();

        let mut table = Self {
            slack_rows:   Vec::new(),
            inequalities: Vec::new(),
            object_row:   SimplexObjectRow {
                value:        0.,
                coefficients: {
                    let mut c = HashMap::new();
                    for v in &ALL_VARIABLES {
                        c.insert(
                            v.clone(),
                            problem.objective_function.terms.into_iter()
                                .find(|(scalor, var)| var == v)
                                .map_or(0., |(scalor, _)| scalor),
                        );
                    }
                    c
                }
            },
        };
        for condition in problem.conditions {
            match condition {
                Condition::Inequality(i) => table.inequalities.push(i),
                Condition::Equation(Equation { left, right }) => {
                    let row = SimplexRow {
                        base:         ,
                        base_value:   ,
                        coefficients: ,
                        inc_limit:    ,
                    };
                }
            }
        }
        table
    }
}
