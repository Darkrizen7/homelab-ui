//! Shared look of the homelab panels (vps-edge dashboard, game-manager): the stylesheet, the
//! small DOM/format helpers module, the fonts and the icon, compiled into the binary.
//!
//! Each app serves [`FILES`] under a prefix of its choice (`/ui/`, `/games/ui/`...): every URL
//! inside the files is relative, so nothing points to another app.
//!
//! ```
//! let css = homelab_ui::get("style.css").unwrap();
//! assert_eq!(css.mime, "text/css; charset=utf-8");
//! ```

pub struct Asset {
    /// Path under the prefix, e.g. `fonts/inter.woff2`.
    pub path: &'static str,
    pub bytes: &'static [u8],
    pub mime: &'static str,
    /// Never changes for a given path (fonts): safe to cache for a year.
    pub immutable: bool,
}

macro_rules! assets {
    ($($path:literal => $mime:literal, $immutable:literal),* $(,)?) => {
        &[$(Asset {
            path: $path,
            bytes: include_bytes!(concat!("../assets/", $path)),
            mime: $mime,
            immutable: $immutable,
        }),*]
    };
}

pub const FILES: &[Asset] = assets! {
    "style.css" => "text/css; charset=utf-8", false,
    "format.js" => "text/javascript; charset=utf-8", false,
    "favicon.svg" => "image/svg+xml", false,
    "fonts/inter.woff2" => "font/woff2", true,
    "fonts/jetbrains-mono.woff2" => "font/woff2", true,
};

pub fn get(path: &str) -> Option<&'static Asset> {
    FILES.iter().find(|asset| asset.path == path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_file_is_embedded() {
        for asset in FILES {
            assert!(!asset.bytes.is_empty(), "{}", asset.path);
        }
    }

    /// Absolute URLs would tie the library to one app's routes.
    #[test]
    fn urls_are_relative() {
        let css = std::str::from_utf8(get("style.css").unwrap().bytes).unwrap();
        assert!(!css.contains("url(\"/") && !css.contains("url('/") && !css.contains("url(/"));
        for font in FILES.iter().filter(|a| a.path.starts_with("fonts/")) {
            assert!(css.contains(&format!("url(\"{}\")", font.path)), "{} not used", font.path);
        }
        let js = std::str::from_utf8(get("format.js").unwrap().bytes).unwrap();
        assert!(js.contains("export function el("));
        assert!(!js.contains("import "), "format.js must not depend on any app");
    }
}
