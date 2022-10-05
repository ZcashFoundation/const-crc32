//! A `const fn` crc32 checksum implementation.
//!
//! # Examples
//!
//! ```
//! const BYTES: &[u8] = "The quick brown fox jumps over the lazy dog".as_bytes();
//! const CKSUM: u32 = const_crc32::crc32(BYTES);
//! assert_eq!(CKSUM, 0x414fa339_u32);
//! ```

/// typically crc32 implementations set up a [u32; 256] lookup table. this computes
/// the table on demand for a given "index" `i`
const fn table_fn(i: u32) -> u32 {
    let mut out = i;

    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };
    out = if out & 1 == 1 { 0xedb88320 ^ (out >> 1) } else { out >> 1 };

    out
}

/// A `const fn` crc32 checksum implementation.
///
/// Note: this is a naive implementation that should be expected to have poor performance
/// if used on dynamic data at runtime. Usage should generally be restricted to declaring
/// `const` variables based on `static` or `const` data available at build time.
pub const fn crc32(buf: &[u8]) -> u32 {
    let mut out = !0u32;
    let mut i = 0;
    loop {
        if i >= buf.len() { break }

        out = (out >> 8) ^ table_fn((out & 0xff) ^ (buf[i] as u32));

        i += 1;
    }
    !out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn crc32_compute_table() -> [u32; 256] {
        let mut crc32_table = [0; 256];

        for n in 0..256 {
            crc32_table[n as usize] = (0..8).fold(n as u32, |acc, _| {
                match acc & 1 {
                    1 => 0xedb88320 ^ (acc >> 1),
                    _ => acc >> 1,
                }
            });
        }

        crc32_table
    }

    #[test]
    fn check_table_fn_against_example_code() {
        let table = crc32_compute_table();
        for i in 0..256{
            assert_eq!(table[i], table_fn(i as u32));
        }
    }

    #[test]
    fn simple_test() {
        const BYTES: &[u8] = "The quick brown fox jumps over the lazy dog".as_bytes();
        assert_eq!(crc32(BYTES), 0x414fa339_u32);
        assert_eq!(crc32(BYTES), crc32fast::hash(BYTES));
    }

    #[test]
    fn check_random_inputs_against_crc32_fast() {
        const N_ITER: usize = 100;
        const BUFSIZE: usize = 4096;

        let mut buf = [0u8; BUFSIZE];
        let mut rng = thread_rng();

        for _ in 0..N_ITER {
            rng.fill(&mut buf[..]);
            assert_eq!(crc32(&buf[..]), crc32fast::hash(&buf[..]));
        }
    }
}
