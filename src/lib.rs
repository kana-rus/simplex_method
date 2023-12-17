#[cfg(not(target_pointer_width = "64"))]
compile_error!{ "Not support targets where pointer width is not 64" }

mod expression;
mod problem;
mod table;

use expression::*;
use table::SimplexTable;

type Scalor = f64;
