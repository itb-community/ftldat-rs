
/// PKG\n
pub(super) static PKG_SIGNATURE: [u8; 4] = [80, 75, 71, 10];
pub(super) static INDEX_SIZE: u16 = 16;
pub(super) static ENTRY_SIZE: u16 = 20;
/// Bitmask flag for deflate compression
pub(super) static PKG_DEFLATED: u8 = 0x01;
