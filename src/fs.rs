pub mod consts;
pub mod check;
pub mod error;
mod implement;

pub use consts::*;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum FileType {
    UNUSED,
    DIR,
    FILE,
    DEV,
}

#[derive(Debug, PartialEq)]
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
    pub addrs: [Option<u32>; NDIRECT + 1],
}

#[derive(Debug, PartialEq)]
pub struct Dirent {
    // Inode number
    pub inum: u16,
    // File name
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum BlockStatus {
    Free,
    Allocated,
}

pub struct FS {
    pub superblock: SuperBlock,
    pub dinodes: Vec<Dinode>,
    pub bitmap: Vec<BlockStatus>,
    pub data: Vec<Vec<u8>>,
}
