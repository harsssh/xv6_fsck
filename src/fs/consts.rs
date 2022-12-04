// root i-number
pub const ROOTINO: u16 = 1;
// block size
pub const BSIZE: usize = 1024;

pub const FSMAGIC: u32 = 0x10203040;

pub const NDIRECT: usize = 12;
pub const NINDIRECT: usize = BSIZE / 4;
pub const MAXFILE: usize = NDIRECT + NINDIRECT;

pub const INODESIZE: usize = 64;

// Inodes per block
pub const IPB: usize = BSIZE / INODESIZE;

// Bitmap bits per block
pub const BPB: usize = BSIZE * 8;

pub const DIRSIZ: usize = 14;
