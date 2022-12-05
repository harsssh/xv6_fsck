use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use nom::{combinator, Parser};
use nom::bits;
use nom::bytes;
use nom::IResult;
use nom::multi;
use nom::number::complete::{le_u16, le_u32};
use crate::fs;
use crate::fs::{SuperBlock, Dinode, FileType, BlockStatus, FS, Dirent};

pub fn read_img(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    buf
}

fn parse_superblock(input: &[u8]) -> IResult<&[u8], SuperBlock> {
    let mut parser = combinator::map(
        multi::count(le_u32, 8),
        |v| {
            let magic = v[0];
            if magic != fs::FSMAGIC {
                // TODO: Error handling
                panic!("invalid magic number");
            }
            let size = v[1];
            let nblocks = v[2];
            let ninodes = v[3];
            let nlog = v[4];
            let logstart = v[5];
            let inodestart = v[6];
            let bmapstart = v[7];
            SuperBlock::new(magic, size, nblocks, ninodes, nlog, logstart, inodestart, bmapstart)
        },
    );

    let (input, superblock) = parser.parse(input)?;

    // Skip to end of block
    let remaining = fs::BSIZE - size_of::<SuperBlock>();
    let (input, _) = bytes::complete::take(remaining).parse(input)?;

    Ok((input, superblock))
}

fn parse_file_type(input: &[u8]) -> IResult<&[u8], FileType> {
    let (input, typ) = le_u16(input)?;
    let typ = match typ {
        0 => FileType::UNUSED,
        1 => FileType::DIR,
        2 => FileType::FILE,
        3 => FileType::DEV,
        // TODO: Error handling
        _ => panic!("invalid file type"),
    };
    Ok((input, typ))
}

fn parse_addrs(input: &[u8], addrs_offset: u32) -> IResult<&[u8], [Option<u32>; fs::NDIRECT + 1]> {
    let (input, addrs) = multi::count(le_u32, fs::NDIRECT + 1).parse(input)?;
    let addrs = addrs
        .into_iter()
        .map(|x|
            // TODO: Error handling
            if x == 0 { None } else if x >= addrs_offset { Some(x - addrs_offset) } else { panic!("invalid address") })
        .collect::<Vec<Option<u32>>>()
        .try_into()
        .unwrap();
    Ok((input, addrs))
}

fn parse_dinode(input: &[u8], addrs_offset: u32) -> IResult<&[u8], Dinode> {
    let (input, typ) = parse_file_type(input)?;
    let (input, major) = le_u16(input)?;
    let (input, minor) = le_u16(input)?;
    let (input, nlink) = le_u16(input)?;
    let (input, size) = le_u32(input)?;
    let (input, addrs) = parse_addrs(input, addrs_offset)?;
    Ok((input, Dinode::new(typ, major, minor, nlink, size, addrs)))
}

fn parse_dinodes(input: &[u8], blocks: usize, addrs_offset: u32) -> IResult<&[u8], Vec<Dinode>> {
    let n = blocks * fs::IPB;
    let mut parser = multi::count(|i| parse_dinode(i, addrs_offset), n);
    parser.parse(input)
}

fn parse_bit(input: (&[u8], usize)) -> IResult<(&[u8], usize), BlockStatus> {
    let mut parser = bits::complete::take(1usize);
    let ((input, offset), bit) = parser.parse(input)?;
    let status = match bit {
        0 => BlockStatus::Free,
        1 => BlockStatus::Allocated,
        // TODO: Error handling
        _ => panic!("invalid status"),
    };
    Ok(((input, offset), status))
}

// TODO: refactor
fn parse_bitmap(input: &[u8], blocks: usize) -> IResult<&[u8], Vec<BlockStatus>> {
    let n = blocks * fs::BSIZE * 8;
    let mut parser = multi::count(parse_bit, n);
    let offset = 0;
    let (input, output) = parser((input, offset)).unwrap();
    Ok((input.0, output))
}

fn read_block(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (input, block) = bytes::complete::take(fs::BSIZE).parse(input)?;
    let block = block.to_vec();
    Ok((input, block))
}

fn parse_data(input: &[u8], blocks: usize) -> IResult<&[u8], Vec<Vec<u8>>> {
    let mut parser = multi::count(read_block, blocks);
    parser.parse(input)
}

fn skip_block(input: &[u8], n: usize) -> IResult<&[u8], ()> {
    let mut parser = bytes::complete::take(fs::BSIZE * n);
    let (input, _) = parser.parse(input)?;
    Ok((input, ()))
}

fn parse_dirname(input: &[u8]) -> IResult<&[u8], String> {
    let mut parser = bytes::complete::take(fs::DIRSIZ);
    let (input, dirname) = parser.parse(input)?;
    let dirname = std::str::from_utf8(dirname).unwrap();
    let dirname = dirname.trim_end_matches('\0').to_string();
    Ok((input, dirname))
}

pub fn parse_dirent(input: &[u8]) -> Dirent {
    let (input, inum) = le_u16::<_, nom::error::Error<_>>(input).unwrap();
    let (_, name) = parse_dirname(input).unwrap();
    Dirent::new(inum, name)
}

pub fn parse_fs(input: &[u8]) -> FS {
    let (input, _) = skip_block(input, 1).unwrap();
    let (input, sb) = parse_superblock(input).unwrap();

    let ninodeblocks: usize = sb.ninodes as usize / fs::IPB + 1;
    let nbitmap: usize = sb.size as usize / fs::BPB + 1;
    let datastart: u32 = sb.size - sb.nblocks;

    let (input, _) = skip_block(input, sb.nlog as usize).unwrap();
    let (input, dinodes) = parse_dinodes(input, ninodeblocks, datastart).unwrap();
    let (input, bitmap) = parse_bitmap(input, nbitmap).unwrap();
    let (_, data) = parse_data(input, sb.nblocks as usize).unwrap();

    FS::new(sb, dinodes, bitmap, data)
}
