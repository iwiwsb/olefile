use std::fmt::Display;

/// Bitmap16 Object structure
pub const CF_BITMAP: u16 = 2;
/// Windows metafile
pub const CF_METAFILEPICT: u16 = 3;
/// DeviceIndependentBitmap Object structure
pub const CF_DIB: u16 = 8;
/// Enhanced Metafile
pub const CF_ENHMETAFILE: u16 = 0xE;

pub const CLSID_NULL: CLSID = CLSID {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0u8; 8],
};

#[derive(PartialEq)]
/// A GUID, also known as a UUID, which is a 16-byte structure, intended to serve as a unique identifier for an object.
pub struct CLSID {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl CLSID {
    /// Constructs new CLSID
    pub fn new() -> Self {
        todo!()
    }
}

impl From<[u8; 16]> for CLSID {
    fn from(value: [u8; 16]) -> Self {
        let data1 = u32::from_le_bytes(value[0..4].try_into().unwrap());
        let data2 = u16::from_le_bytes(value[4..6].try_into().unwrap());
        let data3 = u16::from_le_bytes(value[6..8].try_into().unwrap());
        let data4 = value[8..16].try_into().unwrap();

        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

impl Display for CLSID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:X}-{:X}-{:X}-{:X}{:X}-{:X}{:X}{:X}{:X}{:X}{:X}",
            self.data1.swap_bytes(),
            self.data2.swap_bytes(),
            self.data3.swap_bytes(),
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        ))
    }
}

/// A 64-bit value that represents the number of 100-nanosecond intervals that have elapsed since January 1, 1601, Coordinated Universal Time (UTC)
pub struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

/// Initialization data for a printer
pub struct DevModeA {
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

pub struct DvTargetDevice {}

#[cfg(test)]
mod tests {
    use super::CLSID;

    #[test]
    fn test_clsid_print() {
        let clsid_string = String::from("DEADBEEF-BAAD-F00D-DEAD-BEEFDEADBEEF");
        let clsid = CLSID::from([
            0xDE, 0xAD, 0xBE, 0xEF, 0xBA, 0xAD, 0xF0, 0x0D, 0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD,
            0xBE, 0xEF,
        ]);
        assert_eq!(clsid.to_string(), clsid_string)
    }
}
