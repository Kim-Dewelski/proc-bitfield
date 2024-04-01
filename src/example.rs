pub mod support;

use crate::*;
use support::*;

bitfield! {
    /// A bitfield showcasing how to specify bit ranges.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitRanges(pub u16): Debug, FromRaw, IntoRaw, DerefRaw {
        // A single field spanning the entire bitfield, using an unbounded range:
        pub whole_bitfield: u16 @ ..,                 // Bits 0 to 15

        // Multi-bit field, specified using an inclusive range:
        pub inclusive_range: u8 @ 0..=3,              // Bits 0 to 3

        // Multi-bit field, specified using an exclusive range:
        pub exclusive_range: u8 @ 4..7,               // Bits 4 to 6

        // Multi-bit field specified using its start bit and length:
        pub start_and_length: u8 @ 7; 5,              // Bits 7 to 11

        // Single-bit field, specified using an inclusive range:
        pub single_bit_inclusive_range: u8 @ 12..=12, // Bit 12

        // Single-bit field, specified using an exclusive range:
        pub single_bit_exclusive_range: u8 @ 13..14,  // Bit 13

        // Single-bit field, specified using its start bit and a length of 1:
        pub single_bit_start_and_length: u8 @ 14; 1,  // Bit 14

        // Single-bit boolean flag, specified using a single bit position.
        // This is equivalent to the single-bit exclusive range, but uses the `Bit` traits instead
        // of `Bits<T>`, and is specific to `bool` (which is conversely not allowed using bit
        // ranges).
        pub flag: bool @ 15,                          // Bit 15
    }
}

bitfield! {
    /// A bitfield showcasing how to specify access restrictions.
    pub struct AccessRestrictions(pub u8): Debug, FromRaw, IntoRaw, DerefRaw {
        // By specifying `read_only` (or `ro`), only `Example::read_only_flag` will be generated (no
        // setters):
        pub read_only_flag: bool [read_only] @ 0,
        // Equivalent:
        // pub read_only_flag: bool [ro] @ 0,

        // By specifying `write_only` (or `wo`), only `Example::set_write_only_flag` and
        // `Example::with_write_only_flag` will be generated (no getters):
        pub write_only_flag: bool [write_only] @ 1,
        // Equivalent:
        // pub read_only_flag: bool [wo] @ 0,

        // Both getters and setters will be generated without any explicit access restrictions:
        // `Example::read_write_flag`, `Example::set_read_write_flag` and
        // `Example::with_read_write_flag` will all be generated.
        pub read_write_flag: bool @ 2,
    }
}

bitfield! {
    /// A bitfield showcasing various kinds of field type conversions.
    pub struct FieldTypeConversions(pub u16): Debug, FromRaw, IntoRaw, DerefRaw {
        // Infallible conversions

        // Will:
        // - Return a `U8WithParity` on reads, calling `<U8WithParity as From<u8>>::from`
        // - Take a `u8` for writes
        pub read_as_u8_with_parity: u8 [get U8WithParity] @ 0..=3,

        // Will:
        // - Return a `u8` on reads
        // - Take a `U8WithParity` for writes, calling `<U8WithParity as Into<u8>>::into`
        pub write_as_u8_with_parity: u8 [set U8WithParity] @ 4..=7,

        // Will:
        // - Return a `U8WithParity` on reads, calling `<U8WithParity as From<u8>>::from`
        // - Take a `U8WithParity` for writes, calling `<U8WithParity as Into<u8>>::into`
        pub as_u8_with_parity: u8 [U8WithParity] @ 8..=11,
        // Equivalent to:
        // pub u8_with_parity: u8 [get U8WithParity, set U8WithParity] @ 8..=11,


        // Fallible conversions

        // Will:
        // - Return a `Result<NonZeroU8, TryFromIntError>` on reads, calling
        //   `<NonZeroU8 as TryFrom<u8>>::try_from`
        // - Take a `u8` for writes
        pub try_read_as_non_zero_u8: u8 [try_get NonZeroU8] @ 0..=3,

        // Will:
        // - Return a `u8` on reads
        // - Take a `U16` for writes, returning `Result<(), TryFromIntError>` and calling
        //   `<U16 as TryInto<u8>>::try_into`
        pub try_write_as_u16: u8 [try_set U16] @ 4..=7,

        // Will:
        // - Return a `Result<SpuriouslyFailingU8, ()>` on reads, calling
        //   `<SpuriouslyFailingU8 as TryFrom<u8>>::try_from`
        // - Take a `SpuriouslyFailingU8` for writes, returning `Result<(), ()>` and calling
        //   `<SpuriouslyFailingU8 as TryInto<u8>>::try_into`
        pub try_both_as_spuriously_failing: u8 [try_both SpuriouslyFailingU8] @ 8..=11,
        // Equivalent to:
        // pub try_both_as_spuriously_failing: u8
        //  [try_get SpuriouslyFailingU8, try_set SpuriouslyFailingU8] @ 8..=11,

        // Will:
        // - Return a `Result<NonZeroU8, TryFromIntError>` on reads, calling
        //   `<NonZeroU8 as TryFrom<u8>>::try_from`
        // - Take a `NonZeroU8` for writes, calling `<NonZeroU8 as Into<u8>>::into`
        pub try_as_non_zero_u8: u8 [try NonZeroU8] @ 12..=15,
        // Equivalent to:
        // pub try_as_non_zero_u8: u8 [try_get NonZeroU8, set NonZeroU8] @ 12..=15,


        // Unwrapping conversions

        // Will:
        // - Return a `NonZeroU8` on reads, calling `<NonZeroU8 as TryFrom<u8>>::try_from` and
        //   unwrapping the result
        // - Take a `u8` for writes
        pub unwrap_read_as_non_zero_u8: u8 [unwrap_get NonZeroU8] @ 0..=3,

        // Will:
        // - Return a `u8` on reads
        // - Take a `U16` for writes, returning `()`, calling `<U16 as TryInto<u8>>::try_into` and
        //   unwrapping the result
        pub unwrap_write_as_u16: u8 [unwrap_set U16] @ 4..=7,

        // Will:
        // - Return a `SpuriouslyFailingU8` on reads, calling
        //   `<SpuriouslyFailingU8 as TryFrom<u8>>::try_from` and unwrapping the result
        // - Take a `SpuriouslyFailingU8` for writes, returning `()`, calling
        //   `<SpuriouslyFailingU8 as TryInto<u8>>::try_into` and unwrapping the result
        pub unwrap_both_as_spuriously_failing: u8 [unwrap_both SpuriouslyFailingU8] @ 8..=11,
        // Equivalent to:
        // pub try_both_as_spuriously_failing: u8
        //  [try_get SpuriouslyFailingU8, try_set SpuriouslyFailingU8] @ 8..=11,

        // Will:
        // - Return a `NonZeroU8` on reads, calling `<NonZeroU8 as TryFrom<u8>>::try_from` and
        //   unwrapping the result
        // - Take a `NonZeroU8` for writes, calling `<NonZeroU8 as Into<u8>>::into`
        pub unwrap_as_non_zero_u8: u8 [unwrap NonZeroU8] @ 12..=15,
        // Equivalent to:
        // pub unwrap_as_non_zero_u8: u8 [unwrap_get NonZeroU8, set NonZeroU8] @ 12..=15,


        // Unsafe/unchecked conversions

        // Will:
        // - Return a `NonZeroU8` on reads, marking them as `unsafe` and calling
        //   `<NonZeroU8 as UnsafeFrom<u8>>::unsafe_from`
        // - Take a `u8` for writes
        pub unsafe_read_as_non_zero_u8: u8 [unsafe_get NonZeroU8] @ 0..=3,

        // Will:
        // - Return a `u8` on reads
        // - Take a `U16` for writes, marking them as `unsafe` and calling
        //   `<U16 as UnsafeInto<u8>>::unsafe_into`
        pub unsafe_write_as_u16: u8 [unsafe_set U16] @ 4..=7,

        // Will:
        // - Return a `SpuriouslyFailingU8` on reads, marking them as `unsafe` and calling
        //   `<SpuriouslyFailingU8 as UnsafeFrom<u8>>::unsafe_from`
        // - Take a `SpuriouslyFailingU8` for writes, marking them as `unsafe` and calling
        //   `<SpuriouslyFailingU8 as UnsafeInto<u8>>::unsafe_into`
        pub unsafe_as_spuriously_failing: u8 [unsafe_both SpuriouslyFailingU8] @ 8..=11,
        // Equivalent to:
        // pub unsafe_as_spuriously_failing: u8
        //  [unsafe_get SpuriouslyFailingU8, unsafe_set SpuriouslyFailingU8] @ 8..=11,

        // Will:
        // - Return a `NonZeroU8` on reads, marking them as `unsafe` and calling
        //   `<NonZeroU8 as UnsafeFrom<u8>>::unsafe_from`
        // - Take a `NonZeroU8` for writes, calling `<NonZeroU8 as Into<u8>>::into`
        pub unsafe_as_non_zero_u8: u8 [unsafe NonZeroU8] @ 12..=15,
        // Equivalent to:
        // pub unsafe_as_non_zero_u8: u8 [unsafe_get NonZeroU8, set NonZeroU8] @ 12..=15,
    }
}

bitfield! {
    /// A bitfield showcasing how to specify bit ranges.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct FieldAccessorFunctions(pub u8): Debug, FromRaw, IntoRaw, DerefRaw {
        // A single field without any accessor function.
        pub no_accessor: u8 @ ..,

        // A single field with a get accessor function.
        // The function takes a `u16` as input, and returns a `u16`.
        pub get_accessor: u8 [get_fn handle_on_get] @ ..,

        // A single field with a set accessor function.
        // The function takes a `u16` as input, and returns a `u16`.
        pub set_accessor: u8 [set_fn handle_on_set] @ ..,

        // A single field with a get and set accessor functions.
        // The function takes a `u16` as input, and returns a `u16`.
        pub get_set_accessors: u8 [get_fn handle_on_get, set_fn handle_on_set] @ ..,

        // A single field with a get accessor function.
        // The function taken a `NonZeroU8` as input, and returns a `NonZeroU8`.
        pub get_accessor_with_ty: u8 [try NonZeroU8, get_fn handle_on_get] @ ..
    }
}

/// An enum showcasing the `ConvRaw` derive.
#[derive(ConvRaw)]
pub enum ConvRawExample {
    A,
    B = 2,
    C,
    D = -1,
    E = 1,
    F = -128,
    G = 128,
}

#[cfg(feature = "nightly")]
#[cfg_attr(all(doc, feature = "nightly"), doc(cfg(feature = "nightly")))]
/// A type showcasing the `UnwrapBits` derive.
#[derive(UnwrapBits)]
pub struct UnwrapBitsExample(NonZeroU8);
