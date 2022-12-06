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
    fn check_device_numbers_individual(&self, inum: u16) -> Result<(), FSError> {
        let dinode = &self.dinodes[inum as usize];
        if dinode.typ != FileType::DEV {
            return Ok(());
        }
        if dinode.has_valid_device_numbers() {
            return Ok(());
        }
        Err(FSError::InvalidDevice(inum as u16))
    }

    pub fn check_device_numbers(&self) -> Result<(), FSError> {
        for (i, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DEV {
                self.check_device_numbers_individual(i as u16)?;
            }
        }
        Ok(())
    }

    fn check_addrs_ref_individual(&self, inum: u16) -> Result<(), FSError> {
        let dinode = &self.dinodes[inum as usize];
        let valid = dinode.addrs.iter().all(|addr| {
            match addr {
                Some(addr) => self.bitmap[*addr as usize] == BlockStatus::Allocated,
                None => true,
            }
        });

        if valid {
            Ok(())
        } else {
            Err(FSError::InvalidDataBlockRef(inum))
        }
    }

    // Assuming the bitmap is valid
    pub fn check_addrs_ref(&self) -> Result<(), FSError> {
        for i in 0..self.dinodes.len() {
            self.check_addrs_ref_individual(i as u16)?;
        }
        Ok(())
    }
}
