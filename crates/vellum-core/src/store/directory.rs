use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

/**
    A trait representing a directory abstraction for file operations.
    This allows for different implementations, such as in-memory or filesystem-based storage.
    There will not be any subdirectories, only files directly under the root.
**/
pub trait Directory {
    fn list_all(&self) -> std::io::Result<Vec<String>>;
    // Trait objects that combine Read/Write and Seek.
    // We expose these via small helper traits because `dyn Read + Seek` is not allowed directly.
    fn create_output(&self, name: &str) -> std::io::Result<Box<dyn DirectoryWriter>>;
    fn open_input(&self, name: &str) -> std::io::Result<Box<dyn DirectoryReader>>;
    fn delete_file(&self, name: &str) -> std::io::Result<()>;
}

// Helper trait combining Write and Seek for trait objects.
pub trait DirectoryWriter: Write + Seek {}
impl<T: Write + Seek> DirectoryWriter for T {}

// Helper trait combining Read and Seek for trait objects.
pub trait DirectoryReader: Read + Seek {}
impl<T: Read + Seek> DirectoryReader for T {}

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
    fn create_output(&self, name: &str) -> std::io::Result<Box<dyn DirectoryWriter>> {
        let path = self.resolve(name);
        let file = std::fs::File::create(path)?;
        Ok(Box::new(file))
    }

    fn open_input(&self, name: &str) -> std::io::Result<Box<dyn DirectoryReader>> {
        let path = self.resolve(name);
        let file = std::fs::File::open(path)?;
        Ok(Box::new(file))
    }
    fn delete_file(&self, name: &str) -> std::io::Result<()> {
        let path = self.resolve(name);
        std::fs::remove_file(path)
    }
}
