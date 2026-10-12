use std::str::FromStr;

use crate::error::PrismError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OptimizationLevel {
    #[default]
    O0, // No Optimization (Default)
    O1, // Basic Optimization
    O2, // High Optimization
    O3, // Aggressive Optimization
    Os, // Optimization for Size
    Oz, // Aggressive Optimization for Size
}

impl FromStr for OptimizationLevel {
    type Err = PrismError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-O0" => Ok(Self::default()),
            "-O1" => Ok(Self::O1),
            "-O2" => Ok(Self::O2),
            "-O3" => Ok(Self::O3),
            "-Os" => Ok(Self::Os),
            "-Oz" => Ok(Self::Oz),

            _ => Err(PrismError::Parse(format!(
                "Invalid optimization flag: '{s}'."
            ))),
        }
    }
}

impl OptimizationLevel {
    pub fn opt_flag(self) -> Option<&'static str> {
        match self {
            Self::O0 => None,
            Self::O1 => Some("-passes=default<O1>"),
            Self::O2 => Some("-passes=default<O2>"),
            Self::O3 => Some("-passes=default<O3>"),
            Self::Os => Some("-passes=default<Os>"),
            Self::Oz => Some("-passes=default<Oz>"),
        }
    }

    pub fn llc_flag(self) -> &'static str {
        match self {
            Self::O0 => "-O=0",
            Self::O1 => "-O=1",
            Self::O2 | Self::Os | Self::Oz => "-O=2",
            Self::O3 => "-O=3",
        }
    }
}