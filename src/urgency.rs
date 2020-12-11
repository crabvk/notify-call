use std::str::FromStr;

pub const URGENCY_LEVELS: [&str; 3] = ["low", "normal", "critical"];

pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl FromStr for Urgency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(Self::Low),
            "normal" => Ok(Self::Normal),
            "critical" => Ok(Self::Critical),
            // This case already covered by clap's possible_values
            _ => Err(format!(
                "Unknown urgency level \"{}\". Possible levels are low, normal and critical.",
                s
            )),
        }
    }
}
