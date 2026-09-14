use crate::app;
use crate::app::enums::Unit;

pub struct Converter {
    input: f32,
    unit: app::enums::Unit,
    kb: app::keyboard::Keyboard,
    modal_open: bool,
}

impl Default for Converter {
    fn default() -> Self {
        Self {
            input: 0.0,
            unit: app::enums::Unit::MegaBq,
            kb: app::keyboard::Keyboard::new(None, true, "0", None),
            modal_open: false,
        }
    }
}

impl Converter {
    pub fn show(&mut self, ui: &egui::Ui, open: &mut bool) {
        egui::Window::new("Unit converter")
            .open(open)
            .resizable(false)
            .constrain_to(ui.available_rect_before_wrap())
            .title_bar(false)
            .movable(false)
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    //ui.heading("Unit converter");

                    egui::Frame::new()
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            egui::Grid::new("unit_grid")
                                .num_columns(2)
                                .spacing([8.0, 10.0])
                                .striped(true)
                                .min_col_width(100.0)
                                .show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        egui::ComboBox::from_id_salt("unit_combo")
                                            .selected_text(self.unit.display())
                                            .width(100.0)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    &mut self.unit,
                                                    Unit::MegaBq,
                                                    "MBq",
                                                );
                                                ui.selectable_value(
                                                    &mut self.unit,
                                                    Unit::GigaBq,
                                                    "GBq",
                                                );
                                                ui.selectable_value(
                                                    &mut self.unit,
                                                    Unit::MicroCi,
                                                    "µCi",
                                                );
                                                ui.selectable_value(
                                                    &mut self.unit,
                                                    Unit::MiliCi,
                                                    "mCi",
                                                );
                                            });
                                    });

                                    if crate::app::is_kb_active(ui) {
                                        let b = ui.add(
                                            egui::Button::new(format!("{}", self.input))
                                                .min_size(egui::Vec2 { x: 50.0, y: 10.0 }),
                                        );
                                        if b.clicked() {
                                            self.modal_open = true;
                                        }

                                        self.input = self.kb.get_f() as f32;
                                    } else {
                                        ui.add(
                                            egui::DragValue::new(&mut self.input)
                                                .range(0.0..=1000000.0)
                                                .max_decimals(4)
                                                .update_while_editing(false),
                                        );
                                    }

                                    ui.end_row();

                                    ui.label("MBq");
                                    ui.label(format!(
                                        "{:.3}",
                                        self.input * self.unit.multi() / Unit::MegaBq.multi()
                                    ));
                                    ui.end_row();

                                    ui.label("GBq");
                                    ui.label(format!(
                                        "{:.3}",
                                        self.input * self.unit.multi() / Unit::GigaBq.multi()
                                    ));
                                    ui.end_row();

                                    ui.label("µCi");
                                    ui.label(format!(
                                        "{:.3}",
                                        self.input * self.unit.multi() / Unit::MicroCi.multi()
                                    ));
                                    ui.end_row();

                                    ui.label("mCi");
                                    ui.label(format!(
                                        "{:.3}",
                                        self.input * self.unit.multi() / Unit::MiliCi.multi()
                                    ));
                                    ui.end_row();
                                });

                            if self.modal_open {
                                self.kb.show(ui, &mut self.modal_open);
                            }
                        });
                });
            });
    }
}
