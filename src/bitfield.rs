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

//! Type definition of an owned bitfield with compile-time calculation of mask and shift

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

pub struct BitFieldImpl<TOwner, Shift, Mask>(
    PhantomData<TOwner>,
    PhantomData<Shift>,
    PhantomData<Mask>,
);

pub trait BitFieldTrait {
    type Owner;
    const SHIFT: u32;
    const MASK: u64;
}

impl<TOwner, TShift, TMask> BitFieldTrait for BitFieldImpl<TOwner, TShift, TMask>
where
    TShift: Unsigned,
    TMask: Unsigned
{
    type Owner = TOwner;
    const SHIFT: u32 = TShift::U32;
    const MASK: u64 = TMask::U64;
}

type BitMask<X> = Shleft<U1, X>;
type RightMask<X> = Sub1<BitMask<X>>;

/// Define a BitField including bits from `X` to `Y`, inclusively, for `TOwner`
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
