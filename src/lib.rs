use bitfield_struct::bitfield;
use zerocopy_derive::TryFromBytes;

// Enum that's using an integer representation,
// but does not cover the full range.
// Thus, it must be TryFromBytes.

#[derive(TryFromBytes, Debug)]
#[repr(u8)]
pub enum IntBackedEnum {
    VariantA = 0,
    VariantB = 1,
    VariantC = 26,
}

impl IntBackedEnum {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::VariantA,
            1 => Self::VariantB,
            26 => Self::VariantC,
            _ => panic!("As far as I can tell, this crate doesn't support failable ops.")
        }
    }
}

// I thought that there was an issue idiosyncratic to repr(<some int>)
// but that turned out to not be the case...
// Only the ordering of the macros matters.
// Flip them to get it compiling.

#[derive(TryFromBytes)]
#[bitfield(u8)]
struct BitfieldWithEnum {
    // We're bit packing and only care about 6 bits!
    #[bits(6)]
    enum_value: IntBackedEnum,
    #[bits(2)]
    other_field: u8,
}

// And here's a second example of breakage.
// If you derive TryFromBytes before the bitfield macro,
// compilation fails.
// It works when you flip the order (bitfield macro first).

#[derive(TryFromBytes)]
#[bitfield(u8)]
struct BitfieldWithInteger {
    // We're bit packing and only care about 6 bits!
    #[bits(6)]
    enum_value: u8,
    #[bits(2)]
    other_field: u8,
}