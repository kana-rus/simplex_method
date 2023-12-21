use std::collections::HashMap;
use crate::{Problem, Condition, Solution, var, matrix};


#[test] fn test_simplex_method() {
    let problem = Problem::maximize(4*var("x1") + 5*var("x2"),
        Condition::each_le(
            matrix! {
                2, 5
                6, 4
                3, 2
            },
            vec![
                var("x1"),
                var("x2"),
            ],

            // le

            vec![
                20,
                27,
                12,
            ]
        )
    );
    assert_eq!(problem.solve().unwrap(), Solution {
        variables:     HashMap::from([
            (var("x1"), 2.5),
            (var("x2"), 3.),
        ]),
        optimal_value: 25.,
    });
}
