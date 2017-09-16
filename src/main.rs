/*

Copyright Eldad Zack 2017
License: MIT

 */
use std::env;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::error::Error;

use std::slice;

#[cfg(test)]
mod tests;

mod gpt;
use gpt::*;

fn unsafe_read_to_struct<T: Default, SRC: Read>(mut src: SRC) -> Result<T, Box<Error>> {
    let mut ret = T::default();
    let s: &mut [u8] = unsafe {
        let p = (&mut ret) as *mut T as *mut u8;
        slice::from_raw_parts_mut(p, mem::size_of::<T>())
    };
    src.read_exact(s)?;
    Ok(ret)
}

fn gpt_checksum(filename: &str) -> Result<(), Box<Error>>
{
    eprintln!("* Opening '{}'", filename);
    let f: File = File::open(filename)?;

    let pmbr: ProtectiveMBR = unsafe_read_to_struct(f)?;
    eprintln!("{:?}", pmbr);
    Ok(())
}

fn _main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected exactly one parameter.");
    }

    gpt_checksum(&args[1])?;

    Ok(())
}

pub fn main() {
    if let Err(e) = _main() {
        panic!("Error: {}", e);
    }
}