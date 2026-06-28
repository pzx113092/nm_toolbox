mod enums;
use egui::{CornerRadius, Margin};
use egui_plot::{Line, Plot, PlotPoints};
use enums::{Isotope, Unit};

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
    cal_time: i32,
    target_date: jiff::civil::Date,
    target_time: i32,
    tooltip_text: Option<String>,
    tooltip_until: Option<f64>,
}

impl Default for App {
    fn default() -> Self {
        let unit = Unit::MegaBq;
        let conv_input = 0.0;
        let isotope = Isotope::Tc99m;
        let cal_date = d_now();
        let cal_time = t_now();

        Self {
            style: false,
            settings: false,
            unit,
            conv_input,
            isotope,
            cal_date,
            cal_time,
            target_time: cal_time,
            target_date: cal_date,
            tooltip_text: None,
            tooltip_until: None,
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
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
                if ui.button("🔧 Settings").clicked() {
                    self.settings = true;
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                egui::ScrollArea::vertical()
                    .max_width(600.0)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            app_frame(&visuals).show(ui, |ui| {
                                converter(self, ui);
                            });
                            app_frame(&visuals).show(ui, |ui| {
                                isotope_info(self, ui);
                            });
                            app_frame(&visuals).show(ui, |ui| {
                                calculator(self, ui);
                            });
                        });
                    });
            });
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
                            .format(String::new()),
                    );
                    if ui.button("today").clicked() {
                        app.cal_date = d_now();
                    }

                    time_picker(ui, &mut app.cal_time);

                    if ui.button("now").clicked() {
                        app.cal_time = t_now();
                    }
                });

                ui.end_row();

                ui.label("Target:");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.add(
                        egui_extras::DatePickerButton::new(&mut app.target_date)
                            .id_salt("target_datepicker")
                            .format(String::new()),
                    );
                    if ui.button("today").clicked() {
                        app.target_date = d_now();
                    }
                    time_picker(ui, &mut app.target_time);
                    if ui.button("now").clicked() {
                        app.target_time = t_now();
                    }
                });
                ui.end_row();
                ui.label("Result:");
                let cal_t = i32_to_hms(app.cal_time);
                let tar_t = i32_to_hms(app.target_time);
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

fn time_picker(ui: &mut egui::Ui, time: &mut i32) {
    ui.horizontal(|ui| {
        //let mut hour_str = format!("{:02}", &app.cal_time.0);
        //let mut h_buf = egui::TextBuffer::insert_text(&mut self, text, char_index)
        ui.add(
            egui::DragValue::new(time)
                .range(0..=((60 * 60 * 24) - 1))
                .custom_formatter(|n, _| {
                    let n = n as i32;
                    let hours = n / (60 * 60);
                    let mins = (n / 60) % 60;
                    let secs = n % 60;
                    format!("{hours:02}:{mins:02}:{secs:02}")
                })
                .custom_parser(|s| {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        parts[0]
                            .parse::<i32>()
                            .and_then(|h| {
                                let m = parts[1].parse::<i32>()?;
                                parts[2]
                                    .parse::<i32>()
                                    .map(|s| ((h * 60 * 60) + (m * 60) + s) as f64)
                            })
                            .ok()
                    } else {
                        None
                    }
                }),
        );
    });
}

fn t_now() -> i32 {
    let now = (
        jiff::Zoned::now().hour() as i32,
        jiff::Zoned::now().minute() as i32,
        jiff::Zoned::now().second() as i32,
    );
    now.0 * 3600 + now.1 * 60 + now.2
}

fn d_now() -> jiff::civil::Date {
    jiff::Zoned::now().date()
}

fn i32_to_hms(s: i32) -> jiff::civil::Time {
    let mut sec = s;
    let hours = sec / 3600;
    sec -= hours * 3600;
    let minutes = sec / 60;
    sec -= minutes * 60;

    jiff::civil::time(hours as i8, minutes as i8, sec as i8, 0)
}
