use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Cli {
    pub color: String,

    #[arg(long)]
    pub from_srgb: bool,

    #[arg(long, default_value = "dark")]
    pub mode: Mode,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Mode {
    Dark,
    Light,
}
