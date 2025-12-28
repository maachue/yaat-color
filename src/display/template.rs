use serde::Serialize;

use crate::colors::unified::ViewerAsIndexMapAnsiPalette;

#[derive(Serialize)]
pub struct FreshJson<'a> {
    pub yaat: ViewerAsIndexMapAnsiPalette<'a, String>,
}
