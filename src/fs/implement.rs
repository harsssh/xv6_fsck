use super::*;

use std::collections;
use crate::parser;
use crate::util::tree::Node;

impl SuperBlock {
    pub fn new(magic: u32, size: u32, nblocks: u32, ninodes: u32, nlog: u32, logstart: u32, inodestart: u32, bmapstart: u32) -> Self {
        SuperBlock {
            magic,
            size,
            nblocks,
            ninodes,
            nlog,
            logstart,
            inodestart,
            bmapstart,
        }
    }
}

impl Dinode {
    pub fn new(typ: FileType, major: u16, minor: u16, nlink: u16, size: u32, addrs: [Option<u32>; NDIRECT + 1]) -> Self {
        Dinode {
            typ,
            major,
            minor,
            nlink,
            size,
            addrs,
        }
    }
}

impl Dirent {
    pub fn new(inum: u16, name: String) -> Self {
        Dirent {
            inum,
            name,
        }
    }
}

impl FS {
    pub fn new(superblock: SuperBlock, dinodes: Vec<Dinode>, bitmap: Vec<BlockStatus>, data: Vec<Vec<u8>>) -> Self {
        let mut fs = FS {
            superblock,
            dinodes,
            bitmap,
            data,
            inum_to_dirents: collections::HashMap::new(),
            directory_tree: Rc::new(Node::new(ROOTINO)),
        };
        fs.init();
        fs
    }

    pub fn get_dirents(&self, inum: &u16) -> Option<&Vec<Dirent>> {
        match self.inum_to_dirents.get(inum).unwrap() {
            Some(dirents) => Some(dirents),
            None => None,
        }
    }

    fn init(&mut self) {
        self.init_dirents_hashmap();
        self.construct_directory_tree();
    }

    fn init_dirents_hashmap(&mut self) {
        let map = &mut self.inum_to_dirents;
        for (i, dinode) in self.dinodes.iter().enumerate() {
            if dinode.typ != FileType::DIR {
                map.insert(i as u16, None);
                continue;
            }
            for v in dinode.addrs.iter() {
                if let Some(addr) = v {
                    let dirents = parser::parse_dirents(&self.data[*addr as usize]);
                    let dirents: Vec<Dirent> = dirents.into_iter().filter_map(|x| x).collect();
                    map.insert(i as u16, Some(dirents));
                }
            }
        }
    }

    fn construct_directory_tree(&mut self) {
        let root = &mut self.directory_tree;
        Node::set_parent(root, root);

        let q = &mut collections::VecDeque::new();
        q.push_back(Rc::clone(&root));
        while let Some(node) = q.pop_front() {
            let inum = node.value;
            if let Some(dirents) = self.get_dirents(&inum) {
                for dirent in dirents {
                    if dirent.name == "." || dirent.name == ".." {
                        continue;
                    }
                    let child = Rc::new(Node::new(dirent.inum));
                    Node::set_relation(&node, &child);
                    q.push_back(Rc::clone(&child));
                }
            }
        }
    }
}
