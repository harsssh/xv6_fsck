use super::*;
use thiserror::Error;

// TODO: refine naming
#[derive(Debug, Error)]
pub enum FSError {
    /* About superblock */
    #[error("incorrect field value in superblock")]
    IncorrectSuperBlockField(SuperBlock),

    /* About block usage */
    // (block number, status of block)
    #[error("bitmap assumes block {0} is {1}, but this is incorrect")]
    IncorrectBitmap(u32, BlockStatus),
    // (data block number, number of references)
    #[error("{0}-th data block is referenced from {1} inodes")]
    MultipleRef(u32, u32),

    /* About inode */
    #[error("major and minor of device are not set")]
    InvalidDevice,
    // Note that in the case of directories, references by "." is not counted
    // (inode number, nlink)
    #[error("{0}-th inode assumes nlink is {1}, but this is incorrect")]
    IncorrectNLink(u32, u32),
    // (inode number)
    #[error("{0}-th inode refers to a freed data block")]
    InvalidDataBlockRef(u32),
    // Must be ceil(dinode.size/BSIZE)
    // (inode number, number of valid references in addrs)
    #[error("{0}-th inode refers to {1} data blocks, but this is invalid")]
    InvalidNumberOfDataBlockRef(u32, u32),

    /* About directory */
    // (data block number of directory)
    #[error("{0}-th data block of directory refers to an unused inode")]
    InvalidInodeRef(u32),
    // (data block number of directory)
    #[error("{0}-th data block of directory does not refer to itself by \".\"")]
    IncorrectCurrentDirRef(u32),
    // Note that for "/", it refers to itself
    // (data block number of directory)
    #[error("{0}-th data block of directory does not refer to parent directory by \"..\"")]
    IncorrectParentDirRef(u32),
    // Must be referenced only by itself, its parent directories, and subdirectories
    // (data block number of directory)
    #[error("{0}-th data block of directory is falsely referenced by other directories")]
    InvalidDirRef(u32),
}