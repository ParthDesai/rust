// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The `bitflags!` macro generates a `struct` that holds a set of C-style
//! bitmask flags. It is useful for creating typesafe wrappers for C APIs.
//!
//! The flags should only be defined for integer types, otherwise unexpected
//! type errors may occur at compile time.
//!
//! # Example
//!
//! ~~~rust
//! bitflags!(Flags: u32 {
//!     FlagA       = 0x00000001,
//!     FlagB       = 0x00000010,
//!     FlagC       = 0x00000100,
//!     FlagABC     = FlagA.bits
//!                 | FlagB.bits
//!                 | FlagC.bits
//! })
//!
//! fn main() {
//!     let e1 = FlagA | FlagC;
//!     let e2 = FlagB | FlagC;
//!     assert!((e1 | e2) == FlagABC);   // union
//!     assert!((e1 & e2) == FlagC);     // intersection
//!     assert!((e1 - e2) == FlagA);     // set difference
//! }
//! ~~~
//!
//! The generated `struct`s can also be extended with type and trait implementations:
//!
//! ~~~rust
//! use std::fmt;
//!
//! bitflags!(Flags: u32 {
//!     FlagA   = 0x00000001,
//!     FlagB   = 0x00000010
//! })
//!
//! impl Flags {
//!     pub fn clear(&mut self) {
//!         self.bits = 0;  // The `bits` field can be accessed from within the
//!                         // same module where the `bitflags!` macro was invoked.
//!     }
//! }
//!
//! impl fmt::Show for Flags {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f.buf, "hi!")
//!     }
//! }
//!
//! fn main() {
//!     let mut flags = FlagA | FlagB;
//!     flags.clear();
//!     assert!(flags.is_empty());
//!     assert_eq!(format!("{}", flags).as_slice(), "hi!");
//! }
//! ~~~
//!
//! # Derived traits
//!
//! The `Eq`, `TotalEq`, and `Clone` traits are automatically derived for the
//! `struct` using the `deriving` attribute.
//!
//! # Operators
//!
//! The following operator traits are implemented for the generated `struct`:
//!
//! - `BitOr`: union
//! - `BitAnd`: intersection
//! - `Sub`: set difference
//!
//! # Methods
//!
//! The following methods are defined for the generated `struct`:
//!
//! - `empty`: an empty set of flags
//! - `bits`: the raw value of the flags currently stored
//! - `is_empty`: `true` if no flags are currently stored
//! - `intersects`: `true` if there are flags common to both `self` and `other`
//! - `contains`: `true` all of the flags in `other` are contained within `self`
//! - `insert`: inserts the specified flags in-place
//! - `remove`: removes the specified flags in-place

#[macro_export]
macro_rules! bitflags(
    ($BitFlags:ident: $T:ty {
        $($Flag:ident = $value:expr),+
    }) => (
        #[deriving(Eq, TotalEq, Clone)]
        pub struct $BitFlags {
            bits: $T,
        }

        $(pub static $Flag: $BitFlags = $BitFlags { bits: $value };)+

        impl $BitFlags {
            /// Returns an empty set of flags.
            pub fn empty() -> $BitFlags {
                $BitFlags { bits: 0 }
            }

            /// Returns the raw value of the flags currently stored.
            pub fn bits(&self) -> $T {
                self.bits
            }

            /// Returns `true` if no flags are currently stored.
            pub fn is_empty(&self) -> bool {
                *self == $BitFlags::empty()
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            pub fn intersects(&self, other: $BitFlags) -> bool {
                !(self & other).is_empty()
            }

            /// Returns `true` all of the flags in `other` are contained within `self`.
            pub fn contains(&self, other: $BitFlags) -> bool {
                (self & other) == other
            }

            /// Inserts the specified flags in-place.
            pub fn insert(&mut self, other: $BitFlags) {
                self.bits |= other.bits;
            }

            /// Removes the specified flags in-place.
            pub fn remove(&mut self, other: $BitFlags) {
                self.bits &= !other.bits;
            }
        }

        impl BitOr<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the union of the two sets of flags.
            #[inline]
            fn bitor(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits | other.bits }
            }
        }

        impl BitAnd<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the intersection between the two sets of flags.
            #[inline]
            fn bitand(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & other.bits }
            }
        }

        impl Sub<$BitFlags, $BitFlags> for $BitFlags {
            /// Returns the set difference of the two sets of flags.
            #[inline]
            fn sub(&self, other: &$BitFlags) -> $BitFlags {
                $BitFlags { bits: self.bits & !other.bits }
            }
        }
    )
)

#[cfg(test)]
mod tests {
    use ops::{BitOr, BitAnd, Sub};

    bitflags!(Flags: u32 {
        FlagA       = 0x00000001,
        FlagB       = 0x00000010,
        FlagC       = 0x00000100,
        FlagABC     = FlagA.bits
                    | FlagB.bits
                    | FlagC.bits
    })

    #[test]
    fn test_bits(){
        assert_eq!(Flags::empty().bits(), 0x00000000);
        assert_eq!(FlagA.bits(), 0x00000001);
        assert_eq!(FlagABC.bits(), 0x00000111);
    }

    #[test]
    fn test_is_empty(){
        assert!(Flags::empty().is_empty());
        assert!(!FlagA.is_empty());
        assert!(!FlagABC.is_empty());
    }

    #[test]
    fn test_two_empties_do_not_intersect() {
        let e1 = Flags::empty();
        let e2 = Flags::empty();
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_empty_does_not_intersect_with_full() {
        let e1 = Flags::empty();
        let e2 = FlagABC;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_disjoint_intersects() {
        let e1 = FlagA;
        let e2 = FlagB;
        assert!(!e1.intersects(e2));
    }

    #[test]
    fn test_overlapping_intersects() {
        let e1 = FlagA;
        let e2 = FlagA | FlagB;
        assert!(e1.intersects(e2));
    }

    #[test]
    fn test_contains() {
        let e1 = FlagA;
        let e2 = FlagA | FlagB;
        assert!(!e1.contains(e2));
        assert!(e2.contains(e1));
        assert!(FlagABC.contains(e2));
    }

    #[test]
    fn test_insert(){
        let mut e1 = FlagA;
        let e2 = FlagA | FlagB;
        e1.insert(e2);
        assert!(e1 == e2);
    }

    #[test]
    fn test_remove(){
        let mut e1 = FlagA | FlagB;
        let e2 = FlagA | FlagC;
        e1.remove(e2);
        assert!(e1 == FlagB);
    }

    #[test]
    fn test_operators() {
        let e1 = FlagA | FlagC;
        let e2 = FlagB | FlagC;
        assert!((e1 | e2) == FlagABC);   // union
        assert!((e1 & e2) == FlagC);     // intersection
        assert!((e1 - e2) == FlagA);     // set difference
    }
}
