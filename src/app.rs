mod enums;
use egui::{CornerRadius, Margin};
use egui_plot::{Line, Plot, PlotPoints};
use enums::{Isotope, Unit};

use crate::app::enums::TimeID;

pub struct App {
    // Converter
    style: bool,
    settings: bool,
    unit: Unit,
    conv_input: f32,
    // Isotope info
    isotope: Isotope,
    //Activity calculator
    cal_date: jiff::civil::Date,
    target_date: jiff::civil::Date,
    tooltip_text: Option<String>,
    tooltip_until: Option<f64>,
    cal_time: (u8, u8, u8),
    target_time: (u8, u8, u8),
    time_area_state: (bool, bool),
}

impl Default for App {
    fn default() -> Self {
        let unit = Unit::MegaBq;
        let conv_input = 0.0;
        let isotope = Isotope::Tc99m;
        let cal_date = d_now();
        let time = t_now();

        Self {
            style: false,
            settings: false,
            unit,
            conv_input,
            isotope,
            cal_date,
            target_date: cal_date,
            tooltip_text: None,
            tooltip_until: None,
            time_area_state: (false, false),
            cal_time: time,
            target_time: time,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let visuals = ui.ctx().global_style().visuals.clone();
        let s = ui.ctx().clone();

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
                if ui.button("🔧 Settings").clicked() {
                    self.settings = true;
                }
                ui.add(egui::Separator::default().vertical());
                if ui.button("⟲ Reset").clicked() {
                    *self = Self::default();
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center).with_cross_align(egui::Align::Center),
                |ui| {
                    egui::ScrollArea::both()
                        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                        .max_width(600.0)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                                app_frame(&visuals).show(ui, |ui| {
                                    calculator(self, ui);
                                });
                                app_frame(&visuals).show(ui, |ui| {
                                    converter(self, ui);
                                });
                                app_frame(&visuals).show(ui, |ui| {
                                    isotope_info(self, ui);
                                });
                            });
                        });
                },
            );
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

fn app_frame(visuals: &egui::Visuals) -> egui::Frame {
    egui::Frame::default()
        .stroke(visuals.window_stroke)
        .inner_margin(Margin::same(10))
        .shadow(visuals.popup_shadow)
        .fill(visuals.panel_fill)
        .corner_radius(CornerRadius::same(16))
}

fn converter(app: &mut App, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Unit conversion").heading());
    ui.vertical_centered(|ui| {
        egui::Grid::new("unit_grid")
            .num_columns(2)
            .spacing([8.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    egui::ComboBox::from_id_salt("unit_combo")
                        .selected_text(app.unit.display())
                        .width(20.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.unit, Unit::MegaBq, "MBq");
                            ui.selectable_value(&mut app.unit, Unit::GigaBq, "GBq");
                            ui.selectable_value(&mut app.unit, Unit::MicroCi, "µCi");
                            ui.selectable_value(&mut app.unit, Unit::MiliCi, "mCi");
                        });
                });

                ui.add(
                    egui::DragValue::new(&mut app.conv_input)
                        .range(0.0..=1000000.0)
                        .max_decimals(4),
                );

                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("MBq");
                ui.label(format!(
                    "{:.3}",
                    app.conv_input * app.unit.multi() / Unit::MegaBq.multi()
                ));
                ui.end_row();

                ui.label("GBq");
                ui.label(format!(
                    "{:.3}",
                    app.conv_input * app.unit.multi() / Unit::GigaBq.multi()
                ));
                ui.end_row();

                ui.label("µCi");
                ui.label(format!(
                    "{:.3}",
                    app.conv_input * app.unit.multi() / Unit::MicroCi.multi()
                ));
                ui.end_row();

                ui.label("mCi");
                ui.label(format!(
                    "{:.3}",
                    app.conv_input * app.unit.multi() / Unit::MiliCi.multi()
                ));
                ui.end_row();
            });
    });
}

fn isotope_info(app: &mut App, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Isotope info").heading());
    ui.vertical_centered(|ui| {
        egui::Grid::new("isotope_grid")
            .num_columns(2)
            .spacing([8.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    isotope_combo(app, ui, "first");
                });
                ui.end_row();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Half life:");
                ui.label(parse_hl(app.isotope.hl().as_secs_f32()));
                ui.end_row();

                ui.label("Full decay:");
                ui.label(parse_hl(app.isotope.hl().as_secs_f32() * 10.0));
                ui.end_row();
            });
        ui.separator();

        if app.conv_input > 0.0 {
            let d = app.isotope.hl().as_secs_f32() / 10.0;
            let pp: PlotPoints<'_> = (0..100)
                .map(|i| {
                    let x = i as f32 * d;
                    [
                        x as f64,
                        activity_left(app.conv_input, app.isotope.hl().as_secs_f32(), x) as f64,
                    ]
                })
                .collect();

            let line = Line::new("activity", pp);
            let now = ui.input(|i| i.time);
            use std::cell::RefCell;
            let latest: RefCell<Option<String>> = RefCell::new(None);

            egui_plot::Plot::new("Activity_plot")
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .allow_axis_zoom_drag(false)
                .width(ui.available_width())
                .view_aspect(2.0)
                .show_grid(false)
                .x_axis_label("[s]")
                .y_axis_label(format!("[{}]", app.unit.display()))
                .label_formatter(|pos| {
                    let text = match pos {
                        egui_plot::HoverPosition::NearDataPoint { position, .. } if true => {
                            Some(format!(
                                "Activity: {:.1} {} ({:.1}%)\nTime passed: {}",
                                position.y,
                                app.unit.display(),
                                position.y * 100.0 / app.conv_input as f64,
                                parse_hl(position.x as f32)
                            ))
                        }
                        _ => None,
                    };

                    *latest.borrow_mut() = text.clone();
                    text
                })
                .show(ui, |plot_ui| plot_ui.line(line));

            if let Some(s) = latest.into_inner() {
                app.tooltip_text = Some(s);
                app.tooltip_until = Some(ui.input(|i| i.time) + 1.5);
            }

            if let Some(until) = app.tooltip_until
                && now > until
            {
                app.tooltip_text = None;
                app.tooltip_until = None;
            }

            if let Some(text) = &app.tooltip_text {
                egui::Window::new("area")
                    .title_bar(false)
                    .movable(true)
                    .drag_area(egui::WindowDrag::Anywhere)
                    .resizable(false)
                    .show(ui, |ui| {
                        ui.add(egui::Label::new(text));
                    });
            }
        } else {
            let line = Line::new("activity", PlotPoints::default());
            Plot::new("Activity_plot")
                .allow_drag(false)
                .allow_scroll(false)
                .allow_zoom(false)
                .allow_axis_zoom_drag(false)
                .width(ui.available_width())
                .view_aspect(2.0)
                .show_grid(false)
                .x_axis_label("[s]")
                .y_axis_label(format!("[{}]", app.unit.display()))
                .show(ui, |plot_ui| plot_ui.line(line));
        }
    });
}

fn calculator(app: &mut App, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Activity calculator").heading());
    ui.vertical_centered(|ui| {
        egui::Grid::new("activity_calculator")
            .num_columns(2)
            .spacing([8.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    isotope_combo(app, ui, "second");
                });
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("Activity:");
                ui.add(
                    egui::DragValue::new(&mut app.conv_input)
                        .range(0.0..=1000000.0)
                        .max_decimals(4),
                );
                ui.end_row();

                ui.label("Initial: ");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.add(
                        egui_extras::DatePickerButton::new(&mut app.cal_date)
                            .id_salt("cal_datepicker")
                            .format("%d-%m-%y")
                            .show_icon(false),
                    );

                    time_picker(ui, app, TimeID::Calibration);
                });

                ui.end_row();

                ui.label("Target:");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.add(
                        egui_extras::DatePickerButton::new(&mut app.target_date)
                            .id_salt("tar_datepicker")
                            .format("%d-%m-%y")
                            .show_icon(false),
                    );
                    time_picker(ui, app, TimeID::Target);
                });

                ui.end_row();
                ui.label("Result:");
                let cal_t = jiff::civil::time(
                    app.cal_time.0 as i8,
                    app.cal_time.1 as i8,
                    app.cal_time.2 as i8,
                    0,
                );
                let tar_t = jiff::civil::time(
                    app.target_time.0 as i8,
                    app.target_time.1 as i8,
                    app.target_time.2 as i8,
                    0,
                );
                let span = app.target_date.to_datetime(tar_t) - app.cal_date.to_datetime(cal_t);
                let span_f = span.total(jiff::Unit::Second).unwrap_or(0.0);
                ui.label(format!(
                    "{:.4}",
                    activity_left(
                        app.conv_input,
                        app.isotope.hl().as_secs_f32(),
                        span_f as f32
                    )
                ));
            });
    });
}

fn isotope_combo(app: &mut App, ui: &mut egui::Ui, name: &str) {
    egui::ComboBox::from_id_salt(name)
        .selected_text(app.isotope.display())
        .width(20.0)
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut app.isotope, Isotope::Tc99m, Isotope::Tc99m.display());
            ui.selectable_value(&mut app.isotope, Isotope::I131, Isotope::I131.display());
            ui.selectable_value(&mut app.isotope, Isotope::I123, Isotope::I123.display());
            ui.selectable_value(&mut app.isotope, Isotope::Lu177, Isotope::Lu177.display());
        });
}
// duration in seconds
fn parse_hl(duration: f32) -> String {
    if duration >= 86400.0 {
        format!("{:.2} days", duration / 86400.0)
    } else if duration >= 7200.0 {
        format!("{:.2} hours", duration / 3600.0)
    } else if duration >= 120.0 {
        format!("{:.2} minutes", duration / 60.0)
    } else {
        format!("{duration:.2} seconds")
    }
}

fn activity_left(n0: f32, hl: f32, t: f32) -> f32 {
    if hl != 0.0 {
        n0 * f32::powf(0.5, t / hl)
    } else {
        0.0
    }
}

fn time_picker(ui: &mut egui::Ui, app: &mut App, id: TimeID) {
    let time = match id {
        TimeID::Calibration => &mut app.cal_time,
        _ => &mut app.target_time,
    };

    let hm_time = format!("{}:{}", &time.0, &time.1);

    if ui.button(&hm_time).clicked() {
        match id {
            TimeID::Calibration => app.time_area_state.0 = true,
            _ => app.time_area_state.1 = true,
        }
    }

    let mut open = match id {
        TimeID::Calibration => app.time_area_state.0,
        _ => app.time_area_state.1,
    };

    egui::Window::new(id.display())
        .open(&mut open)
        .default_pos(ui.pointer_latest_pos().unwrap_or_default())
        .title_bar(false)
        .movable(true)
        .drag_area(egui::WindowDrag::Anywhere)
        .resizable(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(match id {
                        TimeID::Calibration => &mut app.cal_time.0,
                        _ => &mut app.target_time.0,
                    })
                    .range(0..=23)
                    .custom_formatter(|n, _| {
                        let n = n as u8;
                        format!("{n:02}")
                    }),
                );

                ui.label(":");
                ui.add(
                    egui::DragValue::new(match id {
                        TimeID::Calibration => &mut app.cal_time.1,
                        _ => &mut app.target_time.1,
                    })
                    .range(0..=59)
                    .custom_formatter(|n, _| {
                        let n = n as u8;
                        format!(" {n:02} ")
                    }),
                );
                if ui.button(" Now ").clicked() {
                    if let TimeID::Calibration = id {
                        app.cal_time = t_now();
                        app.time_area_state.0 = false;
                    } else {
                        app.target_time = t_now();
                        app.time_area_state.1 = false;
                    }
                }
            });

            if ui.button(" Ok ").clicked() {
                if let TimeID::Calibration = id {
                    app.cal_time.2 = 0;
                    app.time_area_state.0 = false;
                } else {
                    app.target_time.2 = 0;
                    app.time_area_state.1 = false;
                }
            }
        });
}

fn t_now() -> (u8, u8, u8) {
    let now = jiff::Zoned::now();
    (now.hour() as u8, now.minute() as u8, now.second() as u8)
}

fn d_now() -> jiff::civil::Date {
    jiff::Zoned::now().date()
}
