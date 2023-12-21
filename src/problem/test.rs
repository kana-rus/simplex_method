use crate::{*, components::variable::slack};


#[test] fn test_into_standard_form() {
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

            /* less than or equals to */

            vec![
                20,
                27,
                12,
            ]
        )
    );
    assert_eq!(problem.into_standard_form(), Problem {
        objective_function: 4*var("x1") + 5*var("x2"),
        condition: Condition::each_le(
            matrix! {
                2, 5, 1, 0, 0
                6, 4, 0, 1, 0
                3, 2, 0, 0, 1
            },
            vec![
                var("x1"),
                var("x2"),
                slack(1),
                slack(2),
                slack(3),
            ],

            /* less than or equals to */

            vec![
                20,
                27,
                12,
            ]
        )
    });
}
