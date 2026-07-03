use std::time::Duration;

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub enum WidgetSelection {
    Calculator,
    Converter,
    Info,
    NONE,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum TimeID {
    Calibration,
    Target,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq)]
pub enum Unit {
    MegaBq,
    GigaBq,
    MicroCi,
    MiliCi,
}

impl Unit {
    pub fn multi(&self) -> f32 {
        match self {
            Self::MegaBq => 1.0,
            Self::GigaBq => 1000.0,
            Self::MicroCi => 0.037,
            Self::MiliCi => 37.0,
        }
    }

    pub fn display(&self) -> &str {
        match self {
            Self::MegaBq => "MBq",
            Self::GigaBq => "GBq",
            Self::MicroCi => "µCi",
            Self::MiliCi => "mCi",
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq)]
pub enum Isotope {
    Tc99m,
    I131,
    I123,
    Lu177,
    Ra223,
}

impl Isotope {
    pub fn energy(&self) -> &'static str {
        match self {
            Self::Tc99m => "γ: 140.5 keV",
            Self::I131 => "β-: 606 keV\nγ: 364.4 keV: ",
            Self::I123 => "β-: 1.228 MeV (electron capture)\nγ: 159.0 keV",
            Self::Lu177 => "β-: 496.8 keV\nγ: 321.3 keV",
            Self::Ra223 => "α: ~5.7 MeV",
        }
    }

    pub fn hl(&self) -> Duration {
        match self {
            Self::Tc99m => Duration::from_secs_f32(21625.92),
            Self::I131 => Duration::from_secs_f32(693377.28),
            Self::I123 => Duration::from_secs_f32(47602.8),
            Self::Lu177 => Duration::from_secs_f32(574067.52),
            Self::Ra223 => Duration::from_secs_f32(988122.24),
        }
    }

    pub fn display(&self) -> egui::text::LayoutJob {
        let mut job = egui::text::LayoutJob::default();
        let style = egui::style::Style::default();

        let tx: (&str, &str) = match self {
            Self::Tc99m => ("99m", "Tc"),
            Self::I131 => ("131", "I"),
            Self::I123 => ("123", "I"),
            Self::Lu177 => ("177", "Lu"),
            Self::Ra223 => ("223", "Ra"),
        };

        egui::RichText::new(tx.0)
            .strong()
            .size(10.0)
            .color(style.visuals.text_color())
            .append_to(
                &mut job,
                &style,
                egui::FontSelection::Default,
                egui::Align::Min,
            );

        egui::RichText::new(tx.1)
            .strong()
            .size(20.0)
            .color(style.visuals.text_color())
            .append_to(
                &mut job,
                &style,
                egui::FontSelection::Default,
                egui::Align::Min,
            );

        job
    }
}
