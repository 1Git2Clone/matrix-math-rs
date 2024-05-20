#![allow(unused_imports)]

#[doc(hidden)]
pub use clap::Parser;

pub use crate::enums::MatrixOperation;
pub use crate::error::*;
pub use crate::structs::Args;
pub use crate::structs::Matrix;
// pub use crate::types::{Matrix, MatrixRef};
pub use crate::utils::{matrix, matrix_operation, matrix_operation_unchecked};
