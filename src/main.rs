mod enums;
mod error;
mod prelude;
mod structs;
mod utils;

use crate::prelude::*;

fn interactive_matrices_example() -> Result<(), Error> {
    let matrix1 = matrix::<f32>(1, None, None);

    // By calling matrix2 with the length parameters of matrix1 like so. Doing the operations
    // unchecked is **SAFE**.
    let matrix2 = matrix::<f32>(
        2,
        Some(matrix1.content.len()),
        Some(matrix1.content[0].len()),
    );

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
