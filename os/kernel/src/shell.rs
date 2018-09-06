use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

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

    fn process(&self) {
        match self.path() {
            "echo" => {
                let mut iter = self.args.iter();
                iter.next(); // skip over path
                for arg in iter {
                    kprint!("{} ", arg);
                }
                kprintln!("");
            },
            _ => { kprintln!("unknown command: {}", self.path()); }
        }
    }
}
/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
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
                Ok(cmd) => { cmd.process(); },
                Err(Error::TooManyArgs) => { kprintln!("error: too many arguments"); },
                Err(Error::Empty) => {}
            };
        } else {
            kprint!("{}", 7); // sound bell for unrecognized character
        }

        buffer.truncate(0);
    }
}
