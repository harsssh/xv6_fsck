// root i-number
pub const ROOTINO: u16 = 1;
// block size
pub const BSIZE: usize = 1024;

#[derive(Debug, Clone, PartialEq)]
pub struct SuperBlock {
    // Must be FSMAGIC
    pub magic: u32,
    // Size of file system image (blocks)
    pub size: u32,
    // Number of data blocks
    pub nblocks: u32,
    // Number of inodes.
    pub ninodes: u32,
    // Number of log blocks
    pub nlog: u32,
    // Block number of first log block
    pub logstart: u32,
    // Block number of first inode block
    pub inodestart: u32,
    // Block number of first free map block
    pub bmapstart: u32,
}

pub const FSMAGIC: u32 = 0x10203040;

pub const NDIRECT: usize = 12;
pub const NINDIRECT: usize = BSIZE / 4;
pub const MAXFILE: usize = NDIRECT + NINDIRECT;

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    UNUSED,
    DIR,
    FILE,
    DEV,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dinode {
    // File type
    pub typ: FileType,
    // Major device number (DEV only)
    pub major: u16,
    // Minor device number (DEV only)
    pub minor: u16,
    // Number of links to inode in file system
    pub nlink: u16,
    // Size of file (bytes)
    pub size: u32,
    // Data block addresses
    pub addrs: [u32; NDIRECT + 1],
}

// Inodes per block
pub const IPB: usize = BSIZE / 64;

// Bitmap bits per block
pub const BPB: usize = BSIZE * 8;

pub const DIRSIZ: usize = 14;

#[derive(Debug, Clone, PartialEq)]
pub struct Dirent {
    // Inode number
    pub inum: u16,
    // File name
    pub name: String,
}
