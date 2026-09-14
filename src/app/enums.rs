use std::time::Duration;

#[derive(PartialEq, Eq)]
pub enum WidgetSelection {
    Calculator,
    Converter,
    Info,
    None,
}

#[derive(Debug)]
pub enum TimeID {
    Calibration,
    Target,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Isotope {
    Tc99m,
    I131,
    I123,
    Lu177,
    Ra223,
    F18,
    Ga68,
    Cs137,
}

impl Isotope {
    pub fn hl(&self) -> Duration {
        match self {
            Self::Tc99m => Duration::from_secs_f32(21623.76),
            Self::I131 => Duration::from_secs_f32(693_351.4),
            Self::I123 => Duration::from_secs_f32(47603.52),
            Self::Lu177 => Duration::from_secs_f32(574_067.5),
            Self::Ra223 => Duration::from_secs_f32(988001.28),
            Self::F18 => Duration::from_secs_f32(6584.60),
            Self::Ga68 => Duration::from_secs_f32(4070.52),
            Self::Cs137 => Duration::from_mins(1.579984e7 as u64),
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
            Self::F18 => ("18", "F"),
            Self::Ga68 => ("68", "Ga"),
            Self::Cs137 => ("137", "Cs"),
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
