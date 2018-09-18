use FILE_SYSTEM;
use fat32::traits::{
    FileSystem as FileSystemTrait,
    Dir as DirTrait,
    Entry as EntryTrait,
    Metadata as MetadataTrait,
    Timestamp as TimestampTrait
};
use fat32::vfat::{Dir, File};
use std::path::{Path, PathBuf};
use std::io;
use std::io::Read;
use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Contains all of the state related to the current shell. Until syscalls are
/// implemented, this is just the current path
struct ShellState {
    pub path: PathBuf
}

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

fn write_bool(b: bool, c: char) {
    if b { kprint!("{}", c) } else { kprint!("-") }
}

fn write_timestamp<T: TimestampTrait>(ts: T) {
    kprint!("{:02}/{:02}/{} {:02}:{:02}:{:02} ",
           ts.month(), ts.day(), ts.year(), ts.hour(), ts.minute(), ts.second());
}

fn print_io_error(kind: io::ErrorKind) {
    match kind {
        io::ErrorKind::Other => { kprintln!("error: not a directory"); },
        io::ErrorKind::NotFound => { kprintln!("error: not found"); },
        io::ErrorKind::InvalidInput => { kprintln!("error: invalid utf8"); },
        _ => { kprintln!("unknown error"); }
    };
}

/// A wrapper around FILE_SYSTEM.open() that prints errors to the shell if any
fn open_dir<P: AsRef<Path>>(path: P) -> Result<Dir, ()> {
    match FILE_SYSTEM.open_dir(path) {
        Err(err) => {
            print_io_error(err.kind());
            Err(())
        },
        Ok(dir) => Ok(dir)
    }
}

fn open_file<P: AsRef<Path>>(path: P) -> Result<File, ()> {
    match FILE_SYSTEM.open_file(path) {
        Err(err) => {
            print_io_error(err.kind());
            Err(())
        },
        Ok(file) => Ok(file)
    }
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }

    /// processes this command by printing output and updating the shell state
    fn process(&self, state: &mut ShellState) {
        match self.path() {
            "echo" => {
                let mut iter = self.args.iter();
                iter.next(); // skip over path
                for arg in iter {
                    kprint!("{} ", arg);
                }
                kprintln!("");
            },
            "ls" => {
                let mut path = state.path.clone();
                let list_all = self.args.len() > 1 && self.args[1] == "-a";

                if self.args.len() == 2 && !self.args[1].starts_with("-") {
                    path.push(self.args[1]);
                } else if self.args.len() > 2 {
                    path.push(self.args[2]);
                };

                if let Ok(dir) = open_dir(path) {
                    for entry in dir.entries().expect("ls") {
                        if list_all || !entry.metadata().hidden() {
                            write_bool(entry.is_dir(), 'd');
                            write_bool(entry.is_file(), 'f');
                            write_bool(entry.metadata().read_only(), 'r');
                            write_bool(entry.metadata().hidden(), 'h');
                            kprint!("\t");

                            write_timestamp(entry.metadata().modified());
                            kprint!("\t");

                            kprint!("{:>8}", entry.metadata().size());
                            kprint!("\t");

                            kprintln!("{}", entry.name());
                        }
                    }
                }
            },
            "pwd" => {
                kprintln!("{:?}", state.path);
            },
            "cd" => {
                if self.args.len() < 2 {
                    kprintln!("expected argument");
                    return;
                }
                match self.args[1] {
                    "." => { },
                    ".." => { state.path.pop(); },
                    s => {
                        let mut dest = state.path.clone();
                        dest.push(s);
                        if let Ok(_) = open_dir(dest) {
                            state.path.push(s);
                        }
                    }
                };
            },
            "cat" => {
                let mut iter = self.args.iter();
                iter.next();
                for file in iter {
                    let mut path = state.path.clone();
                    path.push(file);
                    if let Ok(mut file) = open_file(path) {
                        let mut contents = String::new();
                        if file.read_to_string(&mut contents).is_err() {
                            kprintln!("error printing file");
                        } else {
                            kprintln!("{}", contents);
                        }
                    }
                }

            }
            _ => { kprintln!("unknown command: {}", self.path()); }
        }
    }
}
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str, debug: bool) {
    let mut state = ShellState { path: PathBuf::from("/") };
    let mut raw_buffer = [0u8; 512];
    let mut buffer = StackVec::new(&mut raw_buffer);
    let parsed_cmd: [&str; 64] = [""; 64];

    loop {
        kprint!("{}", prefix);

        // read until a full command (+ newline) has been written
        loop {
            let byte = CONSOLE.lock().read_byte();
            if byte == b'\n' || byte == b'\r' {
                break;
            }
            // don't automatically process the instruction when the max
            // length is reached. instead, wait until newline
            if buffer.is_full() {
                continue;
            }
            if byte == 8 || byte == 127 { // backspace
                if !buffer.is_empty() {
                    kprint!("{} {}", byte as char, byte as char);
                    buffer.pop();
                }
            } else {
                kprint!("{}", byte as char);
                buffer.push(byte).expect("buffer is full!");
            }
        }

        kprintln!("");
        if let Ok(s) = ::core::str::from_utf8(&buffer.as_slice()) {
            match Command::parse(s, &mut {parsed_cmd}) {
                Ok(ref cmd) if debug && cmd.path() == "exit" => { return; }
                Ok(cmd) => { cmd.process(&mut state); },
                Err(Error::TooManyArgs) => { kprintln!("error: too many arguments"); },
                Err(Error::Empty) => {}
            };
        } else {
            kprint!("{}", 7); // sound bell for unrecognized character
        }

        buffer.truncate(0);
    }
}
