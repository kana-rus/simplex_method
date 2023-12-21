//! Simplex method is a widely used algorithm for solving linear programming problems.
//! Here, "linear programming problem" is generally represented by:
//! ```ignore
//!     maximize:  c x
//!     s.t.  A x <= b, \forall{i}, x_i >= 0
//!     ( x, b, c \in R^n,  A \in R^m × R^n ) 
//! ```
//! 
//! This can be transformed to "standarnd form":
//! ```ignore
//!     maximize:  c x
//!     s.t.  A' x' = b, \forall{i}, x'_i >= 0
//!     ( A' = [A | E] )
//!     ( x' = (x_1, ..., x_n, s_1, ..., s_m)^t )
//! ```
//! 
//! Here `s_1, ..., s_m` are called "slack variable"s, introduced for following transformation:
//! 
//! ```ignore
//!     a_{i1} x_1 + a_{i2} x_2 + ... + a_{in} x_n  <=  b_i
//! --> a_{i1} x_1 + a_{i2} x_2 + ... + a_{in} x_n  + s_i  = b_i,  s_i >= 0
//! ```
//! 

#![feature(slice_concat_trait)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!{ "Not support targets where pointer width is not 64" }

mod components;
mod problem;
mod table;


pub use components::{
    variable::var,
    matrix::Matrix,
};
pub use problem::{
    GeneralFormCondition as Condition,
    GeneralFormProblem as Problem,
};
pub use table::{
    Solution,
};

impl Problem {
    pub fn solve(self) -> Result<Solution, String> {
        table::Table::from_standard_form_problem(
            self.into_standard_form()
        ).solve()
    }
}

#[macro_export]
/// ```ignore
/// let m = matrix! {
///     1., 2., 3., 4.
///     5., 6., 4., 8.
///     9., 10.,11.,12.
/// };
/// ```
macro_rules! matrix {
    {$( $($item:literal),+ )+} => {
        $crate::Matrix::try_from([
            $( [
                $($item.into()),+
            ] ),+
        ]).unwrap()
    };
}

#[cfg(test)] #[test] fn test_matrix_macro() {
    let m = matrix! {
        1., 2.
    };
    assert_eq!(m.row_size,    2);
    assert_eq!(m.column_size, 1);

    let m = matrix! {
        42,  3.14, 1
        5.2, 777,  9.2
    };
    assert_eq!(m.row_size,    3);
    assert_eq!(m.column_size, 2);

    let m = matrix! {
        1., 2., 3., 4.
        5., 6., 7., 8.
        9.,10.,11.,12.
    };
    assert_eq!(m.row_size,    4);
    assert_eq!(m.column_size, 3);
}
