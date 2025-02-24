use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::read_to_string;

pub struct FileIO<'a> {
    path:    &'a str
}

impl<'a> FileIO<'a> {

    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    pub fn read(self) -> String {
        let mut content: String = String::from("");
        if Path::new(self.path).exists() {
            content = read_to_string(self.path).expect("Error in file");
        }
        content
    }

    pub fn write(self, content: String) -> Result<()> {
        let mut file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path)?;

        file.write_all(content.as_bytes())?;
        file.sync_all()?;

        Ok(())
    }
}