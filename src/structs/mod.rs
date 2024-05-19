use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Whether you want to use User Input or have predefined values.
    #[arg(short, long, default_value_t = true)]
    pub interactive: bool,
}
