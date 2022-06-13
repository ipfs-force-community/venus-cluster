use byte_unit::Byte;

pub mod cpu;
pub mod disk;
pub mod gpu;
pub mod mem;

/// Format the bytes to string.
pub fn byte_string(bytes: u64, fractional_digits: usize) -> String {
    Byte::from(bytes)
        .get_appropriate_unit(true)
        .format(fractional_digits)
}