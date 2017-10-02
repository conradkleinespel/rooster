#![allow(dead_code)]
//! Standard clipboard formats.
//!
//! Header: Winuser.h
//!
//! Description is taken from [Standard Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)

///A handle to a bitmap (HBITMAP).
pub const CF_BITMAP: u32 = 2;
///A memory object containing a <b>BITMAPINFO</b> structure followed by the bitmap bits.
pub const CF_DIB: u32 = 8;
///A memory object containing a <b>BITMAPV5HEADER</b> structure followed by the bitmap color space
///information and the bitmap bits.
pub const CF_DIBV5: u32 = 17;
///Software Arts' Data Interchange Format.
pub const CF_DIF: u32 = 5;
///Bitmap display format associated with a private format. The hMem parameter must be a handle to
///data that can be displayed in bitmap format in lieu of the privately formatted data.
pub const CF_DSPBITMAP: u32 = 0x0082;
///Enhanced metafile display format associated with a private format. The *hMem* parameter must be a
///handle to data that can be displayed in enhanced metafile format in lieu of the privately
///formatted data.
pub const CF_DSPENHMETAFILE: u32 = 0x008E;
///Metafile-picture display format associated with a private format. The hMem parameter must be a
///handle to data that can be displayed in metafile-picture format in lieu of the privately
///formatted data.
pub const CF_DSPMETAFILEPICT: u32 = 0x0083;
///Text display format associated with a private format. The *hMem* parameter must be a handle to
///data that can be displayed in text format in lieu of the privately formatted data.
pub const CF_DSPTEXT: u32 = 0x0081;
///A handle to an enhanced metafile (<b>HENHMETAFILE</b>).
pub const CF_ENHMETAFILE: u32 = 14;
///Start of a range of integer values for application-defined GDI object clipboard formats.
pub const CF_GDIOBJFIRST: u32 = 0x0300;
///End of a range of integer values for application-defined GDI object clipboard formats.
pub const CF_GDIOBJLAST: u32 = 0x03FF;
///A handle to type <b>HDROP</b> that identifies a list of files.
pub const CF_HDROP: u32 = 15;
///The data is a handle to the locale identifier associated with text in the clipboard.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_LOCALE: u32 = 16;
///Handle to a metafile picture format as defined by the <b>METAFILEPICT</b> structure.
pub const CF_METAFILEPICT: u32 = 3;
///Text format containing characters in the OEM character set.
pub const CF_OEMTEXT: u32 = 7;
///Owner-display format.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_OWNERDISPLAY: u32 = 0x0080;
///Handle to a color palette.
///
///For details see [Standart Clipboard Formats](https://msdn.microsoft.com/en-us/library/windows/desktop/ff729168%28v=vs.85%29.aspx)
pub const CF_PALETTE: u32 = 9;
///Data for the pen extensions to the Microsoft Windows for Pen Computing.
pub const CF_PENDATA: u32 = 10;
///Start of a range of integer values for private clipboard formats.
pub const CF_PRIVATEFIRST: u32 = 0x0200;
///End of a range of integer values for private clipboard formats.
pub const CF_PRIVATELAST: u32 = 0x02FF;
///Represents audio data more complex than can be represented in a ```CF_WAVE``` standard wave format.
pub const CF_RIFF: u32 = 11;
///Microsoft Symbolic Link (SYLK) format.
pub const CF_SYLK: u32 = 4;
///ANSI text format.
pub const CF_TEXT: u32 = 1;
///Tagged-image file format.
pub const CF_TIFF: u32 = 6;
///UTF16 text format.
pub const CF_UNICODETEXT: u32 = 13;
///Represents audio data in one of the standard wave formats.
pub const CF_WAVE: u32 = 12;
