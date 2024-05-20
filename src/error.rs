//! Not all errors are used that's why.
#![allow(dead_code)]

use crate::structs::Matrix;

#[derive(Debug, thiserror::Error)]
pub enum MatrixError<'m, T>
where
    T: num_traits::Num,
    T: std::fmt::Debug,
{
    #[error("The matrix {0} is empty")]
    Empty(&'m Matrix<T>),
    #[error("Matrices aren't equal! {0} != {1}")]
    NotEqual(&'m Matrix<T>, &'m Matrix<T>),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),
}
