use super::*;

impl Dirent {
    pub fn new(inum: u16, name: String) -> Self {
        Dirent {
            inum,
            name,
        }
    }
}