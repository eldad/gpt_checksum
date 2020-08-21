/*
 * Copyright Eldad Zack 2017
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
 * associated documentation files (the "Software"), to deal in the Software without restriction, including without
 * limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
 * PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * https://opensource.org/licenses/MIT
 *
 * Assumes block size of 512 (compare with output of `lsblk -o name,phy-sec`)
 *
 */

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::mem;
use std::error::Error;
use std::io::SeekFrom;

use std::slice;

mod util;

mod gpt;
use crate::gpt::*;

#[cfg(test)]
mod tests;

const BLOCK_SIZE: usize = 512;

fn unsafe_read_to_struct<T: Default, H: Read>(src: &mut H) -> Result<T, Box<Error>> {
    let mut ret = T::default();
    let s: &mut [u8] = unsafe {
        let p = (&mut ret) as *mut T as *mut u8;
        slice::from_raw_parts_mut(p, mem::size_of::<T>())
    };
    src.read_exact(s)?;
    Ok(ret)
}

fn unsafe_write_struct<T, H: Write>(dst: &mut H, data: &T) -> Result<(), Box<Error>> {
    let len = mem::size_of::<T>();
    let bytes = unsafe {
        let p = data as *const T as *const u8;
        slice::from_raw_parts(p, len)
    };
    let written = dst.write(bytes)?;
    if len != written {
        panic!("Unexpected amount of bytes written: wrote {}, wanted to write {}", written, len);
    }
    Ok(())
}

fn seek_next_block_after<T: Sized, H: Seek>(handle: &mut H, block_size: usize) -> Result<(), Box<Error>>
{
    let len = mem::size_of::<T>();
    if len != block_size {
        let x: i64 = (block_size - len) as i64;
        handle.seek(SeekFrom::Current(x))?;
    }
    Ok(())
}

fn read_gpt(filename: &str) -> Result<Gpt, Box<Error>>
{
    println!("* Opening '{}'", filename);
    let mut f: File = File::open(filename)?;

    let pmbr: ProtectiveMBR = unsafe_read_to_struct(&mut f)?;
    seek_next_block_after::<ProtectiveMBR, File>(&mut f, BLOCK_SIZE)?;

    let header: GptHeader = unsafe_read_to_struct(&mut f)?;
    seek_next_block_after::<GptHeader, File>(&mut f, BLOCK_SIZE)?;

    let mut parts: Vec<GptPart> = Vec::new();
    for _ in 0..header.partition_entries {
        let gpt_part: GptPart = unsafe_read_to_struct(&mut f)?;
        parts.push(gpt_part);
        seek_next_block_after::<GptPart, File>(&mut f, header.partition_entry_size as usize)?;
    }

    Ok(Gpt {
        pmbr,
        header,
        parts,
    })
}

fn print_gpt_info(gpt: &Gpt) {
    println!("{:?}", gpt.pmbr);
    println!("{:?}",gpt.header);

    let part_table_crc32 = gpt_part_table_crc32(&gpt.parts);
    println!("** Partition Table CRC: {} ({:08X})",
        if part_table_crc32 == gpt.header.partition_entry_crc32 {
            "Valid"
        } else {
            "Invalid"
        },
        part_table_crc32,
    );

    let mut num_empty_parts = 0;
    for part in &gpt.parts {
        if part.is_empty() {
            num_empty_parts += 1;
        } else {
            println!("- {:?}", part);
        }
    }

    if num_empty_parts != 0 {
        println!("--     Empty partitions: {}", num_empty_parts);
    }
}

fn write_gpt(mut gpt: Gpt, filename: &str) -> Result<(), Box<Error>> {
    println!("*\n* Opening for WRITE: '{}'\n*", filename);
    let mut f: File = File::create(filename)?;

    /* Randomize and calculate CRC32 */
    gpt.header.disk_guid = util::urandom_uuid().unwrap();
    for part in &mut gpt.parts {
        if !part.is_empty() {
            part.unique_partition_guid = util::urandom_uuid().unwrap();
        }
    }
    gpt.header.partition_entry_crc32 = gpt_part_table_crc32(&gpt.parts);
    gpt.header.header_crc32 = gpt.header.crc32();
    print_gpt_info(&gpt);

    unsafe_write_struct(&mut f, &gpt.pmbr)?;
    seek_next_block_after::<ProtectiveMBR, File>(&mut f, BLOCK_SIZE)?;

    unsafe_write_struct(&mut f, &gpt.header)?;
    seek_next_block_after::<GptHeader, File>(&mut f, BLOCK_SIZE)?;

    for part in &gpt.parts {
        unsafe_write_struct(&mut f, part)?;
        seek_next_block_after::<GptPart, File>(&mut f, gpt.header.partition_entry_size as usize)?;
    }

    Ok(())
}

fn _main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        panic!("Expected one or two parameters: [input] ([output]).");
    }

    let gpt = read_gpt(&args[1])?;
    print_gpt_info(&gpt);

    if args.len() == 3 {
        if args[1] == args[2] {
            panic!("Error: input and output must not be the same");
        }
        write_gpt(gpt, &args[2]).expect("Cannot open file for writing");
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = _main() {
        panic!("Error: {}", e);
    }
}
