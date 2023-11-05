use chrono::{prelude::*, Duration};
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
///
#[derive(Debug)]
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

impl Default for CLSID {
    fn default() -> Self {
        CLSID_NULL
    }
}

impl From<[u8; 16]> for CLSID {
    fn from(value: [u8; 16]) -> Self {
        let data1 = u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
        let data2 = u16::from_le_bytes([value[4], value[5]]);
        let data3 = u16::from_le_bytes([value[6], value[7]]);
        let data4 = [
            value[8], value[9], value[10], value[11], value[12], value[13], value[14], value[15],
        ];

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
///
#[derive(Debug)]
pub struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

impl From<FileTime> for DateTime<Utc> {
    fn from(value: FileTime) -> Self {
        let start = Utc.with_ymd_and_hms(1601, 1, 1, 1, 0, 0).unwrap();
        let duration = Duration::nanoseconds((u64::from(value) * 100) as i64);
        start + duration
    }
}

impl From<FileTime> for u64 {
    fn from(value: FileTime) -> Self {
        let bytes = [
            value.high_date_time.to_le_bytes(),
            value.low_date_time.to_le_bytes(),
        ]
        .concat();
        u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

/// Initialization data for a printer
pub struct DevModeA {
    pub device_name: String,
    pub form_name: String,
    pub spec_version: u16,
    pub driver_version: u16,
    pub size: u16,
    pub driver_extra: u16,
    pub fields: u32,
    pub orientation: u16,
    pub paper_size: u16,
    pub paper_length: u16,
    pub paper_width: u16,
    pub scale: u16,
    pub copies: u16,
    pub default_source: u16,
    pub print_quality: u16,
    pub color: u16,
    pub duplex: u16,
    pub y_res: u16,
    pub tt_option: u16,
    pub collate: u16,
    pub reserved0: u16,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
    pub nup: u32,
    pub reserved4: u32,
    pub icm_method: u32,
    pub icm_intent: u32,
    pub media_type: u32,
    pub dither_type: u32,
    pub reserved5: u32,
    pub reserved6: u32,
    pub reserved7: u32,
    pub reserved8: u32,
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
