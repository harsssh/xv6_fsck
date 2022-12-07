use super::*;
use crate::fs::error::FSError;

impl FS {
    fn check_current_directory_individual(&self, inum: u16) -> Result<(), FSError> {
        let dirents = match self.get_dirents(&inum) {
            Some(dirents) => dirents,
            // not a directory
            None => return Ok(()),
        };

        let mut inum_from_dirents = 0;
        for dirent in dirents {
            if dirent.name == "." {
                inum_from_dirents = dirent.inum;
                break;
            }
        }

        if inum_from_dirents == inum {
            Ok(())
        } else {
            Err(FSError::IncorrectCurrentDirRef(inum))
        }
    }

    pub fn check_current_directory(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DIR {
                if let Err(e) = self.check_current_directory_individual(inum as u16) {
                    errors.push(e);
                }
            }
        }
        errors
    }

    fn check_parent_directory_individual(&self, inum: u16) -> Result<(), FSError> {
        let dirents = match self.get_dirents(&inum) {
            Some(dirents) => dirents,
            // not a directory
            None => return Ok(()),
        };

        let mut inum_from_dirents = 0;
        for dirent in dirents {
            if dirent.name == ".." {
                inum_from_dirents = dirent.inum;
                break;
            }
        }

        let dir = self.get_node(&inum).unwrap();
        let parents = dir.parents.borrow();
        assert_eq!(parents.len(), 1);
        let parent = parents[0].upgrade().unwrap();

        let inum_from_node = parent.value;
        if inum_from_dirents == inum_from_node {
            Ok(())
        } else {
            Err(FSError::IncorrectParentDirRef(inum))
        }
    }

    pub fn check_parent_directory(&self) -> Vec<FSError> {
        let mut errors = vec![];
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DIR {
                if let Err(e) = self.check_parent_directory_individual(inum as u16) {
                    errors.push(e);
                }
            }
        }
        errors
    }

    fn check_directory_ref_individual(&self, inum: u16) -> Result<(), FSError> {
        if self.dinodes[inum as usize].typ != FileType::DIR {
            return Ok(());
        }

        let dir = self.get_node(&inum).unwrap();
        if dir.parents.borrow().len() == 1 {
            Ok(())
        } else {
            Err(FSError::InvalidDirRef(inum))
        }
    }

    // Whether the directory is referenced only by its parent and children
    pub fn check_directory_ref(&self) -> Result<(), FSError> {
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DIR {
                self.check_directory_ref_individual(inum as u16)?;
            }
        }
        Ok(())
    }

    fn check_dirents_individual(&self, inum: u16) -> Result<(), FSError> {
        let dirents = match self.get_dirents(&inum) {
            Some(dirents) => dirents,
            // not a directory
            None => return Ok(()),
        };

        let valid = dirents.iter().all(|dirent| {
            let i = dirent.inum;
            self.dinodes[i as usize].typ != FileType::UNUSED
        });

        if valid {
            Ok(())
        } else {
            Err(FSError::InvalidInodeRef(inum))
        }
    }

    pub fn check_dirents(&self) -> Result<(), FSError> {
        for (i, dindoe) in self.dinodes.iter().enumerate() {
            if dindoe.typ == FileType::DIR {
                self.check_dirents_individual(i as u16)?;
            }
        }
        Ok(())
    }
}
