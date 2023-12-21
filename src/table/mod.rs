#[cfg(test)] mod test;

use std::collections::HashMap;
use crate::problem::Problem;
use crate::components::{variable::Variable, scalor::Scalor, matrix::Matrix};


#[derive(PartialEq)]
pub struct Table {
    variables:    Vec<Variable>,      // This's index is just the column number of `coefficients`
    bases:        Vec<BaseVariable>,  // This's index is just the row number of `coefficients`
    coefficients: Matrix<Scalor>,
}

#[derive(PartialEq)]
struct BaseVariable {
    variable: Variable,
    value:    Scalor,
}

#[derive(Debug, PartialEq)]
struct Pivot {
    row:    usize,
    column: usize,
    value:  Scalor,
}

#[derive(Debug, PartialEq)]
pub struct Solution {
    pub variables:     HashMap<Variable, Scalor>,
    pub optimal_value: Scalor,
}

impl Table {
    pub fn from_problem(problem: Problem) -> Self {
        let Problem { objective_function, condition } = problem.into_standard_form();

        let variables = condition.x;

        let bases = {
            let slack_variables = variables.clone().into_iter()
                .skip_while(Variable::is_normal);
            let mut bases = slack_variables.zip(condition.b)
                .map(|(variable, value)| BaseVariable { variable, value })
                .collect::<Vec<_>>();
            bases.push(BaseVariable {
                variable: Variable::Object,
                value:    0.,
            });
            bases
        };

        let coefficients = {
            let mut coefficients = condition.A;

            let objective_coefficients = variables.clone().into_iter()
                .map(|var| objective_function.terms.iter()
                    .find(|term| term.variable == var)
                    .map_or(0., |term| -term.coefficient)
                ).collect();

            coefficients.push_row(objective_coefficients).unwrap();
            coefficients
        };

        Self { variables, bases, coefficients }
    }

    pub fn solve(mut self) -> Result<Solution, String> {
        const UPDATE_LIMIT: usize = 5;
        let mut update_count = 0;

        #[cfg(test)] println!("{self:?}");

        while !self.is_optimal() {
            let p = self.pivot();
            self.bases[p.row].variable = self.variables[p.column].clone();
            self.update_coefficients(p);

            update_count += 1;
            if update_count == UPDATE_LIMIT {
                return Err((|| format!("Reaeched UPDATE_LIMIT"))())
            }

            #[cfg(test)] println!("{self:?}");
        }

        Ok(Solution {
            optimal_value: self.object_value(),
            variables:     HashMap::from_iter(
                self.bases.into_iter()
                    .filter_map(|BaseVariable { variable, value }| variable.is_normal().then(|| (variable, value)))
            ),
        })
    }
}

impl Table {
    fn object_value(&self) -> Scalor {
        self.bases.last().unwrap().value
    }

    fn criterions(&self) -> &Vec<Scalor> {
        self.coefficients.rows.last().unwrap()
    }

    fn is_optimal(&self) -> bool {
        self.criterions().iter().all(|&c| c >= 0.)
    }

    /// Search
    /// 
    /// - Row index where the max-increase is minimum
    /// - Column index where the simplex criterion is minimum
    /// 
    /// and returns `(row index, column index)`\
    /// e.t. `(pivot row index, pivot column index)`
    fn pivot(&self) -> Pivot {
        let (min_criterion_coloumn, _) = self.criterions().iter()
            .enumerate()
            .reduce(|(min_criterion_coloumn, min), (i, c)| {
                if c < &min {(i, c)} else {(min_criterion_coloumn, min)}
            }).unwrap();
        let max_increases = self.coefficients.column_iter(min_criterion_coloumn).unwrap()
            .enumerate()
            .take_while(|(i, _)| *i < self.coefficients.column_size - 1)
            .map(|(i, c)| self.bases[i].value / c);
        let (min_maxinc_row, _) = max_increases
            .enumerate()
            .reduce(|(min_maxinc_row, min_maxinc), (i, maxinc)|
                if maxinc < min_maxinc {(i, maxinc)} else {(min_maxinc_row, min_maxinc)}
            ).unwrap();
        
        Pivot {
            row:    min_maxinc_row,
            column: min_criterion_coloumn,
            value:  self.coefficients[min_maxinc_row][min_criterion_coloumn],
        }
    }

    /// Update table by pivot operation.
    /// 
    /// **NOTE**：Here *table* is consist of coefficients and **the column of base variables' values**.
    fn update_coefficients(&mut self, pivot: Pivot) {
        /* Divide all values in pivot row by pivot value */
        self.bases[pivot.row].value /= pivot.value;
        for c in &mut self.coefficients[pivot.row] {
            *c /= pivot.value
        }

        /* Add multiple of pivot row to other rows so that their value at pivot column be 0 */
        for i in (0..pivot.row).chain((pivot.row + 1)..(self.coefficients.column_size)) {
            let rate = self.coefficients[i][pivot.column];
            let cofficients_pivot_row = self.coefficients[pivot.row].clone();

            self.bases[i].value -= self.bases[pivot.row].value * rate;
            for (i, c) in self.coefficients[i].iter_mut().enumerate() {
                *c -= cofficients_pivot_row[i] * rate;
            }
        }
    }
}


const _: () = {
    impl std::fmt::Debug for Table {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let base_var_maxwidth = self.bases.iter()
                .map(|BaseVariable { variable, .. }| format!("{variable:?}").len())
                .reduce(usize::max).unwrap_or(0);
            let base_value_maxwidth = self.bases.iter()
                .map(|BaseVariable { value, .. }| format!("{value:?}").len())
                .reduce(usize::max).unwrap_or(0);

            let variable_maxwidths = self.variables.iter()
                .map(|v| format!("{v:?}").len());
            let coefficient_column_maxwidths = {
                let widths = self.coefficients.rows.iter()
                    .map(|row| row.iter()
                        .map(|c| format!("{c:?}").len())
                        .collect::<Vec<_>>()
                    ).collect::<Vec<_>>();

                let mut maxwidth_by_column = Vec::with_capacity(self.coefficients.row_size);
                for x in 0..self.coefficients.row_size {
                    maxwidth_by_column.push(
                        (0..self.coefficients.column_size)
                            .map(|y| widths[y][x])
                            .reduce(usize::max).unwrap_or(0)
                    )
                }
                maxwidth_by_column
            };

            let base_info_width = base_var_maxwidth + 3/* ` = ` */ + base_value_maxwidth;
            let column_widths   = variable_maxwidths.zip(coefficient_column_maxwidths)
                .map(|(v, c)| usize::max(v, c))
                .collect::<Vec<_>>();

            f.write_str("\n")?;

            f.write_str(&" ".repeat(base_info_width))?;
            f.write_str("  ")?;
            for (i, var) in self.variables.iter().enumerate() {
                let fmt   = format!("{var:?}");
                let width = fmt.len();
                f.write_str(&(fmt + &" ".repeat(column_widths[i] - width)))?;
                f.write_str("  ")?;
            }
            f.write_str("\n")?;

            for i in 0..self.bases.len() {
                let (base, coefficients) = (&self.bases[i], &self.coefficients[i]);

                let base_fmt = {
                    let mut fmt = String::with_capacity(base_info_width);

                    let var_fmt = format!("{:?}", base.variable);
                    fmt.push_str(&var_fmt);
                    fmt.push_str(&" ".repeat(base_var_maxwidth - var_fmt.len()));

                    fmt.push_str(" = ");

                    let value_fmt = format!("{:?}", base.value);
                    fmt.push_str(&value_fmt);
                    fmt.push_str(&" ".repeat(base_value_maxwidth - value_fmt.len()));

                    fmt
                };
                f.write_str(&base_fmt)?;

                f.write_str("  ")?;

                for (i, c) in coefficients.iter().enumerate() {
                    let fmt = format!("{c:?}");
                    f.write_str(&fmt)?;
                    f.write_str(&" ".repeat(column_widths[i] - fmt.len()))?;
                    f.write_str("  ")?;
                }

                f.write_str("\n")?;
            }

            Ok(())
        }
    }
};
