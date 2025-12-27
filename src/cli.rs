use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Cli {
    pub color: String,

    #[arg(long)]
    pub from_srgb: bool,

    #[arg(long)]
    pub json_dump: bool,

    #[arg(long, default_value = "dms")]
    pub backend: BackEnd,

    #[arg(long, default_value = "dark")]
    pub mode: Mode,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum BackEnd {
    Dms,
    DmsWcag,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Mode {
    Dark,
    Light,
}
