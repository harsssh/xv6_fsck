use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use nom::{combinator, Parser};
use nom::bits;
use nom::bytes;
use nom::IResult;
use nom::multi;
use nom::sequence;
use nom::number::complete::{le_u16, le_u32};
use crate::fs;
use crate::fs::{SuperBlock, Dinode, FileType, BlockStatus, FS};

pub fn read_img(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    buf
}

pub fn parse_superblock(input: &[u8]) -> IResult<&[u8], SuperBlock> {
    let mut parser = combinator::map(
        multi::count(le_u32, 8),
        |v| {
            let magic = v[0];
            if magic != fs::FSMAGIC {
                panic!("Invalid magic number");
            }
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

    let (input, superblock) = parser.parse(input)?;

    // Skip to end of block
    let remaining = fs::BSIZE - size_of::<SuperBlock>();
    let (input, _) = bytes::complete::take(remaining).parse(input)?;

    Ok((input, superblock))
}

pub fn parse_dinode(input: &[u8]) -> IResult<&[u8], Dinode> {
    let mut parser = combinator::map(
        sequence::tuple((
            multi::count(le_u16, 4),
            le_u32,
            multi::count(le_u32, fs::NDIRECT + 1))
        )
        , |(u, v, w)| {
            let typ = match u[0] {
                0 => FileType::UNUSED,
                1 => FileType::DIR,
                2 => FileType::FILE,
                3 => FileType::DEV,
                _ => panic!("Invalid file type"),
            };
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

pub fn parse_dinodes(input: &[u8], blocks: usize) -> IResult<&[u8], Vec<Dinode>> {
    let n = blocks * fs::IPB;
    let mut parser = multi::count(parse_dinode, n);
    parser(input)
}

fn parse_bit(input: (&[u8], usize)) -> IResult<(&[u8], usize), BlockStatus> {
    let mut parser = bits::complete::take(1usize);
    let ((input, offset), bit) = parser.parse(input)?;
    let status = match bit {
        0 => BlockStatus::Free,
        1 => BlockStatus::Allocated,
        _ => panic!("Invalid bit"),
    };
    Ok(((input, offset), status))
}

// TODO: refactor
pub fn parse_bitmap(input: &[u8], blocks: usize) -> IResult<&[u8], Vec<BlockStatus>> {
    let n = blocks * fs::BSIZE * 8;
    let mut parser = multi::count(parse_bit, n);
    let offset = 0;
    let (input, output) = parser((input, offset)).unwrap();
    Ok((input.0, output))
}

fn read_block(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let parser = bytes::complete::take(fs::BSIZE);
    parser(input)
}

pub fn parse_data(input: &[u8], blocks: usize) -> IResult<&[u8], Vec<&[u8]>> {
    let mut parser = multi::count(read_block, blocks);
    parser(input)
}

fn skip_block(input: &[u8], n: usize) -> IResult<&[u8], ()> {
    let parser = bytes::complete::take(fs::BSIZE * n);
    let (input, _) = parser(input)?;
    Ok((input, ()))
}

pub fn parse_fs(input: &[u8]) -> FS {
    let (input, _) = skip_block(input, 1).unwrap();
    let (input, sb) = parse_superblock(input).unwrap();

    let ninodeblocks: usize = sb.ninodes as usize / fs::IPB + 1;
    let nbitmap: usize = sb.size as usize / fs::BPB + 1;

    let (input, _) = skip_block(input, sb.nlog as usize).unwrap();
    let (input, dinodes) = parse_dinodes(input, ninodeblocks).unwrap();
    let (input, bitmap) = parse_bitmap(input, nbitmap).unwrap();
    let (_, data) = parse_data(input, sb.nblocks as usize).unwrap();

    FS {
        superblock: sb,
        dinodes,
        bitmap,
        data,
    }
}
