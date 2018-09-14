use std::fmt;
use std::mem;

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    pub jmp: [u8; 3],
    pub oem_id: u64,
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub num_reserved: u16,
    pub num_fat: u8,
    pub max_dirs: u16,
    /// if this is 0, use value in total_sectors_hi
    pub total_sectors_lo: u16,
    pub fat_id: u8,
    /// 0 for FAT32, use sectors_per_fat32 instead
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16,
    pub heads: u16,
    pub hidden_sectors: u32,
    pub total_sectors_hi: u32,
    pub sectors_per_fat32: u32,
    pub flags: u16,
    pub fat_version: u16,
    /// cluster number of the root directory, often 2
    pub root: u32,
    /// sector number of the FSInfo structure
    pub fsinfo: u16,
    pub backup_boot: u16,
    pub __reserved: [u8; 12],
    pub drive_number: u8,
    pub windows_flags: u8,
    pub signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub system_id: u64,
    pub boot_code: [u8; 420],
    pub bootable_signature: u16
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(
        mut device: T,
        sector: u64
    ) -> Result<BiosParameterBlock, Error> {
        let mut raw_bios_block = [0u8; 512];
        if let Err(io_err) = device.read_sector(sector, &mut raw_bios_block) {
            return Err(Error::Io(io_err));
        }

        let block: BiosParameterBlock = unsafe { mem::transmute(raw_bios_block) };
        if block.bootable_signature != 0xAA55 {
            return Err(Error::BadSignature);
        }
        Ok(block)
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock")
            .field("oem_id", &self.oem_id)
            .field("bytes_per_sector", &self.bytes_per_sector)
            .field("num_reserved", &self.num_reserved)
            .field("sectors_per_fat32", &self.sectors_per_fat32)
            .field("sectors_per_cluster", &self.sectors_per_cluster)
            .field("num_fats", &self.num_fat)
            .field("root", &self.root)
            .finish()
    }
}
