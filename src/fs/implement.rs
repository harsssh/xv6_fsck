use super::*;

impl SuperBlock {
    pub fn new(magic: u32, size: u32, nblocks: u32, ninodes: u32, nlog: u32, logstart: u32, inodestart: u32, bmapstart: u32) -> Self {
        SuperBlock {
            magic,
            size,
            nblocks,
            ninodes,
            nlog,
            logstart,
            inodestart,
            bmapstart,
        }
    }
}

impl Dinode {
    pub fn new(typ: FileType, major: u16, minor: u16, nlink: u16, size: u32, addrs: [Option<u32>; NDIRECT + 1]) -> Self {
        Dinode {
            typ,
            major,
            minor,
            nlink,
            size,
            addrs,
        }
    }
}

impl Dirent {
    pub fn new(inum: u16, name: String) -> Self {
        Dirent {
            inum,
            name,
        }
    }
}

impl FS {
    pub fn new(superblock: SuperBlock, dinodes: Vec<Dinode>, bitmap: Vec<BlockStatus>, data: Vec<Vec<u8>>) -> Self {
        FS {
            superblock,
            dinodes,
            bitmap,
            data,
        }
    }
}
