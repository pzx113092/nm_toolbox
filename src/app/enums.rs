use std::time::Duration;

#[derive(serde::Deserialize, serde::Serialize)]
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
}

impl Isotope {
    pub fn hl(&self) -> Duration {
        match self {
            Self::Tc99m => Duration::from_secs_f32(21624.76),
            Self::I131 => Duration::from_secs_f32(692928.0),
            Self::I123 => Duration::from_secs_f32(47602.8),
            Self::Lu177 => Duration::from_secs_f32(578880.0),
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
