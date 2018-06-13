#![allow(dead_code)]

/// Write a number as an hexadecimal formatter bytestring
///
/// Panics if the buffer is shorter than `size_of::<usize>()`.
pub fn write_hex(number: usize, buf: &mut [u8]) {
    let length = ::core::mem::size_of::<usize>() / 4;
    for i in 0..length {
        buf[buf.len() - (i + 2)] = match (number & 0xF) as u8 {
            x @ 0x0u8...0x9u8 => x as u8 + b'0',
            y @ 0xAu8...0xFu8 => y as u8 + b'A',
            _ => unreachable!(),
        };
    }
}


