use crate::app::enums::Isotope;

pub struct Calculator {
    input: f32,
    isotope: Isotope,
    cal_date: jiff::civil::Date,
    target_date: jiff::civil::Date,
    cal_time: (i8, i8, i8),
    target_time: (i8, i8, i8),

    // kb_cal_h_input: i32,
    // kb_cal_m_input: i32,
    // kb_target_h_input: i32,
    // kb_target_m_input: i32,
    kb_cal_h: app::keyboard::Keyboard,
    kb_cal_m: app::keyboard::Keyboard,
    kb_cal_h_open: bool,
    kb_cal_m_open: bool,

    kb_target_h: app::keyboard::Keyboard,
    kb_target_m: app::keyboard::Keyboard,
    kb_target_h_open: bool,
    kb_target_m_open: bool,

    kb_activity: app::keyboard::Keyboard,
    kb_activity_open: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        let date = jiff::Zoned::now().date();
        let time = t_now();
        Self {
            input: 0.0,
            isotope: Isotope::Tc99m,
            cal_date: date,
            target_date: date,
            cal_time: time,
            target_time: time,

            // kb_cal_h_input: 0,
            // kb_cal_m_input: 0,
            // kb_target_h_input: 0,
            // kb_target_m_input: 0,
            kb_cal_h: app::keyboard::Keyboard::new(
                Some("Hour".to_owned()),
                false,
                &time.0.to_string(),
                Some(23),
            ),
            kb_cal_m: app::keyboard::Keyboard::new(
                Some("Minute".to_owned()),
                false,
                &time.1.to_string(),
                Some(59),
            ),
            kb_cal_h_open: false,
            kb_cal_m_open: false,

            kb_target_h: app::keyboard::Keyboard::new(
                Some("Hour".to_owned()),
                false,
                &time.0.to_string(),
                Some(23),
            ),
            kb_target_m: app::keyboard::Keyboard::new(
                Some("Minute".to_owned()),
                false,
                &time.1.to_string(),
                Some(59),
            ),
            kb_target_h_open: false,
            kb_target_m_open: false,

            kb_activity: app::keyboard::Keyboard::new(None, true, "0", None),
            kb_activity_open: false,
        }
    }
}

impl Calculator {
    pub fn calculate(&self) -> f32 {
        let cal_t = jiff::civil::time(self.cal_time.0, self.cal_time.1, self.cal_time.2, 0);
        let tar_t = jiff::civil::time(
            self.target_time.0,
            self.target_time.1,
            self.target_time.2,
            0,
        );
        let span = self.target_date.to_datetime(tar_t) - self.cal_date.to_datetime(cal_t);
        let span_f = span.total(jiff::Unit::Second).unwrap_or(0.0);

        crate::app::activity_left(self.input, self.isotope.hl().as_secs_f32(), span_f as f32)
    }

    pub fn show(&mut self, ui: &egui::Ui, open: &mut bool) {
        egui::Window::new("Acivity calculator")
            .open(open)
            .resizable(false)
            .constrain_to(ui.available_rect_before_wrap())
            .title_bar(false)
            .movable(false)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
            .show(ui, |ui| {
                crate::app::isotope_combo(&mut self.isotope, ui, "second");

                ui.vertical(|ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.heading("Calibration");
                            grid(self, ui, &TimeID::Calibration);
                        });

                    let _w = ui.available_width() / 2.0;

                    ui.vertical_centered(|ui| {
                        ui.label("⬇️⬇️⬇️⬇️⬇️⬇️");
                    });

                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.heading("Target");
                            grid(self, ui, &TimeID::Target);
                        });
                });
            });

        if self.kb_cal_h_open {
            self.kb_cal_h.show(ui, &mut self.kb_cal_h_open);
        }

        if self.kb_cal_m_open {
            self.kb_cal_m.show(ui, &mut self.kb_cal_m_open);
        }

        if self.kb_target_h_open {
            self.kb_target_h.show(ui, &mut self.kb_target_h_open);
        }

        if self.kb_target_m_open {
            self.kb_target_m.show(ui, &mut self.kb_target_m_open);
        }

        if self.kb_activity_open {
            self.kb_activity.show(ui, &mut self.kb_activity_open);
        }
    }
}

fn grid(calc: &mut Calculator, ui: &mut egui::Ui, id: &TimeID) {
    egui::Grid::new(format!("{id:?}"))
        .num_columns(2)
        .spacing([8.0, 10.0])
        .striped(true)
        .min_col_width(100.0)
        .show(ui, |ui| {
            ui.label("Date");
            ui.add(
                egui_extras::DatePickerButton::new(match id {
                    TimeID::Calibration => &mut calc.cal_date,
                    TimeID::Target => &mut calc.target_date,
                })
                .id_salt(match id {
                    TimeID::Calibration => "time_picker_cal",
                    TimeID::Target => "time_picker_target",
                })
                .format("%d-%m-%y")
                .show_icon(false)
                .arrows(false),
            );
            ui.end_row();

            ui.label("Time");
            time_picker(ui, calc, id);
            ui.end_row();

            ui.label("Activity");
            match id {
                TimeID::Calibration => {
                    if crate::app::is_kb_active(ui) {
                        let a = ui.add(
                            egui::Button::new(format!("{}", calc.input))
                                .min_size(egui::vec2(48.0, 10.0)),
                        );
                        if a.clicked() {
                            calc.kb_activity_open = true;
                        }

                        calc.input = calc.kb_activity.get_f() as f32;
                    } else {
                        ui.add(
                            egui::DragValue::new(&mut calc.input)
                                .range(0.0..=1000000.0)
                                .max_decimals(4),
                        );
                    }
                }
                TimeID::Target => {
                    ui.label(format!("{:.4}", calc.calculate()));
                }
            }
        });
}

fn t_now() -> (i8, i8, i8) {
    let now = jiff::Zoned::now();
    (now.hour(), now.minute(), now.second())
}

use crate::app::{self, TimeID};
fn time_picker(ui: &mut egui::Ui, calc: &mut Calculator, id: &TimeID) {
    ui.horizontal_centered(|ui| {
        //ui.add_space(10.0);

        let b_size = egui::vec2(48.0, 10.0);
        // Hour
        if crate::app::is_kb_active(ui) {
            let h = ui.add(
                egui::Button::new(format!(
                    "{}",
                    match id {
                        TimeID::Calibration => calc.cal_time.0,
                        TimeID::Target => calc.target_time.0,
                    }
                ))
                .min_size(b_size),
            );

            if h.clicked() {
                match id {
                    TimeID::Calibration => calc.kb_cal_h_open = true,
                    TimeID::Target => calc.kb_target_h_open = true,
                }
            }
            match id {
                TimeID::Calibration => calc.cal_time.0 = calc.kb_cal_h.get_i() as i8,
                TimeID::Target => calc.target_time.0 = calc.kb_target_h.get_i() as i8,
            }
        } else {
            ui.add(
                egui::DragValue::new(match id {
                    TimeID::Calibration => &mut calc.cal_time.0,
                    TimeID::Target => &mut calc.target_time.0,
                })
                .range(0..=23)
                .custom_formatter(|n, _| {
                    let n = n as i8;
                    format!("{n:02}")
                }),
            );
        }

        // Minute
        if crate::app::is_kb_active(ui) {
            let m = ui.add(
                egui::Button::new(format!(
                    "{}",
                    match id {
                        TimeID::Calibration => calc.cal_time.1,
                        TimeID::Target => calc.target_time.1,
                    }
                ))
                .min_size(b_size),
            );

            if m.clicked() {
                match id {
                    TimeID::Calibration => calc.kb_cal_m_open = true,
                    TimeID::Target => calc.kb_target_m_open = true,
                }
            }
            match id {
                TimeID::Calibration => calc.cal_time.1 = calc.kb_cal_m.get_i() as i8,
                TimeID::Target => calc.target_time.1 = calc.kb_target_m.get_i() as i8,
            }
        } else {
            ui.add(
                egui::DragValue::new(match id {
                    TimeID::Calibration => &mut calc.cal_time.1,
                    TimeID::Target => &mut calc.target_time.1,
                })
                .range(0..=59)
                .custom_formatter(|n, _| {
                    let n = n as i8;
                    format!(" {n:02} ")
                }),
            );
        }

        if ui.button("Now").clicked() {
            match id {
                TimeID::Calibration => {
                    calc.cal_time = t_now();
                    calc.kb_cal_h = app::keyboard::Keyboard::new(
                        Some("Hour".to_owned()),
                        false,
                        &calc.cal_time.0.to_string(),
                        Some(23),
                    );
                    calc.kb_cal_m = app::keyboard::Keyboard::new(
                        Some("Minute".to_owned()),
                        false,
                        &calc.cal_time.1.to_string(),
                        Some(59),
                    );
                }
                TimeID::Target => {
                    calc.target_time = t_now();
                    calc.kb_target_h = app::keyboard::Keyboard::new(
                        Some("Hour".to_owned()),
                        false,
                        &calc.target_time.0.to_string(),
                        Some(23),
                    );
                    calc.kb_target_m = app::keyboard::Keyboard::new(
                        Some("Minute".to_owned()),
                        false,
                        &calc.target_time.1.to_string(),
                        Some(59),
                    );
                }
            }
        }
    });
}
