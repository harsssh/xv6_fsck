use crate::fs::consts::*;

/* For consistency check */
// Size of file system image (blocks)
pub const FSSIZE: usize = 2000;
// Number of inodes
pub const NINODES: usize = 200;

// Number of log blocks
pub const LOGSIZE: usize = 30;
// Number of inode blocks
pub const NINODEBLOCKS: usize = NINODES / IPB + 1;
// Number of bitmap blocks
pub const NBITMAP: usize = FSSIZE / BPB + 1;
// Number of data blocks
pub const NBLOCKS: usize = FSSIZE - LOGSIZE - NINODEBLOCKS - NBITMAP - 2;

pub const LOGSTART: usize = 2;
pub const INODESTART: usize = LOGSTART + LOGSIZE;
pub const BITMAPSTART: usize = INODESTART + NINODEBLOCKS;
pub const DATASTART: usize = BITMAPSTART + NBITMAP;
