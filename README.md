# const-crc32

A `const fn` crc32 checksum implementation.

## Examples

```rust
const BYTES: &[u8] = "The quick brown fox jumps over the lazy dog".as_bytes();
const CKSUM: u32 = const_crc32::crc32(BYTES);
assert_eq!(CKSUM, 0x414fa339_u32);
```

## Usage

This is a naive implementation that should be expected to have poor performance
if used on dynamic data at runtime. Usage should generally be restricted to declaring
`const` variables based on `static` or `const` data available at build time.

## `#[const_eval_limit]`

This crate sets `#[const_eval_limit]` to 1,000,000,000 to avoid hitting the limit when
executing the `const fn`, which requires `#![feature(const_eval_limit)]`.

Compile time for `const` data around 100k is less than 1s.