use super::*;

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