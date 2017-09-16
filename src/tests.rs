use std::mem;
use gpt::*;
use util;

#[test]
fn gpt_struct_sizes() {
    assert_eq!(mem::size_of::<ProtectiveMBR>(), 512);
    assert_eq!(mem::size_of::<GptHeader>(), 92);
    assert_eq!(mem::size_of::<GptPart>(), 128);
}

#[test]
fn boot_code_repr() {
    let pmbr = ProtectiveMBR::default();
    assert_eq!(pmbr.boot_code_repr(), "[Zero]");

    let mut pmbr = ProtectiveMBR::default();
    pmbr.boot_code[0] = 0x01;
    pmbr.boot_code[1] = 0x23;
    pmbr.boot_code[2] = 0xAB;
    pmbr.boot_code[3] = 0xEF;
    assert_eq!(pmbr.boot_code_repr(), "[01 23 AB EF ...]");

    let mut pmbr = ProtectiveMBR::default();
    pmbr.boot_code[4] = 0x01;
    assert_eq!(pmbr.boot_code_repr(), "[00 00 00 00 ...]");
}

#[test]
fn crc32_zlib() {
    let check_input: &[u8] = b"123456789";
    assert_eq!(util::crc32(check_input), 0xCBF43926);
}
