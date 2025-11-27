use crate::{landing::LandingPage, search_ribbon::SearchRibbon};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    open_page: PageType,
    search_ribbon: SearchRibbon,
    landing_page: LandingPage,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum PageType {
    LandingPage = 0,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let ribbon = SearchRibbon::default();
        let landing_page = LandingPage::default();
        Self {
            search_ribbon: ribbon,
            open_page: PageType::LandingPage,
            landing_page,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.search_ribbon.update(ctx, frame);
        match self.open_page {
            PageType::LandingPage => self.landing_page.update(ctx, frame),
        }
    }
}
