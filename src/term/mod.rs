use color_eyre::eyre::Result;

use crate::colors::unified::AnsiPaletteHex;

/// unix/unix-like feature: print all sequences to all tty
/// Supported: Linux
#[cfg(target_family = "unix")]
mod unix;

/// windows feature: add palette for windows terminal
#[cfg(feature = "seq_win")]
#[cfg(target_family = "windows")]
mod win;

#[allow(unused_variables)] // annoying
pub fn apply(ansi: &AnsiPaletteHex) -> Result<()> {
    #[cfg(target_family = "unix")]
    unix::unix_term(ansi)?;

    #[cfg(feature = "seq_win")]
    #[cfg(target_family = "windows")]
    win::win_term(ansi)?;

    Ok(())
}

