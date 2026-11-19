//! The Mirai fork's display name for the app.
//!
//! Upstream calls the app GridCraft, in its own strings and in every translation table
//! (`crates/ui-egui/src/i18n/`). The fork shows it as "Mirai Calc" without editing those tables:
//! [`rebrand`] swaps the name in every interface string that goes through
//! [`crate::i18n::Lang::tr`], and the places that draw the name directly (the window title, About,
//! Backstage, message titles) use [`APP_NAME`]. Identifiers stay upstream's: the binary, crate
//! names, the app id, the settings folder and the installer's registry keys, so settings and
//! installs carry over.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// The name the fork shows for the app.
pub const APP_NAME: &str = "Mirai Calc";

/// The upstream credit, shown in the About dialog and the README: the original project, by name.
/// Drawn as it stands, never through [`crate::i18n::Lang::tr`]: [`rebrand`] renames the fork's own
/// interface strings, and this must keep the original project's name (a test pins it).
pub const UPSTREAM_CREDIT: &str = "Based on GridCraft by Storytold, part of ArtCraft.";

/// What the credit must also say: the fork stands on its own. Shown beside
/// [`UPSTREAM_CREDIT`], also drawn as it stands.
pub const FORK_NOTICE: &str = "Mirai Calc is an independent fork, not officially affiliated with ArtCraft or Storytold.";

/// The Mirai fork's community and support: the owner's GitHub home.
pub const COMMUNITY: &str = "https://github.com/mirasins";
/// Where the fork's issues are filed.
pub const ISSUES: &str = "https://github.com/mirasins/mirai-calc-gridcraft-fork/issues";
/// The fork's source and releases.
pub const GITHUB: &str = "https://github.com/mirasins/mirai-calc-gridcraft-fork";

/// The original project this fork is based on. A credit, not the fork's community: the About
/// dialog links to it beside the fork notice, labeled as the original project's, and it never
/// takes the fork's place.
pub const UPSTREAM: &str = "https://github.com/storytold/gridcraft";
/// ArtCraft's own website, linked as the original project's — never as the fork's.
pub const UPSTREAM_PAGE: &str = "https://getartcraft.com";
/// ArtCraft's Discord, linked as the original project's; the fork has no Discord of its own.
pub const UPSTREAM_DISCORD: &str = "https://discord.gg/artcraft";

/// Upstream's name for the app, as its interface strings spell it.
const UPSTREAM_NAME: &str = "GridCraft";

/// Distinct rebranded strings kept. Interface strings are a fixed set (catalog rows and literals),
/// so this is never reached in practice; it guards against unbounded growth all the same: past it,
/// a string not seen before is shown with upstream's name rather than kept.
const CACHE_LIMIT: usize = 8192;

/// `s` with the app's upstream name replaced by [`APP_NAME`]; `s` itself when it doesn't name the
/// app. The rebranded copies are cached for the life of the process, so a string is rebuilt once.
pub fn rebrand(s: &str) -> &str {
    if !s.contains(UPSTREAM_NAME) {
        return s;
    }
    static CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    // A poisoned lock (a panic elsewhere while holding it) only costs the rename, never a crash.
    let Ok(mut cache) = CACHE.get_or_init(Default::default).lock() else { return s };
    if let Some(&branded) = cache.get(s) {
        return branded;
    }
    if cache.len() >= CACHE_LIMIT {
        return s;
    }
    let branded: &'static str = Box::leak(s.replace(UPSTREAM_NAME, APP_NAME).into_boxed_str());
    cache.insert(s.to_owned(), branded);
    branded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_upstream_name_becomes_the_fork_name() {
        assert_eq!(rebrand("About GridCraft"), "About Mirai Calc");
        assert_eq!(rebrand("GridCraft"), APP_NAME);
        assert_eq!(rebrand("Acerca de GridCraft"), "Acerca de Mirai Calc");
    }

    #[test]
    fn other_strings_are_left_alone() {
        let s = "Save As";
        assert!(std::ptr::eq(rebrand(s), s));
        // The environment variable keeps its name.
        assert_eq!(rebrand("GRIDCRAFT_CONTROL_KEY"), "GRIDCRAFT_CONTROL_KEY");
    }

    #[test]
    fn a_string_is_rebuilt_once() {
        assert!(std::ptr::eq(rebrand("Quit GridCraft"), rebrand("Quit GridCraft")));
    }

    #[test]
    fn the_upstream_credit_names_the_original_project() {
        // The credit is drawn as it stands, never through `i18n::tr`: `rebrand` would rename the
        // upstream project in it, as it does in every one of the fork's own interface strings. This
        // pins what the About dialog must show: the original names, never the fork's.
        assert!(UPSTREAM_CREDIT.contains(UPSTREAM_NAME));
        assert!(UPSTREAM_CREDIT.contains("Storytold"));
        assert!(UPSTREAM_CREDIT.contains("ArtCraft"));
        assert!(!UPSTREAM_CREDIT.contains(APP_NAME));
        // The fork notice does the naming instead: it says the fork stands on its own.
        assert!(FORK_NOTICE.contains(APP_NAME));
        assert!(FORK_NOTICE.contains("independent fork"));
        assert!(FORK_NOTICE.contains("not officially affiliated with ArtCraft or Storytold"));
    }

    #[test]
    fn the_fork_links_follow_the_owner() {
        // The fork's own surfaces: community, issues and source all live under the owner's GitHub
        // (docs/release-channel.md).
        assert_eq!(COMMUNITY, "https://github.com/mirasins");
        assert_eq!(GITHUB, "https://github.com/mirasins/mirai-calc-gridcraft-fork");
        assert_eq!(ISSUES, format!("{GITHUB}/issues"));
    }

    #[test]
    fn the_upstream_links_stay_upstreams() {
        // The credit's links keep the original project's addresses, clearly named as ArtCraft's:
        // nothing of ArtCraft is presented as a Mirai channel.
        assert_eq!(UPSTREAM, "https://github.com/storytold/gridcraft");
        assert_eq!(UPSTREAM_PAGE, "https://getartcraft.com");
        assert_eq!(UPSTREAM_DISCORD, "https://discord.gg/artcraft");
        for u in [COMMUNITY, ISSUES, GITHUB, UPSTREAM, UPSTREAM_PAGE, UPSTREAM_DISCORD] {
            assert!(u.starts_with("https://"), "{u}");
        }
    }
}
