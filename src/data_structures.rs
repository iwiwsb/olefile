use std::fmt::Display;

pub struct CLSID {
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

pub struct FileTime {
    low_date_time: u32,
    high_date_time: u32,
}

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
