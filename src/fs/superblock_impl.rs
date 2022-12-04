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