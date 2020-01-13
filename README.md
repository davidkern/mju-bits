# mju-bits

`#[no_std]` implementation of typed bitfield access for `u8`, `u16`, `u32`, `u64` and `usize`.


## Usage

```
use mju_bits::*;

struct RegisterMarker;
type Register = Storage<RegisterMarker, RW, u32>;
type RegisterAll = BitField<Register, U0, U31>;
type RegisterFieldA = BitField<Register, U0, U7>;
type RegisterFieldB = BitField<Register, U8, U24>;

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
type Foo = Storage<FooMarker, RW, u8>;
type FooField = BitField<Foo, U0, U1>;

struct BarMarker;
type Bar = Storage<BarMarker, RW, u8>;

let bar = Bar::new();

// Compiler error: FooField can not be used with Bar storage.
bar.get::<FooField>();
```

## Roadmap

 * BitFields spanning multiple primitive types
 * Iterable sequences of BitFields, supporting functional protocol definition
 * Implementation compatible with generic consts on nightly
 * Improve types so larger bitfields can be supported - 128-bit fields break typenum
 * Others?


## Changelog

 * 0.3.0 - Removal of `B0`..`B63` aliases to avoid confusion with `typenum::B0` and `typenum::B1`
 * 0.2.0 - Documentation and cleanup
 * 0.1.0 - Initial release
