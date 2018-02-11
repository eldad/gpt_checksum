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
 */

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

/*
use test::Bencher;

#[bench]
fn bench_is_empty(b: &mut Bencher) {
    let part = GptPart::default();
    b.iter(|| part.is_empty());
}

#[bench]
fn bench_is_empty_u8(b: &mut Bencher) {
    let part = GptPart::default();
    b.iter(|| part.is_empty_u8());
}

#[bench]
fn bench_is_empty_u32(b: &mut Bencher) {
    let part = GptPart::default();
    b.iter(|| part.is_empty_u8());
}

#[bench]
fn bench_is_empty_jmp(b: &mut Bencher) {
    let part = GptPart::default();
    b.iter(|| part.is_empty_u8_jmp());
}
*/
