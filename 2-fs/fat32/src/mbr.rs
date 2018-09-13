use std::{fmt, io};
use util;

use traits::BlockDevice;

const TABLE_OFFSET: usize = 446;
const ENTRY_SIZE: usize = 16;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CHS {
    // FIXME: Fill me in.
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct PartitionEntry {
    bootable: bool,
    partition_type: u8,
    /// offset, in sectors, from start of disk to start of partition
    relative_sector: u32,
    total_sectors: u32,
}

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    partitions: [PartitionEntry; 4],
}

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut raw_mbr = [0u8; 512];
        if let Err(io_err) = device.read_sector(0, &mut raw_mbr) {
            return Err(Error::Io(io_err));
        }
        
        if raw_mbr[510] != 0x55 || raw_mbr[511] != 0xAA {
            return Err(Error::BadSignature);
        }

        let mut partitions = [
            PartitionEntry::new(),
            PartitionEntry::new(),
            PartitionEntry::new(),
            PartitionEntry::new(),
        ];

        for i in 0..partitions.len() {
            let start = TABLE_OFFSET + i*ENTRY_SIZE;
            let raw_entry = &raw_mbr[start..start + ENTRY_SIZE];
            if let Err(_) = partitions[i].read_data(raw_entry) {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }

        Ok(MasterBootRecord { partitions })
    }

    /// Return the offset in physical sectors of the first FAT partition if one
    /// exists
    pub fn get_fat_partition_offset(&self) -> Option<u32> {
        for partition in self.partitions.iter() {
            if partition.partition_type == 0x0b || partition.partition_type == 0x0c {
                return Some(partition.relative_sector);
            }
        }
        None
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("MasterBootRecord::fmt()")
    }
}

impl PartitionEntry {
    pub fn new() -> PartitionEntry {
        PartitionEntry {
            bootable: false,
            partition_type: 0,
            relative_sector: 0,
            total_sectors: 0
        }
    }

    pub fn read_data(&mut self, buf: &[u8]) -> Result<(), ()> {
        println!("{}", buf[0]);
        self.bootable = match buf[0] {
            0 => false,
            0x80 => true,
            _ => return Err(())
        };
        self.partition_type = buf[4];
        self.relative_sector = util::from_le(&buf[8..12]);
        self.total_sectors = util::from_le(&buf[12..16]);
        Ok(())
    }
}
