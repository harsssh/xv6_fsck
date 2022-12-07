use super::*;
use thiserror::Error;

// TODO: refine
#[derive(Debug, Error)]
pub enum FSError<'a> {
    /* About superblock */
    #[error("incorrect field value in superblock: {0:?}")]
    IncorrectSuperBlockField(&'a SuperBlock),

    /* About block usage */
    // (data block number, status of block)
    #[error("bitmap assumes data block {0} is {1:?}, but this is incorrect")]
    IncorrectBitmap(u32, &'a BlockStatus),
    // (data block number)
    #[error("{0}-th data block is referenced from multiple inodes")]
    MultipleRef(u32),

    /* About inode */
    // (inode number)
    #[error("{0}-th inode is a device file, but its major/minor number is invalid")]
    InvalidDevice(u16),
    // Note that in the case of directories, references by "." is not counted
    // (inode number, nlink)
    #[error("{0}-th inode assumes nlink is {1}, but this is incorrect")]
    IncorrectNLink(u16, u16),
    // (inode number)
    #[error("{0}-th inode refers to a freed data block")]
    InvalidDataBlockRef(u16),
    // Must be ceil(dinode.size/BSIZE)
    // (inode number, number of valid references in addrs)
    #[error("{0}-th inode has size {1} and refers to {2} data blocks, but this is invalid")]
    InvalidNumberOfDataBlockRef(u16, u32, usize),

    /* About directory */
    // (inode number of directory)
    #[error("{0}-th inode of directory refers to an unused inode")]
    InvalidInodeRef(u16),
    // (inode number of directory)
    #[error("{0}-th inode of directory does not refer to itself by \".\"")]
    IncorrectCurrentDirRef(u16),
    // Note that for "/", it refers to itself
    // (inode number of directory)
    #[error("{0}-th inode of directory does not refer to parent directory by \"..\"")]
    IncorrectParentDirRef(u16),
    // Must be referenced only by itself, its parent directories, and subdirectories
    // (inode number of directory)
    #[error("{0}-th inode of directory is falsely referenced by other directories")]
    InvalidDirRef(u16),

    /* Others */
    #[error("{0}-th inode cannot be traced from the root directory")]
    DanglingInode(u16),
}