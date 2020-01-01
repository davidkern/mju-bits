use core::marker::PhantomData;

use typenum::{
    Shleft,
    Sub1,
    Minimum,
    Or,
    Unsigned,
    Xor,
};

// Re-export unsigned ints up to 63 as a convenience
pub use typenum::{
    U0, U1, U2, U3, U4, U5, U6, U7, U8, U9,
    U10, U11, U12, U13, U14, U15, U16, U17, U18, U19,
    U20, U21, U22, U23, U24, U25, U26, U27, U28, U29,
    U30, U31, U32, U33, U34, U35, U36, U37, U38, U39,
    U40, U41, U42, U43, U44, U45, U46, U47, U48, U49,
    U50, U51, U52, U53, U54, U55, U56, U57, U58, U59,
    U60, U61, U62, U63,
};

/// Defines a BitField, which defines a range of bits against an owning type
pub struct BitFieldImpl<TOwner, Shift, Mask>(
    PhantomData<TOwner>,
    PhantomData<Shift>,
    PhantomData<Mask>,
);

pub trait BitFieldTrait {
    type Owner;
    type Shift : Unsigned;
    type Mask : Unsigned;
}

impl<TOwner, Shift, Mask> BitFieldTrait for BitFieldImpl<TOwner, Shift, Mask>
where
    Shift: Unsigned,
    Mask: Unsigned
{
    type Owner = TOwner;
    type Shift = Shift;
    type Mask = Mask;
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
