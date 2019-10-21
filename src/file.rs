use std::path::Path;
use log::{debug, info, error};
use failure::{format_err, Error};
use std::io::Read;
use std::fs;

pub type FileResult<T> = Result<T, Error>;

pub const FILE_SIZE_LIMIT: u64 = 100_000; // Prevent reading large files into a String, by default

pub fn read_complete_file<T: AsRef<Path>>(path: T) -> FileResult<String> {
    debug!("Reading in complete file: [{:?}]", path.as_ref().to_str().ok_or(format_err!("Failed to convert path to str"))?);
    check_file_exists(&path)?;
    let mut file_contents = String::new();
    open(&path)?.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn check_file_exists<T: AsRef<Path>>(path: T) -> FileResult<()>  {
    let path = path.as_ref();
    let path_str = path.to_str().ok_or(format_err!("Failed to convert path to str"))?;
    if path.exists() {
        let file_size = fs::metadata(path)?.len();
        if file_size > FILE_SIZE_LIMIT {
            error!("File is too big: {} [{} Bytes] [{} Bytes Limit]", path_str, file_size, FILE_SIZE_LIMIT);
            return Err(format_err!("File is too big: {} [{} Bytes] [{} Bytes Limit]", path_str, file_size, FILE_SIZE_LIMIT));
        }
        info!("Found File: {} [{} Bytes]", path_str, file_size);
    } else {
        error!("Missing  File: {}", path_str);
        return Err(format_err!("Missing File: {}", path_str));
    }
    Ok(())
}

fn open<T: AsRef<Path>>(path: T) -> FileResult<fs::File> {
    let file = fs::OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(path.as_ref())?;
    info!("Opened file: {}", path.as_ref().to_str().ok_or(format_err!("Failed to convert path to str"))?);
    Ok(file)
}