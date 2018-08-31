extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;
use std::fs::File;
use std::io;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn send_raw<T, U>(in_buf: &mut T, out_buf: &mut U) -> Result<(u64), io::Error> 
    where T: io::Read, U: io::Read + io::Write {
    io::copy(in_buf, out_buf)
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn send_xmodem<T, U>(in_buf: &mut T, out_buf: &mut U) -> Result<(u64), io::Error> 
    where T: io::Read, U: io::Read + io::Write {
    match Xmodem::transmit_with_progress(in_buf, out_buf, progress_fn) {
        Ok(n) => Ok(n as u64),
        Err(e) => Err(e)
    }
}

fn main() {
    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    serial.set_timeout(Duration::new(opt.timeout, 0)).expect("invalid timeout");
    let mut settings = serial.read_settings().expect("could not get settings");
    settings.set_baud_rate(opt.baud_rate).expect("invalid baud rate");
    settings.set_stop_bits(opt.stop_bits);
    settings.set_char_size(opt.char_width);
    settings.set_flow_control(opt.flow_control);
    serial.write_settings(&settings).expect("invalid settings");

    match opt.input {
        Some(path) => {
            let mut f = File::open(path).expect("could not open file");
            let copier = if opt.raw { send_raw } else { send_xmodem };
            copier(&mut f, &mut serial)
        },
        None => {
            let copier = if opt.raw { send_raw } else { send_xmodem };
            copier(&mut io::stdin(), &mut serial)
        }
    }.expect("send data failed");

    // FIXME: Implement the `ttywrite` utility.
}
