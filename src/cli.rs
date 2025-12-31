use clap::{Parser, ValueEnum};

use crate::backends::{BackendEnum, BalanceContrast};

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Cli {
    pub color: Option<String>,

    #[arg(long)]
    pub from_srgb: bool,

    /// Dump json simplified
    #[arg(long, conflicts_with = "dump")]
    pub json_dump: bool,

    /// Dump
    #[arg(long, default_value = "human-readable")]
    pub dump: DumpMode,

    #[arg(short = 'v')]
    pub verbose: bool,

    #[arg(short, long)]
    pub apply: bool,

    #[arg(short, long)]
    pub quiet: bool,

    #[arg(short = 'B', long, default_value = "dms")]
    pub backend: BackendEnum,

    #[arg(short, long)]
    pub balance: Option<BalanceContrast>,

    #[arg(long, default_value = "dark")]
    pub mode: Mode,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum DumpMode {
    #[value(name = "HumanReadable", alias = "human-readable", alias = "human")]
    HumanReadable,

    #[value(
        name = "Block",
        alias = "HumanReadableBlock",
        alias = "human-readable-block",
        alias = "block"
    )]
    Block,

    #[value(name = "JsonSimplifed", alias = "json-simplified", alias = "json")]
    JsonSimplified,

    #[value(name = "JsonPretty", alias = "json-pretty", alias = "matugen")]
    JsonPretty,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Mode {
    Dark,
    Light,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Dark => write!(f, "dark"),
            Mode::Light => write!(f, "light"),
        }
    }
}
