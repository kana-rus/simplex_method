use std::ops::Mul;

use super::{Scalor, Variable, Polynomial};


pub struct Matrix<T> {
    pub rows:        Vec<Vec<T>>,
    pub row_size:    usize,
    pub column_size: usize,
}

impl<T> Matrix<T> {
    pub fn with_capacity(row_size: usize, column_size: usize) -> Self {
        let mut rows = Vec::with_capacity(column_size);
        for _ in 0..column_size {
            rows.push(Vec::with_capacity(row_size))
        }
        Self { rows, row_size, column_size }
    }

    pub fn try_from<Row: IntoIterator<Item = T>>(rows: impl IntoIterator<Item = Row>) -> Result<Self, String> {
        let mut rows = rows.into_iter();

        let first_row: Vec<T> = match rows.next() {
            Some(r) => r.into_iter().collect(),
            None    => return Err((|| format!("Got empty rows in input"))()),
        };

        let row_size = first_row.len();
        let mut column_size = 1;

        let mut collected_rows = Vec::new();
        collected_rows.push(first_row);
        for row in rows {
            let row = row.into_iter().collect::<Vec<_>>();
            if row.len() != row_size {
                return Err((|| format!("Rows have different lengths"))())
            }
            collected_rows.push(row);
            column_size += 1;
        }

        Ok(Self {
            rows: collected_rows,
            row_size,
            column_size,
        })
    }
}

impl<T: Clone> Matrix<T> {
    pub fn try_concat(self, another: Self) -> Result<Self, String> {
        if self.column_size != another.column_size {
            return Err((|| format!("Column sizes not match"))())
        }

        let row_size    = self.row_size + another.row_size;
        let column_size = self.column_size /* == another.column_size */;

        Ok(Self {
            rows: self.rows.into_iter().zip(another.rows).map(|(left, right)| [left, right].concat()).collect(),
            row_size,
            column_size,
        })
    }
}

impl Matrix<Scalor> {
    pub fn zeroed(row_size: usize, column_size: usize) -> Self {
        let mut rows = Vec::with_capacity(column_size);
        for _ in 0..column_size {
            rows.push(vec![0.; row_size])
        }
        Self { rows, row_size, column_size }
    }
    pub fn identity(size: usize) -> Self {
        let mut m = Matrix::zeroed(size, size);
        for i in 0..size {
            m[i][i] = 1.
        }
        m
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.rows[i]
    }
}
impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.rows[i]
    }
}

impl Mul<Vec<Variable>> for Matrix<Scalor> {
    type Output = Vec<Polynomial>;
    fn mul(self, variables: Vec<Variable>) -> Self::Output {
        assert_eq!(self.row_size, variables.len());

        let mut polynomials = Vec::with_capacity(variables.len());
        for row in self.rows {
            let variables = variables.clone();
            polynomials.push(
                row.into_iter().zip(variables)
                    .fold(Polynomial::new(), |p, (scalor, var)| p + scalor*var)
            );
        }
        polynomials
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
        Matrix::try_from([
            $( [
                $($item),+
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
        1., 2., 3., 4.
        5., 6., 7., 8.
        9.,10.,11.,12.
    };
    assert_eq!(m.row_size,    4);
    assert_eq!(m.column_size, 3);
}
