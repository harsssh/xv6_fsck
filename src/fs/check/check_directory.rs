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

    pub fn check_current_directory(&self) -> Result<(), FSError> {
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DIR {
                self.check_current_directory_individual(inum as u16)?;
            }
        }
        Ok(())
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

    pub fn check_parent_directory(&self) -> Result<(), FSError> {
        for (inum, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ == FileType::DIR {
                self.check_parent_directory_individual(inum as u16)?;
            }
        }
        Ok(())
    }
}
