use std::path::{Path, PathBuf};
use std::{env, error, fs};

pub struct ReplaceFile {
    path: PathBuf,
}

impl ReplaceFile {
    pub fn new(filename: &str) -> Self {
        let filepath = Path::new(filename);

        let path = if filepath.has_root() {
            PathBuf::from(filepath)
        } else {
            let dir = dirs::runtime_dir().unwrap_or_else(|| {
                eprintln!(
                    "Warning: XDG_RUNTIME_DIR not set in the environment, fallback to TMPDIR."
                );
                env::temp_dir()
            });
            dir.join(filepath)
        };

        Self { path }
    }

    pub fn read_or(&self, default_nid: u32) -> Result<u32, String> {
        match Self::try_read_or(self, default_nid) {
            Ok(nid) => Ok(nid),
            Err(e) => Err(format!(
                "Error reading from replace file \"{}\"; {}.",
                self.path(),
                e
            )),
        }
    }

    fn try_read_or(&self, default_nid: u32) -> Result<u32, Box<dyn error::Error>> {
        if self.path.exists() {
            let s = self.path.to_str().unwrap();
            Ok(fs::read_to_string(s)?.parse()?)
        } else {
            Ok(default_nid)
        }
    }

    pub fn write(&self, nid: u32) -> Result<(), String> {
        match fs::write(&self.path, nid.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error writing to replace file \"{}\"; {}.",
                self.path(),
                e
            )),
        }
    }

    pub fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }
}
