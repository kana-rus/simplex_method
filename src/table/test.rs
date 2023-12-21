use std::collections::HashMap;
use crate::{Problem, Condition, var, matrix, components::variable::{slack, Variable}, problem::Sign};
use super::{Table, BaseVariable, Pivot, Solution};


#[test] fn test_simplex_method() {
    let problem = Problem::maximize(4*var("x1") + 5*var("x2"),
        Condition::each_le(
            matrix! {
                2, 5
                6, 4
                3, 1
            },
            vec![
                var("x1"),
                var("x2"),
            ],

            /* less than or equals to */

            vec![
                20,
                27,
                12,
            ]
        )
    ).into_standard_form();
    assert_eq!(problem, Problem {
        objective_function: 4*var("x1") + 5*var("x2"),
        condition: Condition {
            A: matrix! {
                2, 5, 1, 0, 0
                6, 4, 0, 1, 0
                3, 1, 0, 0, 1
            },
            x: vec![
                var("x1"),
                var("x2"),
                slack(1),
                slack(2),
                slack(3),
            ],

            sign: Sign::EQ,

            b: vec![
                20.,
                27.,
                12.,
            ]
        }
    });

    let mut table = Table::from_problem(problem.into_standard_form());

    assert_eq!(table, Table {
        variables: vec![
            var("x1"),
            var("x2"),
            slack(1),
            slack(2),
            slack(3),
        ],
        bases: vec![
            BaseVariable { variable:slack(1), value:20. },
            BaseVariable { variable:slack(2), value:27. },
            BaseVariable { variable:slack(3), value:12. },
            BaseVariable { variable:Variable::Object, value:0. },
        ],
        coefficients: matrix! {
             2,  5, 1, 0, 0
             6,  4, 0, 1, 0
             3,  1, 0, 0, 1
            -4, -5, 0, 0, 0
        },
    });
    assert_eq!(table.is_optimal(), false);
    assert_eq!(table.pivot(), Pivot {
        
    });

    // assert_eq!(table.solve().unwrap(), Solution {
    //     variables:     HashMap::from([
    //         (var("x1"), 2.5),
    //         (var("x2"), 3.),
    //     ]),
    //     optimal_value: 25.,
    // });
}
