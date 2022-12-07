mod consts;
mod check_superblock;
mod check_block_usage;
mod check_inode;
mod check_directory;

use super::*;
use crate::fs::error::FSError;
pub use crate::fs::check::consts::*;

impl FS {
    // TODO: Detect all errors for each item
    pub fn check(&self) -> Result<(), FSError> {
        /* Check superblock */
        self.superblock.check_fields()?;

        /* Check block usage */
        self.check_datablock_ref()?;
        // self.check_bitmap()?;

        /* Check directory */
        self.check_current_directory()?;
        self.check_parent_directory()?;

        /* Check inode */
        self.check_device_numbers()?;
        self.check_nlink()?;
        self.check_addrs_ref()?;
        // self.check_addrs_len()?;

        Ok(())
    }
}
