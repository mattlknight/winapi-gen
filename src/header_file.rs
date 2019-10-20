use failure::{format_err, Error};
use std::fs;
use std::io::{Read, BufReader};
use std::path::{Path, PathBuf};
use log::{error, info};

pub type ParseResult<T> = Result<T, Error>;

pub struct HeaderFile {
    pub file_path: PathBuf,
    pub file_contents: Option<String>,
}

impl HeaderFile {
    pub fn new(path: &Path) -> ParseResult<Self> {
        if path.exists() {
            info!("Found Header File: {:?} [{} Bytes]", path, fs::metadata(path)?.len());
        } else {
            error!("Missing Header File: {:?}", path);
            return Err(format_err!("Missing Header File: {:?}", path));
        }
        Ok(Self {
            file_path: path.to_path_buf(),
            file_contents: None,
        })
    }
    pub fn read_contents(&mut self) -> ParseResult<()> {
        let file = fs::OpenOptions::new()
                        .read(true)
                        .write(false)
                        .create(false)
                        .open(&self.file_path)?;
        let mut reader = BufReader::new(file);
        let mut buff = String::new();
        reader.read_to_string(&mut buff)?;
        self.file_contents = Some(buff);
        Ok(())
    }
    pub fn parse(&mut self) -> ParseResult<()> {
        if let Some(buff) = &self.file_contents {
            for (index, line) in buff.lines().enumerate() {
                info!("Line:{}: [{}]", index, line);
                break;
            }
        } else {
            error!("Must run read_contents() first!");
            return Err(format_err!("Must run read_contents() first!"));
        }

        Ok(())
    }
}
