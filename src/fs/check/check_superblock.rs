use super::*;
use crate::fs::error::FSError;

impl SuperBlock {
    pub fn check_fields(&self) -> Vec<FSError> {
        let err = vec![FSError::IncorrectSuperBlockField(self)];
        if self.size != FSSIZE as u32 {
            return err;
        }
        if self.nblocks != FSSIZE as u32 {
            return err;
        }
        if self.ninodes != NINODES as u32 {
            return err;
        }
        if self.nlog != LOGSIZE as u32 {
            return err;
        }
        if self.logstart != LOGSTART as u32 {
            return err;
        }
        if self.inodestart != INODESTART as u32 {
            return err;
        }
        if self.bmapstart != BITMAPSTART as u32 {
            return err;
        }
        vec![]
    }
}