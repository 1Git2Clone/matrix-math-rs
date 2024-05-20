use crate::enums::MatrixOperation;
use crate::error::MatrixError;
use crate::structs::Matrix;

pub fn nonzero_positive_input<T>(msg: &str, desired_count: Option<T>) -> T
where
    T: std::str::FromStr + std::cmp::PartialOrd + std::fmt::Debug,
    T: num_traits::Unsigned + num_traits::NumCast,
{
    use std::io::Write;

    if let Some(count) = desired_count {
        println!("Count inferred as: {count:?}");
        return count;
    }

    let mut input = String::new();

    loop {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        println!();
        match input.trim().parse::<T>() {
            Ok(result) => {
                if result < num_traits::cast(1).expect("Infalliable num trait cast.") {
                    input = String::new();
                    eprintln!("Error: The Input needs to be greater than 0.");
                    continue;
                }
                return result;
            }
            _ => {
                input = String::new();
                eprintln!("Error: invalid input");
                continue;
            }
        };
    }
}

pub fn numeric_input<T>(msg: &str, predefined: Option<T>) -> T
where
    T: std::str::FromStr + std::fmt::Debug + num_traits::Num,
{
    use std::io::Write;

    if let Some(input) = predefined {
        println!("Input was inferred as: {input:?}");
        return input;
    }

    let mut input = String::new();

    loop {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        println!();
        match input.trim().parse::<T>() {
            Ok(result) => {
                return result;
            }
            _ => {
                input = String::new();
                eprintln!("Error: invalid input");
                continue;
            }
        };
    }
}

pub fn matrix<T>(n: u32, cols: Option<usize>, rows: Option<usize>) -> Matrix<T>
where
    T: num_traits::Num,
    T: std::str::FromStr + std::fmt::Debug + std::default::Default,
{
    let col_count =
        nonzero_positive_input::<usize>(&format!("Enter column count for matrix {}: ", n), cols);
    let row_count =
        nonzero_positive_input::<usize>(&format!("Enter row count for matrix {}: ", n), rows);

    let mut res: Matrix<T> = Matrix::default();

    for col in 1..=col_count {
        let mut row_content = Vec::with_capacity(col_count);
        for row in 1..=row_count {
            row_content.push(numeric_input::<T>(
                &format!(
                    "Enter item at row {} and column {} for matrix {n}: ",
                    row, col
                ),
                None,
            ));
        }
        res.content.push(row_content);
    }

    res
}

/// SAFETY:
///
/// This function assumes `m1.len()` == `m2.len()` as well as `m1[0].len()` == `m2[0].len()`.
/// If you wish to instead handle the errors in the case you're unsure if the lengths are the same
/// then please use `fn matrix_operation()` and handle the `Result<T, E>` appropriately.
pub fn matrix_operation_unchecked<'m, T>(
    op: MatrixOperation,
    m1: &'m Matrix<T>,
    m2: &'m Matrix<T>,
) -> Matrix<T>
where
    T: std::str::FromStr + std::fmt::Debug + num_traits::Num + num_traits::RefNum<T>,
    &'m T: std::ops::Add<&'m T, Output = T>,
    &'m T: std::ops::Sub<&'m T, Output = T>,
    &'m T: std::ops::Mul<&'m T, Output = T>,
    &'m T: std::ops::Div<&'m T, Output = T>,
{
    use MatrixOperation as MO;

    let operation: Box<dyn Fn(&'m T, &'m T) -> T> = match op {
        MO::Addition => Box::new(|a, b| a + b),
        MO::Subtraction => Box::new(|a, b| a - b),
        MO::Multiplication => Box::new(|a, b| a * b),
        MO::Division => Box::new(|a, b| a / b),
    };

    Matrix {
        content: m1
            .content
            .iter()
            .enumerate()
            .map(|(i, col)| {
                col.iter()
                    .enumerate()
                    .map(|(j, _)| operation(&m1.content[i][j], &m2.content[i][j]))
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>(),
    }
}

#[allow(unused)]
pub fn matrix_operation<'m, T>(
    op: MatrixOperation,
    m1: &'m Matrix<T>,
    m2: &'m Matrix<T>,
) -> Result<Matrix<T>, MatrixError<'m, T>>
where
    T: std::str::FromStr + std::fmt::Debug + Clone + num_traits::Num + num_traits::RefNum<T>,
    &'m T: std::ops::Add<&'m T, Output = T>,
    &'m T: std::ops::Sub<&'m T, Output = T>,
    &'m T: std::ops::Mul<&'m T, Output = T>,
    &'m T: std::ops::Div<&'m T, Output = T>,
{
    use MatrixError as ME;

    if m1.content.is_empty() {
        return Err(ME::Empty(m1));
    }
    if m2.content.is_empty() {
        return Err(ME::Empty(m2));
    }

    if m1.content.len() != m2.content.len() || m1.content[0].len() != m2.content[0].len() {
        return Err(ME::NotEqual(m1, m2));
    }

    Ok(matrix_operation_unchecked(op, m1, m2))
}
