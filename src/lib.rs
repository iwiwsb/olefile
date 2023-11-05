use crate::data_structures::FileTime;
use crate::data_structures::{CLSID, CLSID_NULL};

use log::{info, warn};
use std::fs::File;
use std::io::{self, Read};

// property types
const VT_EMPTY: u32 = 0;
const VT_NULL: u32 = 1;
const VT_I2: u32 = 2;
const VT_I4: u32 = 3;
const VT_R4: u32 = 4;
const VT_R8: u32 = 5;
const VT_CY: u32 = 6;
const VT_DATE: u32 = 7;
const VT_BSTR: u32 = 8;
const VT_DISPATCH: u32 = 9;
const VT_ERROR: u32 = 10;
const VT_BOOL: u32 = 11;
const VT_VARIANT: u32 = 12;
const VT_UNKNOWN: u32 = 13;
const VT_DECIMAL: u32 = 14;
const VT_I1: u32 = 16;
const VT_UI1: u32 = 17;
const VT_UI2: u32 = 18;
const VT_UI4: u32 = 19;
const VT_I8: u32 = 20;
const VT_UI8: u32 = 21;
const VT_INT: u32 = 22;
const VT_UINT: u32 = 23;
const VT_VOID: u32 = 24;
const VT_HRESULT: u32 = 25;
const VT_PTR: u32 = 26;
const VT_SAFEARRAY: u32 = 27;
const VT_CARRAY: u32 = 28;
const VT_USERDEFINED: u32 = 29;
const VT_LPSTR: u32 = 30;
const VT_LPWSTR: u32 = 31;
const VT_FILETIME: u32 = 64;
const VT_BLOB: u32 = 65;
const VT_STREAM: u32 = 66;
const VT_STORAGE: u32 = 67;
const VT_STREAMED_OBJECT: u32 = 68;
const VT_STORED_OBJECT: u32 = 69;
const VT_BLOB_OBJECT: u32 = 70;
const VT_CF: u32 = 71;
const VT_CLSID: u32 = 72;
const VT_VECTOR: u32 = 0x1000;

//object types
const STGTY_EMPTY: u8 = 0;
const STGTY_STORAGE: u8 = 1;
const STGTY_STREAM: u8 = 2;
const STGTY_LOCKBYTES: u8 = 3;
const STGTY_PROPERTY: u8 = 4;
const STGTY_ROOT: u8 = 5;

pub mod data_structures;

const MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const MAXREGSECT: u32 = 0xFFFFFFFA; // Maximum regular sector number
const DIFSECT: u32 = 0xFFFFFFFC; // Specifies a DIFAT sector in the FAT
const FATSECT: u32 = 0xFFFFFFFD; // Specifies a FAT sector in the FAT
const ENDOFCHAIN: u32 = 0xFFFFFFFE; // End of a linked chain of sectors
const FREESECT: u32 = 0xFFFFFFFF; // Specifies an unallocated sector in the FAT, Mini FAT, or DIFAT

pub struct OleFileHeader {
    /// Identification signature for the compound file structure;
    /// MUST be set to the value 0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1
    pub header_signature: [u8; 8],
    /// Reserved and unused class ID that MUST always be [`CLSID_NULL`](crate::data_structures::CLSID_NULL)
    pub header_clsid: CLSID,
    /// Version number for nonbreaking changes;
    /// SHOULD be set to 0x003E if the major version field is either 0x0003 or 0x0004
    pub minor_version: u16,
    /// Version number for breaking changes;
    /// MUST be set to either 0x0003 (version 3) or 0x0004 (version 4)
    pub major_version: u16,
    /// MUST be set to 0xFFFE;
    /// a byte order mark for all integer fields, specifying little-endian byte order
    pub byte_order: u16,
    /// MUST be set to 0x0009, or 0x000C, depending on the Major Version field;
    /// the sector size of the compound file as a power of 2;
    /// if Major Version is 3, the Sector Shift MUST be 0x0009, specifying a sector size of 512 bytes;
    /// if Major Version is 4, the Sector Shift MUST be 0x000C, specifying a sector size of 4096 bytes
    pub sector_shift: u16,
    /// MUST be set to 0x0006;
    /// the sector size of the Mini Stream as a power of 2;
    /// the sector size of the Mini Stream MUST be 64 bytes
    pub mini_sector_shift: u16,
    /// MUST be set to all zeroes
    pub reserved: [u8; 6],
    /// The count of the number of directory sectors in the compound file;
    /// if Major Version is 3, the Number of Directory Sectors MUST be zero;
    /// not supported for version 3 compound files
    pub num_of_dir_sectors: u32,
    /// The count of the number of FAT sectors in the compound file
    pub num_of_fat_sectors: u32,
    /// The starting sector number for the directory stream
    pub first_dir_sector_location: u32,
    /// MAY contain a sequence number that is incremented every time the compound file is saved by an implementation that supports file transactions
    /// MUST be set to all zeroes if file transactions are not implemented
    pub transaction_sig_num: u32,
    /// MUST be set to 0x00001000;
    /// Maximum size of a user-defined data stream that is allocated from the mini FAT and mini stream, and that cutoff is 4,096 bytes;
    /// Any user-defined data stream that is greater than or equal to this cutoff size must be allocated as normal sectors from the FAT
    pub mini_stream_cutoff_size: u32,
    /// Starting sector number for the mini FAT
    pub first_mini_fat_sector_location: u32,
    /// The count of the number of mini FAT sectors in the compound file
    pub num_of_mini_fat_sectors: u32,
    /// The starting sector number for the DIFAT
    pub first_difat_sector_location: u32,
    /// The count of the number of DIFAT sectors in the compound file
    pub num_of_difat_sectors: u32,
    /// The first 109 FAT sector locations of the compound file
    pub difat: [u32; 109],
}

struct FATSector;

struct FATArray {
    array_size: usize,
    entries: Vec<FATSector>,
}

struct MiniFATSector {}

struct MiniFatArray {
    array_size: usize,
    entries: Vec<MiniFATSector>,
}

struct DIFATArray {}

impl Validation for OleFileHeader {
    fn validate(&self) {
        if self.header_signature != MAGIC {
            warn!("Wrong signature!");
        }

        if self.header_clsid != CLSID_NULL {
            warn!("Header CLSID is non-zero!");
        }

        if self.major_version != 3 || self.major_version != 4 {
            warn!("Wrong major version");
            if self.minor_version != 0x3E {
                info!("Minor version is not 0x3E");
            }
        }
    }
}

#[derive(Debug)]
struct OleDirectoryEntry {
    pub name: String,
    pub namelength: u16,
    pub object_type: u8,
    pub color_flag: bool,
    pub left_sibling_id: u32,
    pub right_sibling_id: u32,
    pub child_id: u32,
    pub clsid: CLSID,
    pub state_bits: u32,
    pub creation_time: FileTime,
    pub modified_time: FileTime,
    pub starting_sector_offset: u32,
    pub stream_size_high: u64,
}

pub fn is_ole_file(file: &mut File) -> Result<bool, io::Error> {
    is_ole(file)
}

fn is_ole<R: Read>(reader: &mut R) -> Result<bool, io::Error> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    return Ok(buf == MAGIC);
}

pub trait Validation {
    fn validate(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ole_file() {
        let is_ole =
            is_ole_file(&mut File::open(r#".\test\example_olefile.bin"#).unwrap()).unwrap();
        assert_eq!(is_ole, true)
    }
}
