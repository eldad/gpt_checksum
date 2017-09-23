use std::fmt;

use std::io;
use std::mem;
use std::slice;

#[repr(C, packed)]
pub struct ProtectiveMBR {
    boot_code: [u8; 440],
    uniqueMbrDiskSignature: [u8; 4],
    unknown444: [u8; 2],
    //partitionRecords: [[u8; 16] ; 4],
    partitionRecords: [u8; 64],
    signature: [u8; 2],
    // Reserved: Offset 512, Size: LBA - 512.
}

pub fn a_struct_size<T, R: io::Read>(read: &mut R) -> usize {
    mem::size_of::<T>()
}

fn read_to_struct<T, R: io::Read>(read: &mut R) -> io::Result<T> {
    let size = mem::size_of::<T>();
    //eprintln!("Size = {}", size);
    //eprintln!("Size 2 = {}", mem::size_of::<ProtectiveMBR>());
    println!("read_to_struct size {}", mem::size_of::<T>());

    let mut buf: [u8; 512];
    match read.read_exact(&mut buf) {
        Ok(_) => 
    }

    /*let size = 1;
    unsafe {
        let mut ret = mem::uninitialized();
        let buf = slice::from_raw_parts_mut(&mut ret as *mut T as *mut u8, size);
        match read.read_exact(buf) {
            Ok(_) => Ok(ret),
            Err(e) => {
                mem::forget(ret);
                Err(e)
            }
        }
    }*/
}

impl ProtectiveMBR {
    pub fn valid_signature(&self) -> bool {
        self.signature[0] == 0x55 && self.signature[1] == 0xAA
    }

    pub fn boot_code_repr(&self) -> String {
        format!("[{:02X} {:02X} {:02X} {:02X} ...]", 
            self.boot_code[0],
            self.boot_code[1],
            self.boot_code[2],
            self.boot_code[3],
        )
    }

    pub fn signature_repr(&self) -> String {
        format!("[{:02X} {:02X}] {}",
            self.signature[0],
            self.signature[1],
            match self.valid_signature() {
                true => "valid",
                false => "invalid",
            }
        )
    }

    pub fn from_read<R: io::Read>(read: &mut R) -> Result<ProtectiveMBR, io::Error> {
        read_to_struct(read)?
    }
/*
    pub fn from_read_works<R: io::Read>(read: R) -> Result<ProtectiveMBR, io::Error> {
        ProtectiveMBR::read_to_struct(read)
    }

    fn read_to_struct<R: io::Read>(mut read: R) -> Result<ProtectiveMBR, io::Error> {
        let size = mem::size_of::<ProtectiveMBR>();
        eprintln!("Size = {}", size);
        unsafe {
            let mut ret = mem::uninitialized();
            let buf = slice::from_raw_parts_mut(&mut ret as *mut ProtectiveMBR as *mut u8, size);
            match read.read_exact(buf) {
                Ok(_) => Ok(ret),
                Err(e) => {
                    mem::forget(ret);
                    Err(e)
                }
            }
        }
    }*/
}

impl fmt::Debug for ProtectiveMBR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Protective MBR: Boot Code {}, Signature: {}>",
            self.boot_code_repr(),
            self.signature_repr(),
        )
    }
}