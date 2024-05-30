# Matrix-math

[![GH_Build Icon]][GH_Build Status]&emsp;[![Build Icon]][Build Status]&emsp;[![Docs Icon]][Docs]&emsp;[![Version Icon]][Crate]&emsp;[![License Icon]][LICENSE]

[GH_Build Icon]: https://img.shields.io/github/actions/workflow/status/1git2clone/matrix-math-rs/rust.yml?branch=main
[GH_Build Status]: https://github.com/1git2clone/matrix-math-rs/actions?query=branch%3Amaster
[Build Icon]: https://gitlab.com/1k2s/matrix-math/badges/main/pipeline.svg
[Build Status]: https://gitlab.com/1k2s/matrix-math/-/pipelines
[Docs Icon]: https://docs.rs/matrix-math/badge.svg
[Docs]: https://docs.rs/matrix-math/latest/leetcode_trees_rs/
[Version Icon]: https://img.shields.io/crates/v/matrix-math.svg
[Crate]: https://crates.io/crates/matrix-math
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

A simple Rust library/CLI app that allows for basic work with mathematical Matrices.

## Getting started

```rs
use matrix::prelude::*;

fn interactive_matrices_example() -> Result<(), Error> {
    let matrix1 = matrix::<f32>(1, None, None);

    // By calling matrix2 with the length parameters of matrix1 like so. Doing the operations
    // unchecked is **SAFE**.
    let matrix2 = matrix::<f32>(2, Some(matrix1.len()), Some(matrix1[0].len()));

    let sum = matrix_operation_unchecked(MatrixOperation::Addition, &matrix1, &matrix2);
    println!("Sum:\n{:#?}", sum);

    let diff = matrix_operation_unchecked(MatrixOperation::Subtraction, &matrix1, &matrix2);
    println!("Difference:\n{:#?}", diff);

    let product = matrix_operation_unchecked(MatrixOperation::Multiplication, &matrix1, &matrix2);
    println!("Product:\n{:#?}", product);

    let division = matrix_operation_unchecked(MatrixOperation::Division, &matrix1, &matrix2);
    println!("Results from division:\n{:#?}", division);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if args.interactive {
        interactive_matrices_example()?;
    }

    Ok(())
}
```
