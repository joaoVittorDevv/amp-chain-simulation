// ABI correctness: f_size_t must be 8 bytes on 64-bit targets (CROSS-05).
// Before this fix, `typedef unsigned long f_size_t` was 4 bytes on Windows (LLP64)
// but 8 bytes on Linux/macOS (LP64) — a silent mismatch.
// With `typedef size_t f_size_t`, it is always pointer-sized on every platform.

#[test]
#[cfg(target_pointer_width = "64")]
fn f_size_t_is_8_bytes() {
    assert_eq!(
        std::mem::size_of::<distortion::bridge::faust::f_size_t>(),
        8,
        "f_size_t must be 8 bytes on 64-bit targets"
    );
    assert_eq!(
        std::mem::size_of::<distortion::bridge::mlc_zero_v::f_size_t>(),
        8,
        "mlc f_size_t must be 8 bytes on 64-bit targets"
    );
}
