use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Profile {
    Eco,
    Balanced,
    Performance,
}

impl Profile {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "eco" | "powersave" => Some(Self::Eco),
            "balanced" | "auto" | "default" => Some(Self::Balanced),
            "performance" | "gaming" | "perf" => Some(Self::Performance),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eco => "eco",
            Self::Balanced => "balanced",
            Self::Performance => "performance",
        }
    }

    pub fn governor(&self) -> &'static str {
        match self {
            Self::Eco => "powersave",
            Self::Balanced => "schedutil",
            Self::Performance => "performance",
        }
    }

    pub fn turbo(&self) -> bool {
        match self {
            Self::Eco => false,
            Self::Balanced => true,
            Self::Performance => true,
        }
    }

    pub fn epp(&self) -> &'static str {
        match self {
            Self::Eco => "power",
            Self::Balanced => "balance_performance",
            Self::Performance => "performance",
        }
    }

    pub fn swappiness(&self) -> u8 {
        match self {
            Self::Eco => 10,
            Self::Balanced => 60,
            Self::Performance => 10,
        }
    }

    pub fn io_scheduler(&self) -> &'static str {
        match self {
            Self::Eco => "kyber",
            Self::Balanced => "mq-deadline",
            Self::Performance => "none",
        }
    }

    pub fn gpu_power(&self) -> &'static str {
        match self {
            Self::Eco => "low",
            Self::Balanced => "auto",
            Self::Performance => "high",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Eco => "\u{1f50b}",
            Self::Balanced => "\u{2696}\u{fe0f}",
            Self::Performance => "\u{26a1}",
        }
    }
}
