# Matrix-math

[![Build Icon]][Build Status]&emsp;[![License Icon]][LICENSE]

[Build Icon]: https://img.shields.io/github/actions/workflow/status/1kill2steal/matrix-rs/ci.yml?branch=main
[Build Status]: https://github.com/1kill2steal/matrix-rs/actions?query=branch%3Amaster
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

A simple Rust library/CLI app that allows for basic work with mathematical Matrices.

## Getting started

```rs
use matrix::prelude::*;

fn interactive_matrices_example() -> Result<(), Error> {
    let matrix1 = matrix(1, None, None);

    // By calling matrix2 with the length parameters of matrix1 like so. Doing the operations
    // unchecked is **SAFE**.
    let matrix2 = matrix(2, Some(matrix1.len()), Some(matrix1[0].len()));

    let sum = matrix_operation_unchecked(MatrixOperation::Addition, &matrix1, &matrix2);
    println!("Sum:\n{:#?}", sum);

    let diff = matrix_operation_unchecked(MatrixOperation::Subtraction, &matrix1, &matrix2);
    println!("Difference:\n{:#?}", diff);

    let product = matrix_operation_unchecked(MatrixOperation::Multiplication, &matrix1, &matrix2);
    println!("Product:\n{:#?}", product);

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
