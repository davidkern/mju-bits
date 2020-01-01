/*
storage.rs - Implements generic storage supporting bitfield access

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

use core::convert::TryFrom;
use core::fmt;
use core::marker::PhantomData;
use crate::bitfield::BitFieldTrait;

/// Shift and masking functionality required for storage types
pub trait StorageData
{
    // shifts and masks storage and returns field value
    fn get_bitfield(&self, mask: Self, shift: u32) -> Self;

    // shifts and masks from field to storage
    fn set_bitfield(&mut self, mask: Self, shift: u32, field: Self);
}

/// Stores a `TData` as a unique type derived from `TMarker`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Storage<TMarker, TData>
where TData: StorageData
{
    /// The actual data
    data: TData,

    /// Zero-sized field to make the concrete struct type unique
    owner: PhantomData<TMarker>
}

macro_rules! impl_storage {
    ($type:ident) => {
        impl<TMarker> Storage<TMarker, $type>
        {
            /// Construct a new Storage
            pub fn new() -> Self {
                Storage {
                    data: $type::default(),
                    owner: PhantomData,
                }
            }
        
            /// Gets the BitField value from storage
            pub fn get<TBitField>(&self) -> $type
            where
                TBitField: BitFieldTrait<Owner=Self>
            {
                self.data.get_bitfield(TBitField::MASK as $type, TBitField::SHIFT)
            }
        
            /// Sets the BitField value in storage
            pub fn set<TBitField>(&mut self, value: $type)
            where
                TBitField: BitFieldTrait<Owner=Self>
            {
                self.data.set_bitfield(TBitField::MASK as $type, TBitField::SHIFT, value);
            }
        }

        impl StorageData for $type {
            fn get_bitfield(&self, mask: Self, shift: u32) -> Self {
                (self & mask as $type).wrapping_shr(shift)
            }
        
            fn set_bitfield(&mut self, mask: Self, shift: u32, field: Self) {
                *self = *self & !mask as $type | (field.wrapping_shl(shift) & mask as $type);
            }
        }

        impl<TMarker> fmt::Display for Storage<TMarker, $type>
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.data)
            }
        }
    };
}

macro_rules! impl_from {
    ($type:ident, $from:ident) => {
        impl<TMarker> From<$from> for Storage<TMarker, $type>
        {
            fn from(value: $from) -> Self {
                Self {
                    data: value.into(),
                    owner: PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_tryfrom {
    ($type:ident, $from:ident) => {
        impl<TMarker> TryFrom<$from> for Storage<TMarker, $type>
        {
            type Error = <$type as TryFrom<$from>>::Error;

            fn try_from(value: $from) -> Result<Self, <$type as TryFrom<$from>>::Error> {
                Ok(Self {
                    data: $type::try_from(value)?,
                    owner: PhantomData,
                })
            }
        }
    };
}

impl_storage!(u8);
impl_from!(u8, u8);
impl_tryfrom!(u8, u16);
impl_tryfrom!(u8, u32);
impl_tryfrom!(u8, u64);
impl_tryfrom!(u8, usize);

impl_storage!(u16);
impl_from!(u16, u8);
impl_from!(u16, u16);
impl_tryfrom!(u16, u32);
impl_tryfrom!(u16, u64);
impl_tryfrom!(u16, usize);

impl_storage!(u32);
impl_from!(u32, u8);
impl_from!(u32, u16);
impl_from!(u32, u32);
impl_tryfrom!(u32, u64);
impl_tryfrom!(u32, usize);

impl_storage!(u64);
impl_from!(u64, u8);
impl_from!(u64, u16);
impl_from!(u64, u32);
impl_from!(u64, u64);
impl_tryfrom!(u64, usize);

impl_storage!(usize);
impl_from!(usize, u8);
impl_from!(usize, u16);
impl_tryfrom!(usize, u32);
impl_tryfrom!(usize, u64);
impl_from!(usize, usize);