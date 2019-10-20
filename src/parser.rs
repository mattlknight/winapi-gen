use failure::{format_err, Error};
use std::fs;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use log::{debug, error, info};

pub type ParseResult<T> = Result<T, Error>;
pub type BufferResult<'buffer> = Result<ParsedLine<'buffer>, Error>; // Returns Ok((line_num, bytes_read_usize, reference to buffer)) or Err

const BUFFER_SIZE: usize = 128;

pub struct ParsedLine<'buffer> {
    pub line_num: usize, 
    pub bytes_read: usize, 
    pub buffer: &'buffer str,
}

pub struct Parser {
    file_path: PathBuf,
    reader: Option<BufReader<fs::File>>,
    buffer: String,
    line_num: usize,
}

impl Parser {
    pub fn new<T: AsRef<Path>>(path: T) -> ParseResult<Self>  {
        let path = path.as_ref();
        if path.exists() {
            info!("Found File: {:?} [{} Bytes]", path, fs::metadata(path)?.len());
        } else {
            error!("Missing  File: {:?}", path);
            return Err(format_err!("Missing File: {:?}", path));
        }
        Ok(Self {
            file_path: path.to_path_buf(),
            reader: None,
            buffer: String::with_capacity(BUFFER_SIZE),
            line_num: 0,
        })
    }

    pub fn open(&mut self) -> ParseResult<()> {
        let file = fs::OpenOptions::new()
                        .read(true)
                        .write(false)
                        .create(false)
                        .open(&self.file_path)?;
        self.reader = Some(BufReader::new(file));
        Ok(())
    }

    pub fn read_line<'buffer>(&'buffer mut self) -> BufferResult<'buffer> {
        let reader = self.reader.as_mut().expect("File must be .open()'ed before reading");
        let line_num = self.line_num;
        let len = reader.read_line(&mut self.buffer)?;
        self.line_num += 1;

        debug!("read_chunk() ->  (line_num: {}, bytes_read: {}, buffer: [{:?}])", line_num, len, &self.buffer);
        
        Ok(ParsedLine {line_num, bytes_read: len, buffer: &self.buffer })
    }
}