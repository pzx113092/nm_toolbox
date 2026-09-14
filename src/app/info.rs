use crate::app::enums::Isotope;

pub struct Info {
    isotope: Isotope,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            isotope: Isotope::Tc99m,
        }
    }
}

impl Info {
    pub fn show(&mut self, ui: &egui::Ui, open: &mut bool) {
        egui::Window::new("Info")
            .open(open)
            .resizable(false)
            .constrain_to(ui.available_rect_before_wrap())
            .title_bar(false)
            .movable(false)
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        crate::app::isotope_combo(&mut self.isotope, ui, "isotope_combo_info");
                        ui.vertical(|ui| {
                            egui::Grid::new("isotope_grid")
                                .num_columns(2)
                                .spacing([8.0, 10.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    let half_life = self.isotope.hl().as_secs_f32();

                                    ui.label("Half life:");
                                    ui.label(parse_hl(half_life));
                                    ui.end_row();

                                    ui.label("Full decay:");
                                    ui.label(parse_hl(half_life * 10.0));
                                    ui.end_row();
                                });
                        });
                    });
            });
    }
}

fn parse_hl(duration: f32) -> String {
    if duration >= 3.15576e7 {
        format!("{:.2} years", duration / 3.15576e7)
    } else if duration >= 86400.0 {
        format!("{:.2} days", duration / 86400.0)
    } else if duration >= 7200.0 {
        format!("{:.2} hours", duration / 3600.0)
    } else if duration >= 120.0 {
        format!("{:.2} minutes", duration / 60.0)
    } else {
        format!("{duration:.2} seconds")
    }
}
