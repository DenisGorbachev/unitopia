/// [`Dimension`](HasDimensional::Dimension) is a tuple whose every element is a pair of (QuantityKind, Power) where QuantityKind is a marker struct and Power is a `typenum` integer type.
///
/// ## Requirements
///
/// - Dimension may be a `()` (e.g. mole, radian, steradian units have this dimension)
/// - Dimension must not contain multiple pairs with the same QuantityKind (e.g. bad: `((Length, P1), (Length, N1))`) (the expression must be normalized in powers).
/// - Dimension must not contain any pairs whose Power is zero (e.g. bad: `((Length, Z0))`).
/// - Dimension must be ordered by QuantityKind in a way that is stable across all implementors of this trait (we suggest using SI definition order).
///   - This requirement exists because Rust treats `(A, B)` and `(B, A)` as different types, but the dimension represents a multiplication of quantity kinds in specific powers, and multiplication is commutative, so `(A, B)` and `(B, A)` are the same dimension
pub trait HasDimension {
    type Dimension;
}
