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

// The trouble seems to come when we further use bitfield-struct
// to shove this into a packed field.

#[derive(TryFromBytes)]
#[bitfield(u8)]
struct MyBitfield {
    // We're bit packing and only care about 6 bits!
    #[bits(6)]
    enum_value: IntBackedEnum,
    #[bits(2)]
    other_field: u8,
}
