use std::fmt;
use std::mem;

#[repr(C, packed)]
pub struct ProtectiveMBR {
    boot_code: [u8; 440],
    unique_mbr_disk_signature: [u8; 4],
    unknown444: [u8; 2],
    //partitionRecords: [[u8; 16] ; 4],
    partition_records: [u8; 64],
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
            if self.valid_signature() { "valid" } else { "invalid" }
        )
    }
}

impl fmt::Debug for ProtectiveMBR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Protective MBR: Boot Code {}, Signature: {}>",
            self.boot_code_repr(),
            self.signature_repr(),
        )
    }
}