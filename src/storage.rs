use core::marker::PhantomData;
use core::ops::{
    BitAnd,
    Shr
};
use crate::bitfield::BitFieldTrait;

/// Stores a `TData` as a unique type derived from `TMarker`
pub struct Storage<TMarker, TData> {
    /// The actual data
    data: TData,

    /// Zero-sized field to make the concrete struct type unique
    owner: PhantomData<TMarker>
}

trait DataTrait: Default + Copy + BitAnd + Shr { }
impl DataTrait for u8 { }

impl<TData, TMarker> Storage<TMarker, TData>
where TData: DataTrait
{
    /// Construct a new Storage with a TData::default() value
    pub fn new() -> Self {
        Storage {
            data: TData::default(),
            owner: PhantomData,
        }
    }

    pub fn get<BitField>(&self) -> TData
    where
        BitField: BitFieldTrait
    {
        //TData::default()
        self.data & BitFieldTrait::Mask::to_usize()
        // (source & UMask::to_usize()).wrapping_shr(UShift::to_u32())
        // (source & !UMask::to_usize()) | value.wrapping_shl(UShift::to_u32()) & UMask::to_usize()
    }
}
