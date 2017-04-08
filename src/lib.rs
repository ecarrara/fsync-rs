//! Syncronize files and directories.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate tempdir;

mod errors;
mod utils;

use errors::Result;
use std::fs::{File, copy};
use std::io::Read;
use std::path::Path;


/// Copy `source` to `target` only if files are different.
pub fn sync_file<P: AsRef<Path>>(source: P, target: P) -> Result<u64> {
    match is_equals(&source, &target) {
        Ok(false) => {
            let copied_bytes = copy(&source, &target)?;
            utils::copy_file_times(source, target)?;
            Ok(copied_bytes)
        }
        Ok(true) => Ok(0),
        Err(error) => Err(error)
    }
}

/// Return true if two files are equals.
///
/// Files are considered equals if have same len, same modification time
/// and same contents.
pub fn is_equals<P: AsRef<Path>>(source: P, target: P) -> Result<bool> {
    let source = source.as_ref();
    let target = target.as_ref();

    let source_metadata = source.metadata()?;
    let target_metadata = target.metadata()?;

    if source_metadata.len() != target_metadata.len() ||
       source_metadata.modified()? != target_metadata.modified()?                                                                                                                                     {
        Ok(false)
    } else  {
        have_same_contents(source, target)
    }
}

/// Verify if two files have the same contents.
pub fn have_same_contents<P: AsRef<Path>>(source: P, target: P) -> Result<bool> {
    let source = source.as_ref();
    let target = target.as_ref();

    let mut source_file = File::open(source)?;
    let mut target_file = File::open(target)?;

    const BUFFER_SIZE : usize = 512;

    let mut source_buffer = [0u8;BUFFER_SIZE];
    let mut target_buffer = [0u8;BUFFER_SIZE];

    loop {
        let source_read = source_file.read(&mut source_buffer)?;
        let target_read = target_file.read(&mut target_buffer)?;

        if source_read == 0 && target_read == 0 {
            break;
        }

        if source_read != target_read {
            return Ok(false)
        }

        if &source_buffer[..] != &target_buffer[..] {
            return Ok(false)
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::time::SystemTime;
    use std::time::{Duration, UNIX_EPOCH};
    use utils;
    use errors::Result;
    use super::is_equals;

    fn create_test_file<P: AsRef<Path>>(path: P,
                                        buf: &[u8],
                                        time: &SystemTime) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(buf)?;
        utils::set_file_times(&file, &time, &time)
    }

    #[test]
    fn equals_files_are_equals() {
        let test_time = UNIX_EPOCH + Duration::from_secs(1509062400);
        create_test_file("test_files/a.txt", b"avocado", &test_time).unwrap();
        create_test_file("test_files/b.txt", b"avocado", &test_time).unwrap();

        assert!(is_equals("./test_files/a.txt", "./test_files/b.txt").unwrap());
    }

    #[test]
    fn files_with_same_contents_but_different_mtime_not_are_equals() {
        let test_time_a = UNIX_EPOCH + Duration::from_secs(1509062400);
        let test_time_b = UNIX_EPOCH + Duration::from_secs(1509068400);
        create_test_file("test_files/c.txt", b"avocado", &test_time_a).unwrap();
        create_test_file("test_files/d.txt", b"avocado", &test_time_b).unwrap();

        assert!(!is_equals("./test_files/c.txt", "./test_files/d.txt").unwrap());
    }

    #[test]
    fn files_with_different_contents_and_same_mtime_not_are_equals() {
        let test_time = UNIX_EPOCH + Duration::from_secs(1509062400);
        create_test_file("test_files/e.txt", b"avocado", &test_time).unwrap();
        create_test_file("test_files/f.txt", b"banana", &test_time).unwrap();

        assert!(!is_equals("./test_files/e.txt", "./test_files/f.txt").unwrap());
    }
}
