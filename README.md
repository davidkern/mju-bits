# mju-bits

`#[no_std]` implementation of typed bitfield access for `u8`, `u16`, `u32`, `u64` and `usize`.


Usage:

```
use mju_bits::*;

struct RegisterMarker;
type Register = Storage<RegisterMarker, u32>;
type RegisterAll = BitField<Register, B0, B31>;
type RegisterFieldA = BitField<Register, B0, B7>;
type RegisterFieldB = BitField<Register, B8, B24>;

let mut reg = Register::new();
reg.set::<RegisterFieldA>(0x56);
reg.set::<RegisterFieldB>(0x1234);
assert_eq!(reg.get::<RegisterAll>(), 0x00123456);
```

The Storage type ensures that a field may only be used with its corresponding storage.

For example, this won't compile:

```compilefail
use mju_bits::*;

struct FooMarker;
type Foo = Storage<FooMarker, u8>;
type FooField = BitField<Foo, B0, B1>;

struct BarMarker;
type Bar = Storage<BarMarker, u8>;

let bar = Bar::new();
bar.get::<FooField>();
```
