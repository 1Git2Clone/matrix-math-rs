mod enums;
use enums::MatrixOperation;

mod error;
use error::Error;

mod structs;
use structs::Args;

mod types;

mod utils;
use utils::{matrix, matrix_operation_unchecked};

fn main() -> Result<(), Error> {
    use clap::Parser;
    let args = Args::parse();

    if args.interactive {
        let matrix1 = matrix(1, None, None);
        // By calling matrix2 with the length parameters of matrix1 like so. Doing the operations
        // unchecked is **SAFE**.
        let matrix2 = matrix(2, Some(matrix1.len()), Some(matrix1[0].len()));
        let sum = matrix_operation_unchecked(MatrixOperation::Addition, &matrix1, &matrix2);
        println!("Sum:\n{:#?}", sum);
        let diff = matrix_operation_unchecked(MatrixOperation::Subtraction, &matrix1, &matrix2);
        println!("Difference:\n{:#?}", diff);
        let product =
            matrix_operation_unchecked(MatrixOperation::Multiplication, &matrix1, &matrix2);
        println!("Product:\n{:#?}", product);
    }

    Ok(())
}
