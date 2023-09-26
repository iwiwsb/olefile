#![forbid(unsafe_code)]
use crate::data_structures::FileTime;
use crate::data_structures::{CLSID, CLSID_NULL};
use crate::Validation;

use log::{info, warn};
use std::fs::File;
use std::io::{self, Cursor, Read, Seek, SeekFrom};

const MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const MAXREGSECT: u32 = 0xFFFFFFFA; // Maximum regular sector number
const DIFSECT: u32 = 0xFFFFFFFC; // Specifies a DIFAT sector in the FAT
const FATSECT: u32 = 0xFFFFFFFD; // Specifies a FAT sector in the FAT
const ENDOFCHAIN: u32 = 0xFFFFFFFE; // End of a linked chain of sectors
const FREESECT: u32 = 0xFFFFFFFF; // Specifies an unallocated sector in the FAT, Mini FAT, or DIFAT

struct OLeFileBuffer {
    buffer: Cursor<Box<[u8]>>,
}

impl OLeFileBuffer {
    pub fn read_ole_file_header(&self) -> OleFileHeader {
        todo!()
    }

    pub fn read_fat_sector(&self) -> FATArray {
        todo!()
    }

    pub fn read_mini_fat_sector(&self) -> MiniFATSector {
        todo!()
    }

    pub fn read_difat_array(&self) -> DIFATArray {
        todo!()
    }
}

trait ReadU8 {
    fn read_u8(&self) -> u8;
}

trait ReadU16LE {
    fn read_u16_le(&self) -> u16;
}

trait ReadU16BE {
    fn read_u16_be(&self) -> u16;
}

trait ReadU32LE {
    fn read_u32_le(&self) -> u32;
}

trait ReadU32BE {
    fn read_u32_be(&self) -> u32;
}

trait ReadBits {
    fn read_bits(&self, n: usize) -> Vec<bool>;
}

pub struct OleFileHeader {
    header_signature: [u8; 8],
    header_clsid: [u8; 16],
    minor_version: [u8; 2],
    major_version: [u8; 2],
    byte_order: [u8; 2],
    sector_shift: [u8; 2],
    mini_cector_shift: [u8; 2],
}

impl OleFileHeader {
    /// Identification signature for the compound file structure;
    /// MUST be set to the value 0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1
    pub fn header_signature(&self) -> [u8; 8] {
        todo!()
    }

    /// Reserved and unused class ID that MUST always be [`CLSID_NULL`](crate::data_structures::CLSID_NULL)
    pub fn header_clsid(&self) -> CLSID {
        todo!()
    }

    /// Version number for nonbreaking changes;
    /// SHOULD be set to 0x003E if the major version field is either 0x0003 or 0x0004
    pub fn minor_version(&self) -> u16 {
        todo!()
    }

    /// Version number for breaking changes;
    /// MUST be set to either 0x0003 (version 3) or 0x0004 (version 4)
    pub fn major_version(&self) -> u16 {
        todo!()
    }

    /// MUST be set to 0xFFFE;
    /// a byte order mark for all integer fields, specifying little-endian byte order
    pub fn byte_order(&self) -> u16 {
        todo!()
    }

    /// MUST be set to 0x0009, or 0x000C, depending on the Major Version field;
    /// the sector size of the compound file as a power of 2;
    /// if Major Version is 3, the Sector Shift MUST be 0x0009, specifying a sector size of 512 bytes;
    /// if Major Version is 4, the Sector Shift MUST be 0x000C, specifying a sector size of 4096 bytes
    pub fn sector_shift(&self) -> u16 {
        todo!()
    }

    /// MUST be set to 0x0006;
    /// the sector size of the Mini Stream as a power of 2;
    /// the sector size of the Mini Stream MUST be 64 bytes
    pub fn mini_cector_shift(&self) -> u16 {
        todo!()
    }

    /// MUST be set to all zeroes
    pub fn reserved(&self) -> [u8; 6] {
        todo!()
    }

    /// The count of the number of directory sectors in the compound file;
    /// if Major Version is 3, the Number of Directory Sectors MUST be zero;
    /// not supported for version 3 compound files
    pub fn num_of_dir_sectiors(&self) -> Option<u32> {
        todo!()
    }

    /// The count of the number of FAT sectors in the compound file
    pub fn num_of_fat_sectors(&self) -> u32 {
        todo!()
    }

    /// The starting sector number for the directory stream
    pub fn first_dir_sector_location(&self) -> u32 {
        todo!()
    }

    /// MAY contain a sequence number that is incremented every time the compound file is saved by an implementation that supports file transactions
    /// MUST be set to all zeroes if file transactions are not implemented
    pub fn transaction_sig_num(&self) -> u32 {
        todo!()
    }

    /// MUST be set to 0x00001000;
    /// Maximum size of a user-defined data stream that is allocated from the mini FAT and mini stream, and that cutoff is 4,096 bytes;
    /// Any user-defined data stream that is greater than or equal to this cutoff size must be allocated as normal sectors from the FAT
    pub fn mini_stream_cutoff_size(&self) -> u32 {
        todo!()
    }

    /// Starting sector number for the mini FAT
    pub fn first_mini_fat_sector_location(&self) -> u32 {
        todo!()
    }

    /// The count of the number of mini FAT sectors in the compound file
    pub fn num_of_mini_fat_sectors(&self) -> u32 {
        todo!()
    }

    /// The starting sector number for the DIFAT
    pub fn first_difat_sector_location(&self) -> u32 {
        todo!()
    }

    /// The count of the number of DIFAT sectors in the compound file
    pub fn num_of_difat_sectors(&self) -> u32 {
        todo!()
    }

    /// The first 109 FAT sector locations of the compound file
    pub fn difat(&self) -> [u32; 109] {
        todo!()
    }
}

struct FATSector;

struct FATArray {
    array_size: usize,
    entries: Box<[FATSector]>,
}

struct MiniFATSector {}

struct MiniFatArray {
    array_size: usize,
    entries: Box<[MiniFATSector]>,
}

struct DIFATArray {}

impl Validation for OleFileHeader {
    fn validate(&self) {
        if self.header_signature() != MAGIC {
            warn!("Wrong signature!");
        }

        if self.header_clsid() != CLSID_NULL {
            warn!("Header CLSID is non-zero!");
        }

        if self.major_version() != 3 || self.major_version() != 4 {
            warn!("Wrong major version");
            if self.minor_version() != 0x3E {
                info!("Minor version is not 0x3E");
            }
        }
    }
}

struct OleDirectoryEntry {
    name: [u8; 128],
    namelength: [u8; 2],
    object_type: [u8; 1],
    color_flag: bool,
    left_sibling_id: [u8; 4],
    right_sibling_id: [u8; 4],
    child_id: [u8; 4],
    clsid: [u8; 16],
    state_bits: [u8; 4],
    creation_time: [u8; 8],
    modified_time: [u8; 8],
    total_stream_size_low: [u8; 4],
    total_stream_size_high: [u8; 4],
}

pub fn is_ole_file(file: &mut File) -> Result<bool, io::Error> {
    is_ole(file)
}

fn is_ole<R: Read>(reader: &mut R) -> Result<bool, io::Error> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    return Ok(buf == MAGIC);
}

#[cfg(test)]
mod tests {
    use crate::ole_file::is_ole_file;
    use std::fs::File;

    #[test]
    fn test_is_ole_file() {
        let is_ole =
            is_ole_file(&mut File::open(r#".\test\example_olefile.bin"#).unwrap()).unwrap();
        assert_eq!(is_ole, true)
    }
}
