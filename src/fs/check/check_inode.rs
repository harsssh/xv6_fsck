use super::*;
use crate::fs::error::FSError;

impl Dinode {
    pub fn has_valid_device_numbers(&self) -> bool {
        // not a device file
        if self.typ != FileType::DEV {
            return true;
        }
        // REVIEW: Might not be the correct validation
        !(self.major == 0 && self.minor == 0)
    }
}

impl FS {
    pub fn check_device_numbers(&self, index: usize) -> Result<(), FSError> {
        let dinode = &self.dinodes[index];
        if dinode.typ != FileType::DEV {
            return Ok(());
        }
        if dinode.has_valid_device_numbers() {
            return Ok(());
        }
        Err(FSError::InvalidDevice(index as u16))
    }
}
