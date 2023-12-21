use std::ops::{Mul};
use super::{scalor::Scalor, variable::Variable};


#[derive(Debug, PartialEq)]
pub struct Term {
    pub coefficient: Scalor,
    pub variable:    Variable,
}

const _: () = {
    macro_rules! mul_variable {
        ($( $s:ty )*) => {$(
            impl Mul<Variable> for $s {
                type Output = Term;
                fn mul(self, variable: Variable) -> Self::Output {
                    Term { coefficient:self.into(), variable }
                }
            }
            impl Mul<&Variable> for $s {
                type Output = Term;
                fn mul(self, variable: &Variable) -> Self::Output {
                    Term { coefficient:self.into(), variable:variable.clone() }
                }
            }
        )*};
    } mul_variable! {
        i8 i16 i32 /*i64 i128 */
        u8 u16 u32 /*u64 u128 */
        f32 f64
    }
};
