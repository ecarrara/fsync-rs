use errors::Result;
use libc::{futimens, timespec, c_long, time_t};
use std::fs::File;
use std::io::Error as IOError;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};


/// Set file acessed and modified time.
pub fn set_file_times<P: AsRef<Path>>(path: P,
                                      atime: &SystemTime,
                                      mtime: &SystemTime)
                                      -> Result<()> {
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

    let file = File::open(path)?;
    let ret = unsafe { futimens(file.as_raw_fd(), times.as_ptr()) };
    if ret == 0 {
        Ok(())
    } else {
        bail!(IOError::last_os_error())
    }
}


#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};
    use super::set_file_times;

    #[test]
    fn test_set_file_times() {
        // Oct, 27 -> a special date! :D
        let atime = UNIX_EPOCH + Duration::from_secs(1509062400);
        let mtime = UNIX_EPOCH + Duration::from_secs(1509105600);
        set_file_times("./test_files/a.txt", &atime, &mtime).unwrap();
    }
}