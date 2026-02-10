# Marketing

An implementation of system-of-units-of-measurement with the following properties:

* Works on stable Rust (no nightly features)
* `#![no_std]`
* `#![forbid(unsafe_code)]`
* Memory-optimal (every value takes exactly the same amount of memory as the underlying storage type)
* Supports non-physical units:
  * Information: bit, byte, kilobyte, etc.
  * Angle: degree, radian, gradian, etc.
  * Currency: U.S. Dollar, Bitcoin, [Rai](https://en.wikipedia.org/wiki/Rai_stones), etc.
* Supports alternative unit systems:
  * Imperial system: foot, pound, gallon.
  * Volume: US gallon, imperial gallon, etc.

## UOM

`uom` has an incorrect design. Two problems:

* Can't express fractional units (e.g. milliseconds) with integers (requires using floating points (losing precision) or using alternative numeric types (e.g. `rust_decimal::Decimal`) (wasting memory and processor cycles))

Notes:

* It can express non-SI quantities (allows to define your own system)
