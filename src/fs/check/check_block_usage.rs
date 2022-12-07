use super::*;
use crate::fs::error::FSError;

impl FS {
    fn count_datablock_ref(&self) -> collections::HashMap<u32, u32> {
        let mut count = collections::HashMap::new();
        for dinode in self.dinodes.iter() {
            let addrs = self.get_all_addrs(dinode);
            for addr in addrs {
                *count.entry(addr).or_insert(0) += 1;
            }
        }
        count
    }

    // Check the number of references to data blocks
    pub fn check_datablock_ref(&self) -> Vec<FSError> {
        let mut errors = vec![];
        let count = self.count_datablock_ref();
        for (addr, v) in count.iter() {
            if *v > 1 {
                errors.push(FSError::MultipleRef(*addr));
            }
        }
        errors
    }

    // Check bitmap only for data blocks
    // Assume that the each data block references is at most 1
    pub fn check_bitmap(&self) -> Vec<FSError> {
        let mut errors = vec![];
        let count = self.count_datablock_ref();
        for (i, bmap) in self.bitmap.iter().enumerate() {
            if i < DATASTART {
                continue;
            }
            let addr = (i - DATASTART) as u32;
            let v = count.get(&addr).unwrap_or(&0);
            let status = match *v {
                0 => BlockStatus::Free,
                1 => BlockStatus::Allocated,
                _ => panic!("invalid ref count"),
            };
            if *bmap != status {
                errors.push(FSError::IncorrectBitmap(addr, bmap));
            }
        }
        errors
    }
}
