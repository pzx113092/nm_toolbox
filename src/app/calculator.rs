use std::mem;

use crate::app::enums::Isotope;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Calculator {
    input: f64,
    isotope: Isotope,
    cal_date: jiff::civil::Date,
    target_date: jiff::civil::Date,
    cal_time: (i8, i8, i8),
    target_time: (i8, i8, i8),
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
        }
    }
}

impl Calculator {
    pub fn calculate(&self) -> f64 {
        let cal_t = jiff::civil::time(self.cal_time.0, self.cal_time.1, self.cal_time.2, 0);
        let tar_t = jiff::civil::time(
            self.target_time.0,
            self.target_time.1,
            self.target_time.2,
            0,
        );

        let cal_dt = self.target_date.to_datetime(tar_t);
        let tar_dt = self.cal_date.to_datetime(cal_t);

        let seconds = duration_in_seconds(cal_dt, tar_dt);

        crate::app::activity_left(self.input, self.isotope.hl().as_secs_f64(), seconds)
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
                        if ui.button("🔀").clicked() {
                            mem::swap(&mut self.cal_time, &mut self.target_time);
                            mem::swap(&mut self.cal_date, &mut self.target_date);
                        }
                    });

                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.heading("Target");
                            grid(self, ui, &TimeID::Target);
                        });
                });
            });
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
            ui.horizontal(|ui| {
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
                if ui.button("Now").clicked() {
                    match id {
                        TimeID::Calibration => calc.cal_date = jiff::Zoned::now().date(),
                        TimeID::Target => calc.target_date = jiff::Zoned::now().date(),
                    }
                }
            });

            ui.end_row();

            ui.label("Time");
            time_picker(ui, calc, id);
            ui.end_row();

            ui.label("Activity");
            match id {
                TimeID::Calibration => {
                    ui.add(
                        egui::DragValue::new(&mut calc.input)
                            .range(0.0..=1000000.0)
                            .max_decimals(4),
                    );
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

use crate::app::TimeID;
fn time_picker(ui: &mut egui::Ui, calc: &mut Calculator, id: &TimeID) {
    ui.horizontal_centered(|ui| {
        //ui.add_space(10.0);
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

        if ui.button("Now").clicked() {
            match id {
                TimeID::Calibration => calc.cal_time = t_now(),
                TimeID::Target => calc.target_time = t_now(),
            }
        }
    });
}

fn duration_in_seconds(dt1: jiff::civil::DateTime, dt2: jiff::civil::DateTime) -> f64 {
    let span = dt1 - dt2;

    // 2. Explicitly tell Jiff to treat all days as 86,400 seconds
    span.total(jiff::SpanTotal::from(jiff::Unit::Second).days_are_24_hours())
        .unwrap_or_default()
}
