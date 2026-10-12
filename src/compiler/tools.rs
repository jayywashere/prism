use std::{
    env,
    path::{Path, PathBuf},
};

use crate::error::{PrismError, ToolchainError};

pub struct Toolchain {
    pub llvm_as: PathBuf,
    pub opt: Option<PathBuf>,
    pub llc: PathBuf,
    pub linker: PathBuf,
}

impl Toolchain {
    pub fn new(llvm_as: PathBuf, opt: Option<PathBuf>, llc: PathBuf, linker: PathBuf) -> Self {
        Self {
            llvm_as,
            opt,
            llc,
            linker,
        }
    }

    pub fn discover() -> Result<Self, PrismError> {
        let linker_names = ["gcc", "clang", "cc"];
        let mut found_linker = None;

        for name in &linker_names {
            if let Some(path) = Self::find_tool(name) {
                found_linker = Some(path);
                break;
            }
        }

        Ok(Self {
            llvm_as: Self::find_tool("llvm-as")
                .ok_or(ToolchainError::MissingTool("llvm-as".into()))
                .map_err(|err| PrismError::Toolchain(err))?,
            opt: Self::find_tool("opt"),
            llc: Self::find_tool("llc")
                .ok_or(ToolchainError::MissingTool("llc".into()))
                .map_err(|err| PrismError::Toolchain(err))?,
            linker: found_linker
                .ok_or(ToolchainError::MissingTool("gcc, clang, or cc".into()))
                .map_err(|err| PrismError::Toolchain(err))?,
        })
    }

    fn find_tool(name: &str) -> Option<PathBuf> {
        let path_var = env::var_os("PATH")?;
        let paths = env::split_paths(&path_var);

        let target_name = if cfg!(windows) && !name.ends_with(".exe") {
            format!("{}.exe", name)
        } else {
            name.to_string()
        };

        for path in paths {
            let merged = path.join(&target_name);

            if merged.is_file() && Self::is_executable(&merged) {
                return Some(merged);
            }
        }

        None
    }

    #[cfg(unix)]
    fn is_executable(path: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(metadata) = path.metadata() {
            return metadata.permissions().mode() & 0o111 != 0;
        }

        false
    }

    #[cfg(not(unix))]
    fn is_executable(_path: &Path) -> bool {
        true
    }
}
