use std::io;
use pi::timer;
use fat32::traits::BlockDevice;

#[link(name="sd", kind="static")]
extern "C" {
    /// A global representing the last SD controller error that occured.
    static sd_err: i64;

    /// Initializes the SD card controller.
    ///
    /// Returns 0 if initialization is successful. If initialization fails,
    /// returns -1 if a timeout occured, or -2 if an error sending commands to
    /// the SD controller occured.
    fn sd_init() -> i32;

    /// Reads sector `n` (512 bytes) from the SD card and writes it to `buffer`.
    /// It is undefined behavior if `buffer` does not point to at least 512
    /// bytes of memory.
    ///
    /// On success, returns the number of bytes read: a positive number.
    ///
    /// On error, returns 0. The true error code is stored in the `sd_err`
    /// global. `sd_err` will be set to -1 if a timeout occured or -2 if an
    /// error sending commands to the SD controller occured. Other error codes
    /// are also possible but defined only as being less than zero.
    fn sd_readsector(n: i32, buffer: *mut u8) -> i32;
}

#[no_mangle]
pub fn wait_micros(us: u32) {
    timer::spin_sleep_us(us.into());
}

#[derive(Debug)]
pub enum Error {
    Timeout,
    CommandError,
    UnknownReturnCode
}

/// A handle to an SD card controller.
#[derive(Debug)]
pub struct Sd;

impl Sd {
    /// Initializes the SD card controller and returns a handle to it.
    pub fn new() -> Result<Sd, Error> {
        match unsafe { sd_init() } {
            0 => Ok(Sd),
            -1 => Err(Error::Timeout),
            -2 => Err(Error::CommandError),
            _ => Err(Error::UnknownReturnCode)
        }
    }
}

impl BlockDevice for Sd {
    /// Reads sector `n` from the SD card into `buf`. On success, the number of
    /// bytes read is returned.
    ///
    /// # Errors
    ///
    /// An I/O error of kind `InvalidInput` is returned if `buf.len() < 512` or
    /// `n > 2^31 - 1` (the maximum value for an `i32`).
    ///
    /// An error of kind `TimedOut` is returned if a timeout occurs while
    /// reading from the SD card.
    ///
    /// An error of kind `Other` is returned for all other errors.
    fn read_sector(&mut self, n: u64, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < 512 || n > ((1<<31) - 1) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bad read args"));
        }

        let ret_code = unsafe { sd_readsector(n as i32,  (buf as *mut [u8]) as *mut u8) };
        if ret_code == -1 {
            Err(io::Error::new(io::ErrorKind::TimedOut, "reading from sd"))
        } else if ret_code <= 0 {
            Err(io::Error::new(io::ErrorKind::Other, "unknown error"))
        } else {
            Ok(ret_code as usize)
        }
    }

    fn write_sector(&mut self, _n: u64, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!("SD card and file system are read only")
    }
}
