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

#![doc(html_root_url = "https://docs.rs/mju-bits/0.3.0")]

//! `#[no_std]` implementation of typed bitfield access for `u8`, `u16`, `u32`, `u64` and `usize`.
//!
//!
//! Usage:
//!
//! ```
//! use typenum::*;
//! use mju_bits::*;
//!
//! struct RegisterMarker;
//! type Register = Storage<RegisterMarker, u32>;
//! type RegisterAll = BitField<Register, U0, U31>;
//! type RegisterFieldA = BitField<Register, U0, U7>;
//! type RegisterFieldB = BitField<Register, U8, U24>;
//!
//! let mut reg = Register::new();
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
//! type FooField = BitField<Foo, U0, U1>;
//!
//! struct BarMarker;
//! type Bar = Storage<BarMarker, u8>;
//!
//! let bar = Bar::new();
//! bar.get::<FooField>();
//! ```

#![no_std]

pub mod access;
pub mod bitfield;
pub mod storage;

pub use bitfield::*;
pub use storage::*;

#[cfg(test)]
mod test {
    use super::*;
    use typenum::{
        U0, U3, U4, U7, U8, U15, U16, U31, U32, U63,
    };

    macro_rules! test {
        ($name:ident, $type:ident, $lo0:ident, $lo1:ident, $hi0:ident, $hi1:ident, $lo:expr, $hi:expr, $all:expr) => {
            #[test]
            fn $name() {
                struct RegisterMarker;
                type Register = Storage<RegisterMarker, $type>;
                type RegisterAll = BitField<Register, $lo0, $hi1>;
                type RegisterFieldLo = BitField<Register, $lo0, $lo1>;
                type RegisterFieldHi = BitField<Register, $hi0, $hi1>;
        
                let mut reg = Register::new();
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
        U0, U3,
        U4, U7,
        0b1100, 0b1010,
        0b10101100
    );

    test!(
        test_u16, u16,
        U0, U7,
        U8, U15,
        0b11110000, 0b10101010,
        0b10101010_11110000
    );

    test!(
        test_u32, u32,
        U0, U15,
        U16, U31,
        0b11111111_00000000, 0b10101010_10101010,
        0b10101010_10101010_11111111_00000000
    );

    #[cfg(target_pointer_width="64")]
    test!(
        test_u64, u64,
        U0, U31,
        U32, U63,
        0b11111111_11111111_00000000_00000000, 0b10101010_10101010_10101010_10101010,
        0b10101010_10101010_10101010_10101010_11111111_11111111_00000000_00000000
    );
}
