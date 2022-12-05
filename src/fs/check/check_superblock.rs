use super::*;
use crate::fs::error::FSError;

impl SuperBlock {
    pub fn check_fields(&self) -> Result<(), FSError> {
        if self.size == FSSIZE as u32 {
            return Ok(());
        }
        if self.nblocks == FSSIZE as u32 {
            return Ok(());
        }
        if self.ninodes == NINODES as u32 {
            return Ok(());
        }
        if self.nlog == LOGSIZE as u32 {
            return Ok(());
        }
        if self.logstart == LOGSTART as u32 {
            return Ok(());
        }
        if self.inodestart == INODESTART as u32 {
            return Ok(());
        }
        if self.bmapstart == BITMAPSTART as u32 {
            return Ok(());
        }

        Err(FSError::IncorrectSuperBlockField(self))
    }
}