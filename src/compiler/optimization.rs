use std::str::FromStr;

use inkwell::OptimizationLevel as LLVMOptimizationLevel;

use crate::error::PrismError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OptimizationLevel {
    #[default]
    O0,
    O1,
    O2,
    O3,
    Os,
    Oz,
}

impl FromStr for OptimizationLevel {
    type Err = PrismError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-O0" => Ok(Self::O0),
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
    pub fn pass_pipeline(self) -> Option<&'static str> {
        match self {
            Self::O0 => None,
            Self::O1 => Some("default<O1>"),
            Self::O2 => Some("default<O2>"),
            Self::O3 => Some("default<O3>"),
            Self::Os => Some("default<Os>"),
            Self::Oz => Some("default<Oz>"),
        }
    }

    pub fn llvm_level(self) -> LLVMOptimizationLevel {
        match self {
            Self::O0 => LLVMOptimizationLevel::None,
            Self::O1 => LLVMOptimizationLevel::Less,
            Self::O2 | Self::Os | Self::Oz => LLVMOptimizationLevel::Default,
            Self::O3 => LLVMOptimizationLevel::Aggressive,
        }
    }
}
