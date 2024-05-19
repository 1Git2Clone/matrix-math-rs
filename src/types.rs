// TODO: Newtype pattern with the Iter, len and indexing implementation.
// pub struct MatrixRef<'m, T>(&'m [Vec<T>]);
// pub struct Matrix<T>(Vec<Vec<T>>);
pub type MatrixRef<'m, T> = &'m [Vec<T>];
pub type Matrix<T> = Vec<Vec<T>>;
