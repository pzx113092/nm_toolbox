mod calculator;
mod converter;
mod enums;
mod info;
mod keyboard;

use enums::Isotope;

use crate::app::{
    calculator::Calculator,
    converter::Converter,
    enums::{TimeID, WidgetSelection},
    info::Info,
};

#[derive(Default)]
pub struct WidgetOpen {
    calculator: bool,
    converter: bool,
    information: bool,

    calc_w: Calculator,
    conv_w: Converter,
    info_w: Info,
}

impl WidgetOpen {
    fn swap(&mut self, selection: &WidgetSelection) {
        let (calculator, converter, information) = match selection {
            WidgetSelection::Calculator => (true, false, false),
            WidgetSelection::Converter => (false, true, false),
            WidgetSelection::Info => (false, false, true),
            WidgetSelection::None => (false, false, false),
        };

        self.calculator = calculator;
        self.converter = converter;
        self.information = information;
    }

    fn show_all(&mut self, ui: &egui::Ui) {
        self.calc_w.show(ui, &mut self.calculator);
        self.conv_w.show(ui, &mut self.converter);
        self.info_w.show(ui, &mut self.information);
    }
}

pub struct App {
    // Converter
    style: bool,
    settings: bool,
    zoom_factor: f32,

    widget_open: WidgetOpen,
    widget_selection: WidgetSelection,

    pub on_screen_keyboard: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            style: false,
            settings: false,
            zoom_factor: 1.0,
            widget_open: WidgetOpen::default(),
            widget_selection: WidgetSelection::None,
            on_screen_keyboard: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        replace_fonts(&cc.egui_ctx);
        set_text_sizes(&cc.egui_ctx);
        cc.egui_ctx.set_zoom_factor(1.3);
        Default::default()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //let visuals = ui.ctx().global_style().visuals.clone();
        let s = ui.ctx().clone();
        ui.set_zoom_factor(self.zoom_factor);

        if !self.style {
            set_text_sizes(ui);
            self.style = true;
        }

        egui::Window::new("🔧 Settings")
            .open(&mut self.settings)
            .vscroll(true)
            .show(&s, |ui| {
                s.settings_ui(ui);
            });

        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.collapsing("Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("zoom: ");
                        if ui.button("➖").clicked() {
                            self.zoom_factor -= 0.1;
                        }
                        if ui.button("➕").clicked() {
                            self.zoom_factor += 0.1;
                        }
                    });
                    egui::widgets::global_theme_preference_buttons(ui);
                    ui.checkbox(&mut self.on_screen_keyboard, "touch screen");

                    ui.add(egui::Separator::default().vertical());
                    if ui.button("⟲ reset").clicked() {
                        *self = Self::default();
                    }
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("kb_state"), self.on_screen_keyboard)
                    });
                    if ui.button("🔧 other").clicked() {
                        self.settings = true;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.radio_value(
                &mut self.widget_selection,
                WidgetSelection::Calculator,
                "Calculator",
            );
            ui.radio_value(
                &mut self.widget_selection,
                WidgetSelection::Converter,
                "Converter",
            );
            ui.radio_value(&mut self.widget_selection, WidgetSelection::Info, "Info");

            self.widget_open.swap(&self.widget_selection);
            self.widget_open.show_all(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                use egui::special_emojis::GITHUB;

                ui.hyperlink_to(
                    format!("{GITHUB} Jakub Olejnik"),
                    "https://github.com/pzx113092/nm_toolbox",
                )
            });
        });
    }
}

fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "DejavuSansMono".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/DejavuSansMono.ttf"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "DejavuSansMono".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("DejavuSansMono".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub fn set_text_sizes(ctx: &egui::Context) {
    // Pick the sizes you want (in points)
    let heading_font = egui::FontId::new(26.0, egui::FontFamily::Proportional);
    let body_font = egui::FontId::new(20.0, egui::FontFamily::Proportional); // <- increase from default
    let button_font = egui::FontId::new(20.0, egui::FontFamily::Proportional); // <- increase from default
    ctx.all_styles_mut(|style| {
        style
            .text_styles
            .insert(egui::TextStyle::Heading, heading_font.clone());
        style
            .text_styles
            .insert(egui::TextStyle::Body, body_font.clone());
        style
            .text_styles
            .insert(egui::TextStyle::Button, button_font.clone());
    });
}

fn isotope_combo(isotope: &mut Isotope, ui: &mut egui::Ui, name: &'static str) {
    egui::ComboBox::from_id_salt(name)
        .selected_text(isotope.display())
        .width(20.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(isotope, Isotope::Tc99m, Isotope::Tc99m.display());
            ui.selectable_value(isotope, Isotope::I131, Isotope::I131.display());
            ui.selectable_value(isotope, Isotope::I123, Isotope::I123.display());
            ui.selectable_value(isotope, Isotope::Lu177, Isotope::Lu177.display());
            ui.selectable_value(isotope, Isotope::Ra223, Isotope::Ra223.display());
            ui.selectable_value(isotope, Isotope::F18, Isotope::F18.display());
            ui.selectable_value(isotope, Isotope::Ga68, Isotope::Ga68.display());
            ui.selectable_value(isotope, Isotope::Cs137, Isotope::Cs137.display());
        });
}
// duration in seconds

fn activity_left(n0: f32, hl: f32, t: f32) -> f32 {
    if hl != 0.0 {
        n0 * f32::powf(0.5, t / hl)
    } else {
        0.0
    }
}

pub fn is_kb_active(ui: &egui::Ui) -> bool {
    ui.data(|data| {
        data.get_temp::<bool>(egui::Id::new("kb_state"))
            .unwrap_or(false)
    })
}
