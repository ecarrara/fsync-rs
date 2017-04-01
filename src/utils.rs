use errors::Result;
use std::fs::File;
use std::io::Error as IOError;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};


#[cfg(not(target_os="macos"))]
fn futime(file: &File, atime: &SystemTime, mtime: &SystemTime) -> Result<()> {
    use libc::{futimens, timespec, c_long, time_t};

    let atime_since_epoch = atime.duration_since(UNIX_EPOCH)?;
    let mtime_since_epoch = mtime.duration_since(UNIX_EPOCH)?;

    let times = [timespec {
                     tv_sec: atime_since_epoch.as_secs() as time_t,
                     tv_nsec: atime_since_epoch.subsec_nanos() as c_long,
                 },
                 timespec {
                     tv_sec: mtime_since_epoch.as_secs() as time_t,
                     tv_nsec: mtime_since_epoch.subsec_nanos() as c_long,
                 }];

    if unsafe { futimens(file.as_raw_fd(), times.as_ptr()) } == 0 {
        Ok(())
    } else {
        bail!(IOError::last_os_error())
    }
}

#[cfg(target_os="macos")]
fn futime(file: &File, atime: &SystemTime, mtime: &SystemTime) -> Result<()> {
    use libc::{futimes, timeval, suseconds_t, time_t};

    let atime_since_epoch = atime.duration_since(UNIX_EPOCH)?;
    let mtime_since_epoch = mtime.duration_since(UNIX_EPOCH)?;

    let times = [timeval {
                     tv_sec: atime_since_epoch.as_secs() as time_t,
                     tv_usec: atime_since_epoch.subsec_nanos() as suseconds_t,
                 },
                 timeval {
                     tv_sec: mtime_since_epoch.as_secs() as time_t,
                     tv_usec: mtime_since_epoch.subsec_nanos() as suseconds_t,
                 }];

    if unsafe { futimes(file.as_raw_fd(), times.as_ptr()) } == 0 {
        Ok(())
    } else {
        bail!(IOError::last_os_error())
    }
}


/// Set file acessed and modified time.
pub fn set_file_times<P: AsRef<Path>>(path: P,
                                      atime: &SystemTime,
                                      mtime: &SystemTime)
                                      -> Result<()> {
    let file = File::open(path)?;
    futime(&file, &atime, &mtime)
}


#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};
    use std::fs::copy;
    use super::set_file_times;
    use tempdir::TempDir;

    #[test]
    fn test_set_file_times() {
        // Oct, 27 -> a special date! :D
        let atime = UNIX_EPOCH + Duration::from_secs(1509062400);
        let mtime = UNIX_EPOCH + Duration::from_secs(1509105600);

        let temp_dir = TempDir::new("test_set_file_times").unwrap();
        let test_filepath = temp_dir.path().join("a.txt");
        copy("./test_files/a.txt", &test_filepath).unwrap();

        set_file_times(&test_filepath, &atime, &mtime).unwrap();
    }
}
