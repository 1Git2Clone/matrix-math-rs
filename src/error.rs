//! Not all errors are used that's why.
#![allow(dead_code)]

use crate::types::MatrixRef;

#[derive(Debug, thiserror::Error)]
pub enum MatrixError<'m, T> {
    #[error("The matrix {0} is empty")]
    Empty(MatrixRef<'m, T>),
    #[error("Matrices aren't equal! {0} != {1}")]
    NotEqual(MatrixRef<'m, T>, MatrixRef<'m, T>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),
}
