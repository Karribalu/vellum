use std::io::Write;
use std::path::{Path, PathBuf};

/**
    A trait representing a directory abstraction for file operations.
    This allows for different implementations, such as in-memory or filesystem-based storage.
    There will not be any subdirectories, only files directly under the root.
**/
pub trait Directory {
    fn list_all(&self) -> std::io::Result<Vec<String>>;
    fn create_output(&self, name: &str) -> std::io::Result<Box<dyn Write>>;
    fn open_input(&self, name: &str) -> std::io::Result<Box<dyn std::io::Read>>;
    fn delete_file(&self, name: &str) -> std::io::Result<()>;
}

pub struct FSDirectory {
    root: PathBuf,
}

impl FSDirectory {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root: root.to_path_buf(),
        })
    }
    // Helper to resolve a file path within the directory.
    fn resolve(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}
impl Directory for FSDirectory {
    fn list_all(&self) -> std::io::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    files.push(name.to_string());
                }
            }
        }
        Ok(files)
    }
    fn create_output(&self, name: &str) -> std::io::Result<Box<dyn Write>> {
        let path = self.resolve(name);
        let file = std::fs::File::create(path)?;
        Ok(Box::new(file))
    }

    fn open_input(&self, name: &str) -> std::io::Result<Box<dyn std::io::Read>> {
        let path = self.resolve(name);
        let file = std::fs::File::open(path)?;
        Ok(Box::new(file))
    }
    fn delete_file(&self, name: &str) -> std::io::Result<()> {
        let path = self.resolve(name);
        std::fs::remove_file(path)
    }
}
