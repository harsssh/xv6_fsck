use super::*;

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
