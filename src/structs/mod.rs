use clap::Parser;

#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    T: num_traits::Num,
    T: std::fmt::Debug,
{
    pub content: Vec<Vec<T>>,
}

impl<T> std::default::Default for Matrix<T>
where
    T: num_traits::Num,
    T: std::fmt::Debug,
{
    fn default() -> Self {
        Self {
            content: Vec::default(),
        }
    }
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: num_traits::Num,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("content", &self.content)
            .finish()
    }
}

impl<T> Matrix<T>
where
    T: num_traits::Num,
    T: std::fmt::Debug,
{
    #[allow(dead_code)]
    pub fn from(content: Vec<Vec<T>>) -> Self {
        Self { content }
    }
}

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Whether you want to use User Input or have predefined values.
    #[arg(short, long, default_value_t = true)]
    pub interactive: bool,
}
