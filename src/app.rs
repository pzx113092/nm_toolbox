mod calculator;
mod converter;
mod enums;
mod info;

use enums::Isotope;

use crate::app::{
    calculator::Calculator,
    converter::Converter,
    enums::{TimeID, WidgetSelection},
    info::Info,
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct WidgetOpen {
    calculator: bool,
    converter: bool,
    information: bool,

    calc_w: Calculator,
    conv_w: Converter,
    info_w: Info,
}

impl Default for WidgetOpen {
    fn default() -> Self {
        Self {
            calculator: false,
            converter: false,
            information: false,
            calc_w: Calculator::default(),
            conv_w: Converter::default(),
            info_w: Info::default(),
        }
    }
}

impl WidgetOpen {
    fn swap(&mut self, selection: &WidgetSelection) {
        let (calculator, converter, information) = match selection {
            WidgetSelection::Calculator => (true, false, false),
            WidgetSelection::Converter => (false, true, false),
            WidgetSelection::Info => (false, false, true),
            _ => (false, false, false),
        };

        self.calculator = calculator;
        self.converter = converter;
        self.information = information;
    }

    fn show_all(&mut self, ui: &mut egui::Ui) {
        self.calc_w.show(ui, &mut self.calculator);
        self.conv_w.show(ui, &mut self.converter);
        self.info_w.show(ui, &mut self.information);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    // Converter
    style: bool,
    settings: bool,
    zoom_factor: f32,

    widget_open: WidgetOpen,
    widget_selection: WidgetSelection,
}

impl Default for App {
    fn default() -> Self {
        Self {
            style: false,
            settings: false,
            zoom_factor: 1.0,
            widget_open: WidgetOpen::default(),
            widget_selection: WidgetSelection::NONE,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        set_text_sizes(&cc.egui_ctx);
        cc.egui_ctx.set_zoom_factor(1.3);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

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
                egui::widgets::global_theme_preference_switch(ui);
                ui.add(egui::Separator::default().vertical());
                if ui.button("🔧").clicked() {
                    self.settings = true;
                }
                ui.add(egui::Separator::default().vertical());
                if ui.button("⟲ Reset").clicked() {
                    *self = Self::default();
                }
                ui.add(egui::Separator::default().vertical());
                if ui.button("➖").clicked() {
                    self.zoom_factor -= 0.1;
                }
                if ui.button("➕").clicked() {
                    self.zoom_factor += 0.1;
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
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
            });
            self.widget_open.swap(&self.widget_selection);
            self.widget_open.show_all(ui);
        });
    }
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
