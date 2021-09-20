#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use core::{mem, slice};

mod slice_ext;
mod str_ext;

pub use slice_ext::SliceByteOrderExt;
pub use str_ext::StrExt;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod read_ext;
        pub use read_ext::ReadExt;
    }
}

pub fn cast<T>(data: &[u8]) -> &T {
    unsafe { &*(data.as_ptr() as *const T) }
}

pub fn cast_array<T>(data: &[u8]) -> &[T] {
    unsafe { slice::from_raw_parts(data as *const [u8] as *const T, data.len() / mem::size_of::<T>()) }
}

pub fn round_up(num_to_round: usize, multiple: usize) -> usize {
    if multiple == 0 {
        return num_to_round;
    }

    let remainder = num_to_round % multiple;
    if remainder == 0 {
        num_to_round
    } else {
        num_to_round + multiple - remainder
    }
}
