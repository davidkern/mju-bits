/*
bitfield.rs - Type defining a range of bits for an owning type

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

use core::marker::PhantomData;

use typenum;
use typenum::{
    Shleft,
    Sub1,
    Minimum,
    Or,
    Unsigned,
    U1,
    Xor,
};

/// Defines a BitField, which defines a range of bits against an owning type
pub struct BitFieldImpl<TOwner, Shift, Mask>(
    PhantomData<TOwner>,
    PhantomData<Shift>,
    PhantomData<Mask>,
);

pub trait BitFieldTrait {
    type Owner;
    const SHIFT: u32;
    const MASK: usize;
}

impl<TOwner, TShift, TMask> BitFieldTrait for BitFieldImpl<TOwner, TShift, TMask>
where
    TShift: Unsigned,
    TMask: Unsigned
{
    type Owner = TOwner;
    const SHIFT: u32 = TShift::U32;
    const MASK: usize = TMask::USIZE;
}

type BitMask<X> = Shleft<U1, X>;
type RightMask<X> = Sub1<BitMask<X>>;
pub type BitField<TOwner, X, Y> = BitFieldImpl<
    TOwner,

    // Shift
    Minimum<X, Y>,

    // Mask
    Or<
        Or<
            Xor<
                RightMask<X>,
                RightMask<Y>
            >,
            BitMask<X>
        >,
        BitMask<Y>
    >
>;

// Re-export typenum::{U0..U127} as B0..B127 for ergonomics with BitField
pub use typenum::U0 as B0;
pub use typenum::U1 as B1;
pub use typenum::U2 as B2;
pub use typenum::U3 as B3;
pub use typenum::U4 as B4;
pub use typenum::U5 as B5;
pub use typenum::U6 as B6;
pub use typenum::U7 as B7;
pub use typenum::U8 as B8;
pub use typenum::U9 as B9;
pub use typenum::U10 as B10;
pub use typenum::U11 as B11;
pub use typenum::U12 as B12;
pub use typenum::U13 as B13;
pub use typenum::U14 as B14;
pub use typenum::U15 as B15;
pub use typenum::U16 as B16;
pub use typenum::U17 as B17;
pub use typenum::U18 as B18;
pub use typenum::U19 as B19;
pub use typenum::U20 as B20;
pub use typenum::U21 as B21;
pub use typenum::U22 as B22;
pub use typenum::U23 as B23;
pub use typenum::U24 as B24;
pub use typenum::U25 as B25;
pub use typenum::U26 as B26;
pub use typenum::U27 as B27;
pub use typenum::U28 as B28;
pub use typenum::U29 as B29;
pub use typenum::U30 as B30;
pub use typenum::U31 as B31;

#[cfg(target_pointer_width="64")]
pub use typenum::U32 as B32;

#[cfg(target_pointer_width="64")]
pub use typenum::U33 as B33;

#[cfg(target_pointer_width="64")]
pub use typenum::U34 as B34;

#[cfg(target_pointer_width="64")]
pub use typenum::U35 as B35;

#[cfg(target_pointer_width="64")]
pub use typenum::U36 as B36;

#[cfg(target_pointer_width="64")]
pub use typenum::U37 as B37;

#[cfg(target_pointer_width="64")]
pub use typenum::U38 as B38;

#[cfg(target_pointer_width="64")]
pub use typenum::U39 as B39;

#[cfg(target_pointer_width="64")]
pub use typenum::U40 as B40;

#[cfg(target_pointer_width="64")]
pub use typenum::U41 as B41;

#[cfg(target_pointer_width="64")]
pub use typenum::U42 as B42;

#[cfg(target_pointer_width="64")]
pub use typenum::U43 as B43;

#[cfg(target_pointer_width="64")]
pub use typenum::U44 as B44;

#[cfg(target_pointer_width="64")]
pub use typenum::U45 as B45;

#[cfg(target_pointer_width="64")]
pub use typenum::U46 as B46;

#[cfg(target_pointer_width="64")]
pub use typenum::U47 as B47;

#[cfg(target_pointer_width="64")]
pub use typenum::U48 as B48;

#[cfg(target_pointer_width="64")]
pub use typenum::U49 as B49;

#[cfg(target_pointer_width="64")]
pub use typenum::U50 as B50;

#[cfg(target_pointer_width="64")]
pub use typenum::U51 as B51;

#[cfg(target_pointer_width="64")]
pub use typenum::U52 as B52;

#[cfg(target_pointer_width="64")]
pub use typenum::U53 as B53;

#[cfg(target_pointer_width="64")]
pub use typenum::U54 as B54;

#[cfg(target_pointer_width="64")]
pub use typenum::U55 as B55;

#[cfg(target_pointer_width="64")]
pub use typenum::U56 as B56;

#[cfg(target_pointer_width="64")]
pub use typenum::U57 as B57;

#[cfg(target_pointer_width="64")]
pub use typenum::U58 as B58;

#[cfg(target_pointer_width="64")]
pub use typenum::U59 as B59;

#[cfg(target_pointer_width="64")]
pub use typenum::U60 as B60;

#[cfg(target_pointer_width="64")]
pub use typenum::U61 as B61;

#[cfg(target_pointer_width="64")]
pub use typenum::U62 as B62;

#[cfg(target_pointer_width="64")]
pub use typenum::U63 as B63;
