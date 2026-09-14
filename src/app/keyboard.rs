pub struct Keyboard {
    name: Option<String>,
    decimal: bool,
    input: String,
    value_f: f64,
    value_i: i32,
    limit: Option<i32>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            name: None,
            decimal: false,
            input: "0".to_owned(),
            value_f: 0.0,
            value_i: 0,
            limit: None,
        }
    }
}

impl Keyboard {
    pub fn get_f(&self) -> f64 {
        self.value_f
    }

    pub fn get_i(&self) -> i32 {
        self.value_i
    }

    pub fn new(name: Option<String>, decimal: bool, input: &str, limit: Option<i32>) -> Self {
        Self {
            name,
            decimal,
            input: input.to_owned(),
            value_f: input.parse().unwrap_or_default(),
            value_i: input.parse().unwrap_or_default(),
            limit,
        }
    }

    pub fn show(&mut self, ui: &egui::Ui, open: &mut bool) {
        egui::Modal::new(egui::Id::from("modal")).show(ui.ctx(), |ui| {
            if self.input.is_empty() {
                self.input = "0".to_owned();
            }

            if let Some(n) = &self.name {
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        ui.label(n);
                    },
                );
            }

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    let b = ui.add(egui::Button::new("⌫").frame(false));

                    if b.clicked() {
                        input_digit(self, "back");
                    }
                    ui.label(self.input.clone());
                },
            );

            ui.separator();
            ui.vertical(|ui| {
                egui::Grid::new("numeric_keyboard")
                    .striped(false)
                    .min_row_height(56.0)
                    .show(ui, |ui| {
                        if ui
                            .add(egui::Button::new("7").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "7");
                        }
                        if ui
                            .add(egui::Button::new("8").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "8");
                        }
                        if ui
                            .add(egui::Button::new("9").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "9");
                        }

                        ui.end_row();

                        if ui
                            .add(egui::Button::new("4").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "4");
                        }
                        if ui
                            .add(egui::Button::new("5").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "5");
                        }
                        if ui
                            .add(egui::Button::new("6").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "6");
                        }

                        ui.end_row();

                        if ui
                            .add(egui::Button::new("1").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "1");
                        }
                        if ui
                            .add(egui::Button::new("2").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "2");
                        }
                        if ui
                            .add(egui::Button::new("3").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "3");
                        }

                        ui.end_row();

                        if ui
                            .add(egui::Button::new("C").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "c");
                        }
                        if ui
                            .add(egui::Button::new("0").min_size(egui::Vec2::new(52.0, 52.0)))
                            .clicked()
                        {
                            input_digit(self, "0");
                        }
                        if self.decimal
                            && ui
                                .add(egui::Button::new("∙").min_size(egui::Vec2::new(52.0, 52.0)))
                                .clicked()
                        {
                            input_digit(self, ".");
                        }
                    });
            });

            ui.separator();
            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Save").clicked() {
                        if self.decimal {
                            match self.input.chars().last() {
                                Some(last_char) if &format!("{last_char}") == "." => {
                                    input_digit(self, "0");
                                }
                                _ => {}
                            }
                            self.value_f = self.input.parse().unwrap_or(0.0);
                        } else {
                            let i = self.input.parse().unwrap_or(0);
                            match self.limit {
                                Some(v) => {
                                    if i > v {
                                        self.value_i = v;
                                    } else {
                                        self.value_i = i;
                                    }
                                }
                                _ => self.value_i = i,
                            }
                        }
                        *open = false;
                    }

                    if ui.button("Exit").clicked() {
                        if self.decimal {
                            self.input = format!("{}", self.value_f);
                        } else {
                            self.input = format!("{}", self.value_i);
                        }
                        *open = false;
                    }
                },
            );
        });
    }
}

pub fn input_digit(k: &mut Keyboard, i: &str) {
    match i {
        "c" => k.input = "0".to_owned(),
        "." => {
            if !k.input.contains('.') && k.input.len() < 11 {
                k.input.push_str(i);
            }
        }
        "back" => {
            k.input.pop();
        }
        _ => {
            if k.input.len() < 12 {
                if k.input == "0" {
                    k.input = i.to_owned();
                } else {
                    k.input.push_str(i);
                }
            }
        }
    }
}
