# Marketing

* An implementation of system-of-units-of-measurement with the following properties:
  * Works on stable Rust (no nightly features)
  * `#![no_std]`
  * `#![forbid(unsafe_code)]`
  * Memory-optimal (every value takes exactly the same amount of memory as the underlying storage type)
  * Supports alternative units (feet, gallons)
