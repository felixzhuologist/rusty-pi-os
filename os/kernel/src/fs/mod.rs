pub mod sd;

use std::io;
use std::path::Path;

use fat32::vfat::{self, Shared, VFat};
pub use fat32::traits::FileSystem as FileSystemTrait;

use mutex::Mutex;
use self::sd::Sd;

pub struct FileSystem(Mutex<Option<Shared<VFat>>>);

impl FileSystem {
    /// Returns an uninitialized `FileSystem`.
    ///
    /// The file system must be initialized by calling `initialize()` before the
    /// first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {
        FileSystem(Mutex::new(None))
    }

    /// Initializes the file system.
    ///
    /// # Panics
    ///
    /// Panics if the underlying disk or file sytem failed to initialize.
    pub fn initialize(&self) {
        *self.0.lock() = Some(VFat::from(Sd::new().unwrap()).unwrap());
    }
}

impl FileSystemTrait for FileSystem {
    type File = vfat::File;
    type Dir = vfat::Dir;
    type Entry = vfat::Entry;

    fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<Self::Entry> {
        FileSystemTrait::open(&self.0.lock().as_ref().expect("fs uninitialized"), path)
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
    where
        P: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
