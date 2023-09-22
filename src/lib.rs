use std::fmt::Display;

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

//Clipboard formats
const CF_BITMAP: u16 = 2;
const CF_METAFILEPICT: u16 = 3;
const CF_DIB: u16 = 8;
const CF_ENHMETAFILE: u16 = 0xE;

mod ole_file {
    use crate::FileTime;

    use crate::CLSID;
    use std::fs::File;
    use std::io::{self, Read};
    use std::rc::Rc;

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

    pub fn is_ole_file(reader: &mut File) -> Result<bool, io::Error> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        return Ok(buf == MAGIC);
    }
}

struct CLSID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: u64,
}

impl Display for CLSID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

struct DevModeA {
    device_name: [u8; 32], // This field is a 32-element array of 8-bit ANSI characters.
    form_name: [u8; 32],   // This field is a 32-element array of 8-bit ANSI characters.
    spec_version: u16, // The version of initialization data specification on which the DEVMODE structure is based.
    driver_version: u16, //  For printers, an optional, implementation-defined version of the printer driver.
    size: u16, // The size, in bytes, of the DEVMODE structure. The size MUST NOT include the length of any private,
    // printer driver–specific data that might follow the DEVMODE structure's public fields.
    driver_extra: u16, // The size, in bytes, of the private printer driver data that follows this structure.
    fields: u32, // A bitfield that specifies the fields of the DEVMODE structure that have been
    // initialized. If a bit is set, the corresponding field MUST be initialized and MUST be processed on
    // receipt. If a bit is not set, the value of the corresponding field SHOULD be set to zero and MUST
    // be ignored on receipt.
    orientation: u16, // For printers, the orientation for output. If the DM_ORIENTATION bit is set
    // in dmFields, this value MUST be specified.
    paper_size: u16, // For printers, the size of the output media. If the DM_PAPERSIZE bit is set
    // in dmFields, this value MUST be specified. The value of this field SHOULD be one of the following,
    // or it MAY be a device-specific value that is greater than or equal to 0x0100.
    paper_length: u16, // If the DM_PAPERLENGTH bit is set in the dmFields field, the value of
    // this field specifies the length of the paper, in tenths of a millimeter, to use in the printer for which
    // the job is destined.
    paper_width: u16, // : If the DM_PAPERWIDTH bit is set in the dmFields field, the value of this
    // field specifies the width of the paper, in tenths of a millimeter, to use in the printer for which the
    // job is destined.
    scale: u16,
    copies: u16,
    default_source: u16,
    print_quality: u16,
    color: u16,
    duplex: u16,
    y_res: u16,
    tt_option: u16,
    collate: u16,
    reserved0: u16,
    reserved1: u32,
    reserved2: u32,
    reserved3: u32,
    nup: u32,
    reserved4: u32,
    icm_method: u32,
    icm_intent: u32,
    media_type: u32,
    dither_type: u32,
    reserved5: u32,
    reserved6: u32,
    reserved7: u32,
    reserved8: u32,
}

#[cfg(test)]
mod tests {}
