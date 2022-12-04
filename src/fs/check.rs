use super::*;

mod check_superblock;

/* For consistency check */
// Size of file system image (blocks)
const FSSIZE: usize = 2000;
// Number of inodes
const NINODES: usize = 200;

// Number of log blocks
const LOGSIZE: usize = 30;
// Number of inode blocks
const NINODEBLOCKS: usize = NINODES / IPB + 1;
// Number of bitmap blocks
const NBITMAP: usize = FSSIZE / BPB + 1;
// Number of data blocks
const NBLOCKS: usize = FSSIZE - LOGSIZE - NINODEBLOCKS - NBITMAP - 2;

const LOGSTART: usize = 2;
const INODESTART: usize = LOGSTART + LOGSIZE;
const BITMAPSTART: usize = INODESTART + NINODEBLOCKS;
