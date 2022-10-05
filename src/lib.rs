//! A `const fn` crc32 checksum implementation.
//!
//! # Examples
//!
//! ```
//! const BYTES: &[u8] = "The quick brown fox jumps over the lazy dog".as_bytes();
//! const CKSUM: u32 = const_crc32::crc32(BYTES);
//! assert_eq!(CKSUM, 0x414fa339_u32);
//! ```

#![feature(const_eval_limit)]

#![const_eval_limit = "1000000000"]

/// used to generate up a [u32; 256] lookup table in `crc32`. this computes
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

const fn get_table() -> [u32; 256] {
    let mut table: [u32; 256] = [0u32; 256];
    let mut i = 0;

    while i < 256 {
        table[i] = table_fn(i as u32);
        i += 1;
    }

    table
}

const TABLE: [u32; 256] = get_table();

/// A `const fn` crc32 checksum implementation.
///
/// Note: this is a naive implementation that should be expected to have poor performance
/// if used on dynamic data at runtime. Usage should generally be restricted to declaring
/// `const` variables based on `static` or `const` data available at build time.
pub const fn crc32(buf: &[u8]) -> u32 {
    let mut out = !0u32;
    let mut i = 0usize;
    while i < buf.len() {
        out = (out >> 8) ^ TABLE[((out & 0xff) ^ (buf[i] as u32)) as usize];
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

    #[test]
    fn check_const_eval_limit_not_reached_on_100k_data() {
        const BYTES: &[u8] = &[42u8; 1024 * 100];
        const CKSUM: u32 = crc32(BYTES);
        assert_eq!(CKSUM, crc32fast::hash(&BYTES[..]));
    }

    // #[test]
    // fn check_const_eval_limit_not_reached_on_1mb_data() {
    //     const BYTES: &[u8] = &[42u8; 1024 * 1024];
    //     const CKSUM: u32 = crc32(BYTES);
    //     assert_eq!(CKSUM, crc32fast::hash(&BYTES[..]));
    // }
}
