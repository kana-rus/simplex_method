use std::collections::HashMap;

use crate::{components::{Scalor, Variable, Polynomial}, problem::StandarndFormProblem};


pub struct Table {
    objective_row: ObjectiveRow,
    base_rows:     Vec<BaseRow>,
}

struct ObjectiveRow {
    value:        Scalor,
    coefficients: HashMap<Variable, Scalor>,
}

struct BaseRow {
    base:         Variable,
    value:        Scalor,
    coefficients: HashMap<Variable, Scalor>,
}


impl Table {
    pub fn from_standard_form_problem(problem: StandarndFormProblem) -> Self {
        let StandarndFormProblem { objective_function, condition } = problem;

        let x = || condition.x.clone();

        let objective_row = ObjectiveRow {
            value: 0.,
            coefficients: HashMap::from_iter(
                x().into_iter()
                    .map(|var| (
                        var.clone(),
                        objective_function.terms.iter()
                            .find(|term| term.variable == var)
                            .map_or(0., |term| term.coefficient)
                    ))),
        };

        TODO
    }
}
