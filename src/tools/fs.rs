use std::fs::File;
use std::io::Write;
use std::io::Result;
use std::fs::OpenOptions;


pub struct FileIO<'a> {
    path:    &'a str
}

impl<'a> FileIO<'a> {

    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    pub fn read(self) {
        
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