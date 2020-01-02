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

/// Marker type representing an owned bitfield.  `TOwner` is the owning type of the
/// bitfield.  `Shift` and `Mask` are `typenum::Unsigned` types representing the shift and mask
/// values used to access the bitfield.  These are both converted to constants in the `BitFieldTrait`
/// implemented for this struct.
///
/// While this type can be used directly, it is more convenient to use the `BitField` type
/// alias to define a bit range rather than the lower-level shift and mask.
pub struct BitFieldImpl<TOwner, Shift, Mask>(
    PhantomData<TOwner>,
    PhantomData<Shift>,
    PhantomData<Mask>,
);

/// Tracks the `Owner`, shift and mask used by the `Storage::*` methods to provide access
/// to an owned bit field.
pub trait BitFieldTrait {
    /// The owner, typically a `Storage`, bound to this bit field
    type Owner;

    /// The number of bits the field is shifted in the owner
    const SHIFT: u32;

    /// The bit mask used to to select or clear the field bits in the owner
    const MASK: u64;
}

/// Implementation for `BitFieldImpl` explicitly converts the typenum shift and mask calculations
/// into constants. Forcing the conversion here prevents infinite recursion by the compiler when
/// these values are used by `Storage` to provide access to the field.
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

/// Define a BitField bound to `TOWner` (typically a `Storage`).  The bit range of
/// the field includes bits from `X` to `Y` inclusively.  The ordering of bit positions
/// does not matter: `BitField<Owner, B0, B7>` defines the same field as `BitField<Owner, B7, B0>`
/// although technically the two are distinct types.
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
