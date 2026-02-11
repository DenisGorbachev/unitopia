/// See the definition of "Dimensional (physics)" in .agents/project.md
///
/// [`Dimensional`](HasDimensional::Dimensional) is a tuple whose every element is a pair of (QuantityKind, Power) where QuantityKind is a marker struct and Power is a `typenum` integer type.
///
/// ## Requirements
///
/// - Dimensional may be a `()` (e.g. mole unit has this dimension)
/// - Dimensional may contain multiple pairs with the same QuantityKind (e.g. good: `((Length, P1), (Length, N1))`).
/// - Dimensional must not contain any pairs whose Power is zero (e.g. bad: `((Length, Z0))`).
/// - Dimensional must be ordered by QuantityKind in a way that is stable across all implementors of this trait (we suggest using SI definition order).
///   - This requirement exists because Rust treats `(A, B)` and `(B, A)` as different types, but the dimensional represents a multiplication of quantity kinds in specific powers, and multiplication is commutative, so `(A, B)` and `(B, A)` are the same dimensional
pub trait HasDimensional {
    type Dimensional;
}
