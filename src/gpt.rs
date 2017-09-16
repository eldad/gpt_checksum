/*

UEFI Specifications: http://www.uefi.org/specifications

*/

use std::fmt;
use std::mem;

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
    starting_lba: [u8; 4],
    size_in_lba: [u8; 4],       /* Size of the disk minus one, 0xFFFFFFFF = too big for this field */
    // Partition Record #2-4 set to zero (UEFI spec clause 5.2.3)
    partition_records: [u8; 48],
    signature: [u8; 2],
    // Reserved: Offset 512, Size: LBA - 512.
}

impl Default for ProtectiveMBR {
    fn default() -> ProtectiveMBR {
        unsafe {
            mem::zeroed()
        }
    }
}

impl ProtectiveMBR {
    pub fn valid_signature(&self) -> bool {
        self.signature[0] == 0x55 && self.signature[1] == 0xAA
    }

    pub fn boot_code_repr(&self) -> String {
        if {
            let mut zero_check = 0u8;
            for x in self.boot_code.into_iter() {
                zero_check |= *x;
            }
            zero_check == 0
        } {
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
        format!("[{:02X} {:02X}] {}",
            self.signature[0],
            self.signature[1],
            if self.valid_signature() { "valid" } else { "invalid" }
        )
    }

    pub fn os_type(&self) -> String {
        if self.os_type == 0xEE {
            String::from("GPT Protective (0xEE)")
        } else {
            format!("Unknown ({:02X})", self.os_type)
        }
    }
}

impl fmt::Debug for ProtectiveMBR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Protective MBR: Boot Code {}, Signature: {}, OS Type: {}>",
            self.boot_code_repr(),
            self.signature_repr(),
            self.os_type(),
        )
    }
}