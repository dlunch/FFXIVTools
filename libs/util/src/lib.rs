mod parse_ext;
mod read_ext;
mod str_ext;

pub use read_ext::ReadExt;
pub use str_ext::StrExt;

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
