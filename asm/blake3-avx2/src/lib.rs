use std::arch::global_asm;

#[cfg(target_family = "unix")]
global_asm!(include_str!("blake3_avx2_x86-64_unix_prepro.S"));

#[cfg(target_family = "windows")]
global_asm!(include_str!("blake3_avx2_x86-64_windows_gnu.S"));

pub mod ffi {
    extern "C" {
        pub fn blake3_hash_many_avx2(
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            key: *const u32,
            counter: u64,
            increment_counter: bool,
            flags: u8,
            flags_start: u8,
            flags_end: u8,
            out: *mut u8,
        );
    }
}
