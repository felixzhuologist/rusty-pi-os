// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl Duration {
    fn to_ms(&self) -> u64 {
        match *self {
            Duration::MilliSeconds(ms) => ms,
            Duration::Seconds(s) => (s as u64) * 1000,
            Duration::Minutes(m) => (m as u64) * 60 * 1000
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        self.to_ms() == other.to_ms()
    }
}

impl Eq for Duration {}

fn main() {
    use Duration::*;
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
