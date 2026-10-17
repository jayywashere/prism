#[cfg(unix)]
use std::path::Path;
use std::{env, path::PathBuf, process::Command};

use crate::error::PrismError;

pub struct Linker {
    executable: PathBuf,
}

impl Linker {
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
        }
    }

    pub fn discover() -> Result<Self, PrismError> {
        for name in ["cc", "clang", "gcc"] {
            if let Some(path) = Self::find_executable(name) {
                return Ok(Self::new(path));
            }
        }

        Err(PrismError::Compilation(
            "No linker found. Specify one with --linker.".into(),
        ))
    }

    pub fn link(&self, object: &Path, output: &Path) -> Result<(), PrismError> {
        let status = Command::new(&self.executable)
            .arg(object)
            .arg("-o")
            .arg(output)
            .status()
            .map_err(PrismError::Io)?;

        if !status.success() {
            return Err(PrismError::Compilation(format!(
                "Linker '{}' failed with status {status}.",
                self.executable.display()
            )));
        }

        Ok(())
    }

    fn find_executable(name: &str) -> Option<PathBuf> {
        let paths = env::var_os("PATH")?;
        let paths = env::split_paths(&paths);

        let target_name = if cfg!(windows) && !name.ends_with(".exe") {
            format!("{name}.exe")
        } else {
            name.to_owned()
        };

        paths
            .map(|path| path.join(&target_name))
            .find(|path| path.is_file() && Self::is_executable(path))
    }

    #[cfg(unix)]
    fn is_executable(path: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;

        path.metadata()
            .is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
    }

    #[cfg(not(unix))]
    fn is_executable(_path: &Path) -> bool {
        true
    }
}
