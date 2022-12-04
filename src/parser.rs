use std::fs::File;
use std::io::Read;
use nom::combinator;
use nom::IResult;
use nom::multi;
use nom::sequence;
use nom::number::complete::{le_u16, le_u32};
use crate::fs;
use crate::fs::{SuperBlock, Dinode};

pub fn read_img(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    buf
}

pub fn parse_superblock(input: &[u8]) -> IResult<&[u8], SuperBlock> {
    let parser = combinator::map(
        multi::count(le_u32, 8),
        |v| {
            let magic = v[0];
            let size = v[1];
            let nblocks = v[2];
            let ninodes = v[3];
            let nlog = v[4];
            let logstart = v[5];
            let inodestart = v[6];
            let bmapstart = v[7];
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
        },
    );
    combinator::verify(parser, |sb| sb.magic == fs::FSMAGIC)(input)
}

pub fn parse_dinode(input: &[u8]) -> IResult<&[u8], Dinode> {
    let mut parser = combinator::map(
        sequence::tuple((
            multi::count(le_u16, 4),
            le_u32,
            multi::count(le_u32, fs::NDIRECT + 1))
        )
        , |(u, v, w)| {
            let typ = u[0];
            let major = u[1];
            let minor = u[2];
            let nlink = u[3];
            let size = v;
            let addrs: [u32; fs::NDIRECT + 1] = w.try_into().unwrap();
            Dinode {
                typ,
                major,
                minor,
                nlink,
                size,
                addrs,
            }
        },
    );
    parser(input)
}
