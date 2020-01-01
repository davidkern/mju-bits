/*
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

#![no_std]

pub mod storage;
pub mod bitfield;

pub use storage::*;
pub use bitfield::*;

// Usage

struct RegisterMarker;
type Register = Storage<RegisterMarker, u8>;
type RegisterFieldA = BitField<Register, U0, U3>;

fn usage() {
    let mut reg = Register::new();
    //reg.get::<RegisterFieldA>();
}