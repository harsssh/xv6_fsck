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

    pub fn check_device_numbers(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for (i, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DEV {
                if let Err(e) = self.check_device_numbers_individual(i as u16) {
                    errors.push(e);
                }
            }
        }
        errors
    }

    fn check_addrs_ref_individual(&self, inum: u16) -> Result<(), FSError> {
        let dinode = &self.dinodes[inum as usize];
        let valid = self.get_all_addrs(dinode).iter().all(|addr|
            self.bitmap[*addr as usize] == BlockStatus::Allocated
        );

        if valid {
            Ok(())
        } else {
            Err(FSError::InvalidDataBlockRef(inum))
        }
    }

    // Assuming the bitmap is valid
    pub fn check_addrs_ref(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for i in 0..self.dinodes.len() {
            if let Err(e) = self.check_addrs_ref_individual(i as u16) {
                errors.push(e);
            }
        }
        errors
    }

    // FIXME: Raise error about valid file system
    fn check_addrs_len_individual(&self, inum: u16) -> Result<(), FSError> {
        let dinode = &self.dinodes[inum as usize];
        let correct = (dinode.size as f64 / BSIZE as f64).ceil() as usize;
        let len = self.get_all_addrs(dinode).len();

        if len == correct {
            Ok(())
        } else {
            Err(FSError::InvalidNumberOfDataBlockRef(inum, dinode.size, len))
        }
    }

    pub fn check_addrs_len(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for i in 0..self.dinodes.len() {
            if let Err(e) = self.check_addrs_len_individual(i as u16) {
                errors.push(e);
            }
        }
        errors
    }

    fn check_nlink_individual(&self, inum: u16) -> Result<(), FSError> {
        let dinode = &self.dinodes[inum as usize];
        match dinode.typ {
            FileType::FILE => {
                let dir = self.get_node(&inum).unwrap();
                let ref_count = dir.parents.borrow().len() as u16;
                if dinode.nlink == ref_count {
                    Ok(())
                } else {
                    Err(FSError::IncorrectNLink(inum, dinode.nlink))
                }
            }
            FileType::DIR => {
                let dir = self.get_node(&inum).unwrap();
                assert_eq!(dir.parents.borrow().len(), 1);
                // reference from parent directory
                let mut ref_count = dir.parents.borrow().len() as u16;
                // reference from child directories
                for child in dir.children.borrow().iter() {
                    let inum_child = child.value;
                    let dinode_child = &self.dinodes[inum_child as usize];
                    if dinode_child.typ == FileType::DIR {
                        ref_count += 1;
                    }
                }

                if dinode.nlink == ref_count {
                    Ok(())
                } else {
                    Err(FSError::IncorrectNLink(inum, dinode.nlink))
                }
            }
            // TODO: check nlink for device file
            FileType::DEV => Ok(()),
            FileType::UNUSED => Ok(()),
        }
    }

    // Assuming that reference by ".." is correct
    // and directories must be referenced only by their parent and child directories
    pub fn check_nlink(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ != FileType::UNUSED {
                if let Err(e) = self.check_nlink_individual(inum as u16) {
                    errors.push(e);
                }
            }
        }
        errors
    }
}
