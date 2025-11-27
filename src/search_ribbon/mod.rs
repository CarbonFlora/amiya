use egui::ThemePreference;

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(default)]
#[derive(Default)]
pub struct SearchRibbon {
    #[serde(skip)] // This how you opt-out of serialization of a field
    search_query: String,
}

impl eframe::App for SearchRibbon {
    fn update(&mut self, ctx: &egui::Context, _frasme: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.as_bar(ctx, ui);
        });
    }
}

impl SearchRibbon {
    fn as_bar(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.settings(ctx, ui);
            });

            egui::widgets::global_theme_preference_buttons(ui);
        });
    }

    fn settings(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let mut theme = ui.ctx().options(|opt| opt.theme_preference);

        ui.menu_button(" ⛭ ", |ui| {
            ui.heading("Settings");
            ui.button(theme_title(theme)).clicked().then(|| {
                change_theme(&mut theme);
                ui.ctx().set_theme(theme);
            })
        });

        ui.add(egui::TextEdit::singleline(&mut self.search_query).hint_text("Search Bar"));
    }
}

fn theme_title(theme: ThemePreference) -> String {
    match theme {
        ThemePreference::Light => String::from("Change to 🌙 Mode"),
        ThemePreference::Dark | ThemePreference::System => String::from("Change to ☀ Mode"),
    }
}

fn change_theme(theme: &mut ThemePreference) {
    *theme = match theme {
        ThemePreference::Light => ThemePreference::Dark,
        ThemePreference::Dark | ThemePreference::System => ThemePreference::Light,
    }
}
