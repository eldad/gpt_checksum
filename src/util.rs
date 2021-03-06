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

use std::fs::File;
use std::error::Error;
use std::io::Read;

/*
 * CRC32/zlib style
 * Everything's inverted: start with 0xffffffff in the register, and flip all bits at the end.
 * Use 0xEDB88320 as polynomial (bit mirrored 0x04C11DB7).
 * When reading each byte, we first xor it with the register, and then do the xor division business for each bit,
 * and then xor it into the register (this should be a lookup table... meh).
 * NOW everything makes sense, doesn't it.
 */
pub fn crc32(buf: &[u8]) -> u32 {
    const POLY: u32 = 0xED_B8_83_20;
    !buf.iter().fold(0xFF_FF_FF_FF, |reg, byte| {
        let mut control = (reg & 0xFF) ^ u32::from(*byte);
        for _ in 0..8 {
            if control & 1 == 1 {
                control = (control >> 1) ^ POLY;
            } else {
                control >>= 1;
            }
        }
        control ^ (reg >> 8)
    })
}

/*
 * GUID: fun for the whole family, big and little endian alike.
 * First three components are little endian, the last two are big endian.
 */
pub fn guid_to_string(guid: &[u8; 16]) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        guid[3], guid[2], guid[1], guid[0], // -
        guid[5], guid[4], // -
        guid[7], guid[6], // -
        guid[8], guid[9], // -
        guid[10], guid[11], guid[12], guid[13], guid[14], guid[15],
    )
}

/*
 * Pros: no crate required. Awesome randomness. No ugly seed business.
 * Cons: not portable. Oh, well.
 */
pub fn urandom_uuid() -> Result<[u8; 16], Box<dyn Error>> {
    let mut f = File::open("/dev/urandom")?;
    let mut buf: [u8; 16] = [0; 16];
    f.read_exact(&mut buf)?;

    Ok(buf)
}
