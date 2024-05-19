//! ## Description
//!
//! A simple library/CLI app for matrix math in Rust using Clap and thiserror.
//!
//! ### Getting started
//!
//! ```rs
//! use matrix::prelude::*;
//!
//! fn interactive_matrices_example() -> Result<(), Error> {
//!     let matrix1 = matrix::<f32>(1, None, None);
//!
//!     // By calling matrix2 with the length parameters of matrix1 like so. Doing the operations
//!     // unchecked is **SAFE**.
//!     let matrix2 = matrix::<f32>(2, Some(matrix1.len()), Some(matrix1[0].len()));
//!
//!     let sum = matrix_operation_unchecked(MatrixOperation::Addition, &matrix1, &matrix2);
//!     println!("Sum:\n{:#?}", sum);
//!
//!     let diff = matrix_operation_unchecked(MatrixOperation::Subtraction, &matrix1, &matrix2);
//!     println!("Difference:\n{:#?}", diff);
//!
//!     let product = matrix_operation_unchecked(MatrixOperation::Multiplication, &matrix1, &matrix2);
//!     println!("Product:\n{:#?}", product);
//!
//!     let division = matrix_operation_unchecked(MatrixOperation::Division, &matrix1, &matrix2);
//!     println!("Results from division:\n{:#?}", division);
//!
//!     Ok(())
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let args = Args::parse();
//!
//!     if args.interactive {
//!         interactive_matrices_example()?;
//!     }
//!
//!     Ok(())
//! }
//! ```

// #![warn(missing_docs)]

pub mod enums;
pub mod error;
pub mod prelude;
pub mod structs;
pub mod types;
pub mod utils;
