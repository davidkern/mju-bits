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

use core::marker::PhantomData;
use crate::bitfield::BitFieldTrait;

/// Shift and masking functionality required for storage types
pub trait StorageData
{
    // shifts and masks storage and returns field value
    fn from_storage(&self, mask: usize, shift: u32) -> Self;

    // shifts and masks from field to storage
    fn to_storage(&mut self, mask: usize, shift: u32, field: Self);
}

/// Stores a `TData` as a unique type derived from `TMarker`
pub struct Storage<TMarker, TData>
where TData: StorageData
{
    /// The actual data
    data: TData,

    /// Zero-sized field to make the concrete struct type unique
    owner: PhantomData<TMarker>
}

impl<TData, TMarker> Storage<TMarker, TData>
where TData: StorageData
{
    /// Construct a new Storage with a TData::default() value
    pub fn new(initial: TData) -> Self {
        Storage {
            data: initial,
            owner: PhantomData,
        }
    }

    /// Gets the BitField value from storage
    pub fn get<TBitField>(&self) -> TData
    where
        TBitField: BitFieldTrait<Owner=Self>
    {
        self.data.from_storage(TBitField::MASK, TBitField::SHIFT)
    }

    /// Sets the BitField value in storage
    pub fn set<TBitField>(&mut self, value: TData)
    where
        TBitField: BitFieldTrait<Owner=Self>
    {
        self.data.to_storage(TBitField::MASK, TBitField::SHIFT, value);
    }
}

macro_rules! impl_storagedata {
    ($type:ident) => {
        impl StorageData for $type {
            fn from_storage(&self, mask: usize, shift: u32) -> Self {
                (self & mask as $type).wrapping_shr(shift)
            }
        
            fn to_storage(&mut self, mask: usize, shift: u32, field: Self) {
                *self = *self & !mask as $type | (field.wrapping_shl(shift) & mask as $type);
            }
        }        
    };
}

impl_storagedata!(usize);
impl_storagedata!(u8);
impl_storagedata!(u16);
impl_storagedata!(u32);

// Only implemented if usize is big enough (usize is used for masking)
#[cfg(target_pointer_width="64")]
impl_storagedata!(u64);
