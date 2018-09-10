use atags::raw;

pub use atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => Some(core),
            _ => None
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => Some(mem),
            _ => None
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => Some(cmd),
            _ => None
        }
    }
}

impl<'a> From<&'a raw::Atag> for Atag {
    fn from(atag: &raw::Atag) -> Atag {
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => Atag::from(core),
                (raw::Atag::MEM, &raw::Kind { mem }) => Atag::from(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => Atag::from(cmd),
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id)
            }
        }
    }
}

impl From<raw::Core> for Atag {
    fn from(core: raw::Core) -> Atag {
        Atag::Core(core)
    }
}

impl From<raw::Mem> for Atag {
    fn from(mem: raw::Mem) -> Atag {
        Atag::Mem(mem)
    }
}

impl<'a> From<&'a raw::Cmd> for Atag {
    fn from(cmd: &raw::Cmd) -> Atag {
        let mut strlen = 0usize;
        let mut curr = &cmd.cmd as *const u8;
        unsafe {
            while (*curr) != 0 {
                strlen += 1;
                curr = curr.add(1);
            }
            Atag::Cmd(::core::str::from_utf8_unchecked(
                ::core::slice::from_raw_parts(
                    &cmd.cmd as *const u8, strlen)))
        }
    }
}
