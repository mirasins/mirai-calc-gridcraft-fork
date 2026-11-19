use super::Dialog;

/// Help ▸ About GridCraft: the app, its contributors and the AI models that helped. The credits
/// are compiled in (`crate::credits`, docs/contributors.md).
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            ui.heading(crate::brand::APP_NAME);
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.add_space(4.0);
            ui.label("A clean-room, open-source, Rust-native spreadsheet.");
            // The fork's community, issues and source, all on the owner's GitHub.
            ui.hyperlink_to("Join the Mirai community", crate::brand::COMMUNITY);
            ui.hyperlink_to("Report an issue", crate::brand::ISSUES);
            ui.hyperlink_to("Source on GitHub", crate::brand::GITHUB);
            ui.add_space(4.0);
            // The upstream credit, clearly apart from the fork's own links above. Drawn as it
            // stands, never through `i18n::tr`: `brand::rebrand` renames the fork's interface
            // strings, and the credit must keep the original project's names (a test pins it).
            // Each credit link is labeled as the original project's, never as a Mirai channel.
            ui.label(crate::brand::UPSTREAM_CREDIT);
            ui.label(crate::brand::FORK_NOTICE);
            ui.hyperlink_to("GridCraft on GitHub (original project)", crate::brand::UPSTREAM);
            ui.hyperlink_to("ArtCraft website — original project, not Mirai", crate::brand::UPSTREAM_PAGE);
            ui.hyperlink_to("ArtCraft Discord (original project)", crate::brand::UPSTREAM_DISCORD);
        }
    }
}
