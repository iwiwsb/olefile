#![allow(unused)]

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

pub mod data_structures;
pub mod ole_file;

pub trait Validation {
    fn validate(&self);
}
