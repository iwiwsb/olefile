use crate::data_structures::FileTime;

use crate::data_structures::CLSID;
use std::fs::File;
use std::io::{self, Read};

const MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

const MAXREGSECT: u32 = 0xFFFFFFFA; // Maximum regular sector number
const DIFSECT: u32 = 0xFFFFFFFC; // Specifies a DIFAT sector in the FAT
const FATSECT: u32 = 0xFFFFFFFD; // Specifies a FAT sector in the FAT
const ENDOFCHAIN: u32 = 0xFFFFFFFE; // End of a linked chain of sectors
const FREESECT: u32 = 0xFFFFFFFF; // Specifies an unallocated sector in the FAT, Mini FAT, or DIFAT

struct OleFileHeader {
    header_signature: [u8; 8], // Identification signature for the compound file structure, and MUST be set to the value 0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1
    header_clsid: [u8; 16],    // Reserved and unused class ID that MUST be set to all zeroes
    minor_version: u16, // Version number for nonbreaking changes. This field SHOULD be set to 0x003E if the major version field is either 0x0003 or 0x0004
    major_version: u16, // Version number for breaking changes. This field MUST be set to either 0x0003 (version 3) or 0x0004 (version 4)
    byte_order: u16, //  This field MUST be set to 0xFFFE. This field is a byte order mark for all integer fields, specifying little-endian byte order.
    sector_shift: u16, // This field MUST be set to 0x0009, or 0x000c, depending on the Major Version field. This field specifies the sector size of the compound file
    // as a power of 2. If Major Version is 3, the Sector Shift MUST be 0x0009, specifying a sector size of 512 bytes. If Major Version is 4, the Sector Shift MUST be 0x000C,
    // specifying a sector size of 4096 bytes.
    mini_cector_shift: u16, // This field MUST be set to 0x0006. This field specifies the sector size of the Mini Stream as a power of 2.
    // The sector size of the Mini Stream MUST be 64 bytes.
    reserved: [u8; 6],                // This field MUST be set to all zeroes
    num_of_dir_sectiors: Option<u32>, // This integer field contains the count of the number of directory sectors in the compound file. If Major Version is 3,
    // the Number of Directory Sectors MUST be zero. This field is not supported for version 3 compound files.
    num_of_fat_sectors: u32, // This integer field contains the count of the number of FAT sectors in the compound file.
    first_dir_sector_location: u32, // This integer field contains the starting sector number for the directory stream.
    transaction_sig_num: u32, // This integer field MAY contain a sequence number that is incremented every time the compound file is saved by an implementation that supports
    // file transactions. This is the field that MUST be set to all zeroes if file transactions are not implemented
    mini_stream_cutoff_size: u32, // This integer field MUST be set to 0x00001000. This field specifies the maximum size of a user-defined data stream that is allocated
    // from the mini FAT and mini stream, and that cutoff is 4,096 bytes. Any user-defined data stream that is greater than or equal to this cutoff size must be allocated
    // as normal sectors from the FAT.
    first_mini_fat_sector_location: u32, // This integer field contains the starting sector number for the mini FAT
    num_of_mini_fat_sectors: u32, // This integer field contains the count of the number of mini FAT sectors in the compound file.
    first_difat_sector_location: u32, // This integer field contains the starting sector number for the DIFAT
    num_of_difat_sectors: u32, // This integer field contains the count of the number of DIFAT sectors in the compound file.
    difat: [u32; 109], // This array of 32-bit integer fields contains the first 109 FAT sector locations of the compound file.
}

struct OleDirectoryEntry {
    name: [u16; 64],
    namelength: u16,
    object_type: u8,
    color_flag: bool,
    left_sibling_id: u32,
    right_sibling_id: u32,
    child_id: u32,
    clsid: CLSID,
    state_bits: u32,
    creation_time: FileTime,
    modified_time: FileTime,
    total_stream_size_low: u32,
    total_stream_size_high: u32,
}

pub fn is_ole_file(file: &mut File) -> Result<bool, io::Error> {
    is_ole(file)
}

fn is_ole<R: Read>(reader: &mut R) -> Result<bool, io::Error> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    return Ok(buf == MAGIC);
}

fn parse_header(file: &mut File) -> Result<OleFileHeader, io::Error> {
    let mut header_signature = [0u8; 8];
    let mut header_clsid = [0u8; 16];
    todo!()
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
