# Limitations Encountered

- `num_traits::One` / `num_traits::ConstOne` cannot be implemented for `Measure` or strict wrapper units because those traits require `Mul<Self, Output = Self>`, while unit-safe multiplication must return a distinct monomial unit type.
- `num_traits::CheckedMul`, `CheckedDiv`, `SaturatingMul`, `WrappingMul`, and `OverflowingMul` for unit-safe types are not implemented for the same reason: they require `Output = Self`, which conflicts with unit-safe monomial outputs.
- Scalar `Mul`/`Div` for strict wrapper units require wrapping scalars in `Scalar<T>` to avoid coherence overlap; strict wrapper prefixes still omit scalar `Mul`/`Div` because they overlap with the unit-unit impls in downstream crates. Raw `T` scalar `Mul`/`Div` are not implemented.
- A fully generic `Borrow<U>` implementation for strict wrapper structs conflicts with the blanket `Borrow<T> for T`; the wrapper only implements `Borrow<T>`.
- `wincode` derive macros for generic wrapper types did not infer bounds correctly; manual `SchemaWrite`/`SchemaRead` impls with explicit bounds are required.
