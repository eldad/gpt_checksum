/*
 * UEFI Specifications: http://www.uefi.org/specifications
 * Structs/functions assume little-endian.
 */

use std::fmt;
use std::mem;

use std::slice;

use util;

pub struct Gpt {
    pub pmbr: ProtectiveMBR,
    pub header: GptHeader,
    pub parts: Vec<GptPart>,
}

/*
 * Protective MBR
 */

#[repr(C, packed)]
pub struct ProtectiveMBR {
    pub boot_code: [u8; 440],
    unique_mbr_disk_signature: [u8; 4],
    unknown444: [u8; 2],
    // Partition Record #1
    boot_indicator: u8,
    starting_chs: [u8; 3],
    os_type: u8,                /* 0xEE = GPT Protective */
    ending_chs: [u8; 3],
    starting_lba: u32,
    size_in_lba: u32,       /* Size of the disk minus one, 0xFFFFFFFF = too big for this field */
    // Partition Record #2-4 set to zero (UEFI spec clause 5.2.3)
    partition_records: [u8; 48],
    signature: u16,
    // Reserved: Offset 512, Size: LBA - 512.
}

impl Default for ProtectiveMBR {
    fn default() -> ProtectiveMBR {
        unsafe {
            mem::zeroed()
        }
    }
}

impl fmt::Debug for ProtectiveMBR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "* Protective MBR\n\
                   .  Boot Code: {}\n\
                   .  Signature: {}\n\
                   .  OS Type: {}",
            self.boot_code_repr(),
            self.signature_repr(),
            self.os_type_repr(),
        )
    }
}

impl ProtectiveMBR {
    pub fn valid_signature(&self) -> bool {
        self.signature == 0xAA55
    }

    pub fn boot_code_repr(&self) -> String {
        let zero_check = self.boot_code.iter().fold(0, |acc, &x| acc | x );
        if zero_check == 0 {
            String::from("[Zero]")
        } else {
            format!("[{:02X} {:02X} {:02X} {:02X} ...]",
                self.boot_code[0],
                self.boot_code[1],
                self.boot_code[2],
                self.boot_code[3],
            )
        }
    }

    pub fn signature_repr(&self) -> String {
        format!("{} [0x{:04X}]",
            if self.valid_signature() { "Valid" } else { "Invalid" },
            self.signature,
        )
    }

    pub fn os_type_repr(&self) -> String {
        format!("{} [0x{:02X}]",
            if self.os_type == 0xEE { "GPT Protective" } else { "Unknown" },
            self.os_type,
        )
    }
}

/*
 * GPT Header
 */

const GPT_HEADER_SIGNATURE: &[u8] = b"EFI PART";

#[derive(Clone)]
#[repr(C, packed)]
pub struct GptHeader {
    signature: [u8; 8],
    revision_minor: u16,
    revision_major: u16,
    header_size: u32,
    pub header_crc32: u32,
    reserved_20: [u8; 4],
    my_lba: u64,
    alternate_lba: u64,
    first_usable_lba: u64,
    last_usable_lba: u64,
    pub disk_guid: [u8; 16],
    partition_entry_lba: u64,
    pub partition_entries: u32,     /* num of */
    pub partition_entry_size: u32,
    pub partition_entry_crc32: u32,
}

impl Default for GptHeader {
    fn default() -> GptHeader {
        unsafe {
            mem::zeroed()
        }
    }
}

impl fmt::Debug for GptHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "* GPT Header [Size {}, Signature {}, Revision {}]\n\
                   .  Disk GUID {}\n\
                   .  Header CRC: {}\n\
                   .  Number of Partition Entries {}\n\
                   .  Partition Entry Size: {}",
            self.header_size_repr(),
            if self.is_signature_valid() { "Valid" } else { "Invalid" },
            self.revision_repr(),
            util::guid_to_string(&self.disk_guid),
            self.header_crc32_repr(),
            self.partition_entries,
            self.partition_entry_size,
        )
    }
}

impl GptHeader {
    pub fn is_signature_valid(&self) -> bool {
        self.signature == GPT_HEADER_SIGNATURE
    }

    pub fn revision_repr(&self) -> String {
        format!("{:?}.{:?}",
            self.revision_major,
            self.revision_minor,
        )
    }

    pub fn header_size_repr(&self) -> String {
        format!("{} ({})",
            if self.header_size as usize == (mem::size_of::<GptHeader>()) { "Correct" } else { "Incorrect" },
            self.header_size,
        )
    }

    pub fn header_crc32_repr(&self) -> String {
        let computed_crc = self.crc32();
        format!("{:08X} ({})",
            self.header_crc32,
            if self.header_crc32 == computed_crc {
                String::from("Valid")
            } else {
                format!("Invalid; Expected: {:08X}", computed_crc)
            }
        )
    }

    pub fn crc32(&self) -> u32 {
        /* Get a u8-array after zeroing the CRC32 field */
        let mut temp = self.clone();
        let s = unsafe {
            temp.header_crc32 = 0;
            let p = (&mut temp) as *mut GptHeader as *mut u8;
            slice::from_raw_parts_mut(p, mem::size_of::<GptHeader>())
        };

        util::crc32(s)
    }
}

/*
 * GPT Partition Entry
 */

pub fn gpt_part_table_crc32(parts: &[GptPart]) -> u32 {
    let mut buf: Vec<u8> = Vec::new();
    for part in parts.iter() {
        buf.extend(part.bytesvec());
    }
    util::crc32(&buf)
}

#[derive(Clone)]
#[repr(C, packed)]
pub struct GptPart {
    partition_type_guid: [u8; 16],
    pub unique_partition_guid: [u8; 16],
    starting_lba: u64,
    ending_lba: u64,
    attributes: u64,
    parition_name: [u8; 72], /* null terminated c string */
}

impl Default for GptPart {
    fn default() -> GptPart {
        unsafe {
            mem::zeroed()
        }
    }
}

impl fmt::Debug for GptPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GPT Partition #{} {}",
            util::guid_to_string(&self.unique_partition_guid),
            self.part_type_repr(),
        )
    }
}

impl GptPart {
    pub fn part_type_repr(&self) -> String {
        let guid = util::guid_to_string(&self.partition_type_guid);
         /* There are more types, but these I use. */
        let desc = {
                 if guid == "0fc63daf-8483-4772-8e79-3d69d8477de4" { "Linux Filesystem Data" }
            else if guid == "c12a7328-f81f-11d2-ba4b-00a0c93ec93b" { "EFI System Partition" }
            else if guid == "e3c9e316-0b5c-4db8-817d-f92df00215ae" { "Microsoft Reserved Partition (MSR)" }
            else if guid == "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7" { "Microsoft Windows Basic Data Partition" }
            else if guid == "de94bba4-06d1-4d40-a16a-bfd50179d6ac" { "Microsoft Windows Recovery Environment" }
            else                                                   { "Unknown" }
        };
        format!("{:40} <{}>", desc, guid)
    }

    pub fn is_empty(&self) -> bool {
        self.partition_type_guid.iter().fold(0, |acc, &x| acc | x) == 0 &&
        self.unique_partition_guid.iter().fold(0, |acc, &x| acc | x) == 0
    }

    pub fn bytesvec(&self) -> Vec<u8> {
        Vec::from(
            unsafe {
                let p = self as *const GptPart as *const u8;
                slice::from_raw_parts(p, mem::size_of::<GptPart>())
            }
        )
    }
}
