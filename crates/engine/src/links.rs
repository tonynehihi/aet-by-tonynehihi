//! Sibling-app link helpers for the engine's `help.sibling` command (a protocol
//! surface kept from upstream; the UI of this personal build shows no upstream
//! links anywhere — see the 1.1.0 rebrand notes).

pub const APP_NAME: &str = "After Effects by Tonynehihi";
pub const APP_SLUG: &str = "effectcraft";

/// Sibling apps of the ArtCraft family: (name, what it is, GitHub repo, app page).
pub const SIBLINGS: &[(&str, &str)] = &[
    ("PhotoCraft", "photocraft"),
    ("VectorCraft", "vectorcraft"),
    ("FilmCraft", "filmcraft"),
    ("LightCraft", "lightcraft"),
    ("PdfCraft", "pdfcraft"),
    ("DesignCraft", "designcraft"),
];

pub fn github_of(slug: &str) -> String {
    format!("https://github.com/storytold/{slug}")
}
pub fn page_of(slug: &str) -> String {
    format!("https://getartcraft.com/apps/{slug}")
}
