/*
mju-bits - typed bitfield manipulation

Copyright 2019-2020 David Kern <david@mju.io>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! Implementation of typed bitfield access for `u8`, `u16`, `u32` and `u64`[^1].
//!
//!
//! Usage:
//!
//! ```
//! use mju_bits::*;
//!
//! struct RegisterMarker;
//! type Register = Storage<RegisterMarker, u32>;
//! type RegisterAll = BitField<Register, B0, B31>;
//! type RegisterFieldA = BitField<Register, B0, B7>;
//! type RegisterFieldB = BitField<Register, B8, B24>;
//!
//! let mut reg = Register::new(0);
//! reg.set::<RegisterFieldA>(0x56);
//! reg.set::<RegisterFieldB>(0x1234);
//! assert_eq!(reg.get::<RegisterAll>(), 0x00123456);
//! ```
//!
//! The Storage type ensures that a field may only be used with its corresponding storage.
//!
//! For example, this won't compile:
//!
//! ```compilefail
//! use mju_bits::*;
//!
//! struct FooMarker;
//! type Foo = Storage<FooMarker, u8>;
//! type FooField = BitField<Foo, B0, B1>;
//!
//! struct BarMarker;
//! type Bar = Storage<BarMarker, u8>;
//!
//! let bar = Bar::new(0);
//! bar.get::<FooField>();
//! ```
//!
//! [^1]: for platforms where `target_pointer_width="64"`

#![no_std]

pub mod storage;
pub mod bitfield;

pub use storage::*;
pub use bitfield::*;

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test {
        ($name:ident, $type:ident, $lo0:ident, $lo1:ident, $hi0:ident, $hi1:ident, $lo:expr, $hi:expr, $all:expr) => {
            #[test]
            fn $name() {
                struct RegisterMarker;
                type Register = Storage<RegisterMarker, $type>;
                type RegisterAll = BitField<Register, $lo0, $hi1>;
                type RegisterFieldLo = BitField<Register, $lo0, $lo1>;
                type RegisterFieldHi = BitField<Register, $hi0, $hi1>;
        
                let mut reg = Register::new(0);
                assert_eq!(reg.get::<RegisterFieldLo>(), 0);
                assert_eq!(reg.get::<RegisterFieldHi>(), 0);
        
                reg.set::<RegisterFieldLo>($lo);
                assert_eq!(reg.get::<RegisterFieldLo>(), $lo);
                assert_eq!(reg.get::<RegisterFieldHi>(), 0);
        
                reg.set::<RegisterFieldHi>($hi);
                assert_eq!(reg.get::<RegisterFieldLo>(), $lo);
                assert_eq!(reg.get::<RegisterFieldHi>(), $hi);
        
                assert_eq!(reg.get::<RegisterAll>(), $all);
            }        
        };
    }

    test!(
        test_u8, u8,
        B0, B3,
        B4, B7,
        0b1100, 0b1010,
        0b10101100
    );

    test!(
        test_u16, u16,
        B0, B7,
        B8, B15,
        0b11110000, 0b10101010,
        0b10101010_11110000
    );

    test!(
        test_u32, u32,
        B0, B15,
        B16, B31,
        0b11111111_00000000, 0b10101010_10101010,
        0b10101010_10101010_11111111_00000000
    );

    #[cfg(target_pointer_width="64")]
    test!(
        test_u64, u64,
        B0, B31,
        B32, B63,
        0b11111111_11111111_00000000_00000000, 0b10101010_10101010_10101010_10101010,
        0b10101010_10101010_10101010_10101010_11111111_11111111_00000000_00000000
    );
}
