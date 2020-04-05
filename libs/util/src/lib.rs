extern crate alloc;

mod parse_ext;
mod str_ext;

pub use str_ext::StrExt;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        mod read_ext;
        pub use read_ext::ReadExt;
    }
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
