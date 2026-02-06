#![no_std]

/// Defines a vanilla marker struct.
#[macro_export]
macro_rules! define_marker_struct {
    ($name:ident) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        pub struct $name;
    };
}

/// Defines multiple vanilla marker structs.
#[macro_export]
macro_rules! define_marker_structs {
    ($($name:ident),+ $(,)?) => {
        $(
            $crate::define_marker_struct!($name);
        )+
    };
}

/// Defines a strict open wrapper struct and implements standard wrapper traits.
#[macro_export]
macro_rules! define_strict_wrapper_struct {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        #[repr(transparent)]
        pub struct $name<T> {
            pub inner: T,
        }

        impl<T> $name<T> {
            pub const fn new(inner: T) -> Self {
                Self {
                    inner
                }
            }
        }

        impl<T> core::ops::Deref for $name<T>
        where
            T: core::ops::Deref,
        {
            type Target = <T as core::ops::Deref>::Target;

            fn deref(&self) -> &Self::Target {
                <T as core::ops::Deref>::deref(&self.inner)
            }
        }

        impl<T> core::ops::DerefMut for $name<T>
        where
            T: core::ops::DerefMut,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                <T as core::ops::DerefMut>::deref_mut(&mut self.inner)
            }
        }

        impl<T, U> core::convert::AsRef<U> for $name<T>
        where
            T: core::convert::AsRef<U>,
        {
            fn as_ref(&self) -> &U {
                <T as core::convert::AsRef<U>>::as_ref(&self.inner)
            }
        }

        impl<T> core::borrow::Borrow<T> for $name<T> {
            fn borrow(&self) -> &T {
                &self.inner
            }
        }

        impl<T> From<T> for $name<T> {
            fn from(inner: T) -> Self {
                Self {
                    inner,
                }
            }
        }
    };
}

/// Implements `ConstZero` and `Zero` for a strict wrapper struct with `inner`.
#[macro_export]
macro_rules! impl_wrapper_identity_traits {
    ($name:ident) => {
        impl<T> num_traits::ConstZero for $name<T>
        where
            T: num_traits::ConstZero,
        {
            const ZERO: Self = Self {
                inner: T::ZERO,
            };
        }

        impl<T> num_traits::Zero for $name<T>
        where
            T: num_traits::Zero,
        {
            fn zero() -> Self {
                Self {
                    inner: T::zero(),
                }
            }

            fn is_zero(&self) -> bool {
                self.inner.is_zero()
            }
        }
    };
}

/// Implements add/sub-related traits for a strict wrapper struct with `inner`.
#[macro_export]
macro_rules! impl_wrapper_add_sub_traits {
    ($name:ident) => {
        impl<T> core::ops::Add for $name<T>
        where
            T: core::ops::Add<T, Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    inner: core::ops::Add::add(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::Sub for $name<T>
        where
            T: core::ops::Sub<T, Output = T>,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    inner: core::ops::Sub::sub(self.inner, rhs.inner),
                }
            }
        }

        impl<T> core::ops::AddAssign for $name<T>
        where
            T: core::ops::AddAssign<T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                core::ops::AddAssign::add_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> core::ops::SubAssign for $name<T>
        where
            T: core::ops::SubAssign<T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                core::ops::SubAssign::sub_assign(&mut self.inner, rhs.inner);
            }
        }

        impl<T> num_traits::CheckedAdd for $name<T>
        where
            T: num_traits::CheckedAdd,
        {
            fn checked_add(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedAdd::checked_add(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::CheckedSub for $name<T>
        where
            T: num_traits::CheckedSub,
        {
            fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                num_traits::CheckedSub::checked_sub(&self.inner, &rhs.inner).map(|value| Self {
                    inner: value,
                })
            }
        }

        impl<T> num_traits::SaturatingAdd for $name<T>
        where
            T: num_traits::SaturatingAdd,
        {
            fn saturating_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingAdd::saturating_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::SaturatingSub for $name<T>
        where
            T: num_traits::SaturatingSub,
        {
            fn saturating_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::SaturatingSub::saturating_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingAdd for $name<T>
        where
            T: num_traits::WrappingAdd,
        {
            fn wrapping_add(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingAdd::wrapping_add(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::WrappingSub for $name<T>
        where
            T: num_traits::WrappingSub,
        {
            fn wrapping_sub(&self, rhs: &Self) -> Self {
                Self {
                    inner: num_traits::WrappingSub::wrapping_sub(&self.inner, &rhs.inner),
                }
            }
        }

        impl<T> num_traits::ops::overflowing::OverflowingAdd for $name<T>
        where
            T: num_traits::ops::overflowing::OverflowingAdd,
        {
            fn overflowing_add(&self, rhs: &Self) -> (Self, bool) {
                let (value, overflowed) = num_traits::ops::overflowing::OverflowingAdd::overflowing_add(&self.inner, &rhs.inner);
                (
                    Self {
                        inner: value,
                    },
                    overflowed,
                )
            }
        }

        impl<T> num_traits::ops::overflowing::OverflowingSub for $name<T>
        where
            T: num_traits::ops::overflowing::OverflowingSub,
        {
            fn overflowing_sub(&self, rhs: &Self) -> (Self, bool) {
                let (value, overflowed) = num_traits::ops::overflowing::OverflowingSub::overflowing_sub(&self.inner, &rhs.inner);
                (
                    Self {
                        inner: value,
                    },
                    overflowed,
                )
            }
        }
    };
}

/// Implements `wincode` schema traits by delegating to the wrapped `inner` value.
#[macro_export]
macro_rules! impl_wincode_schema_through_inner {
    ($name:ident) => {
        #[cfg(feature = "wincode")]
        unsafe impl<T, C> wincode::SchemaWrite<C> for $name<T>
        where
            C: wincode::config::ConfigCore,
            T: wincode::SchemaWrite<C, Src = T>,
        {
            type Src = Self;

            const TYPE_META: wincode::TypeMeta = <T as wincode::SchemaWrite<C>>::TYPE_META;

            fn size_of(src: &Self::Src) -> wincode::WriteResult<usize> {
                <T as wincode::SchemaWrite<C>>::size_of(&src.inner)
            }

            fn write(writer: &mut impl wincode::io::Writer, src: &Self::Src) -> wincode::WriteResult<()> {
                <T as wincode::SchemaWrite<C>>::write(writer, &src.inner)
            }
        }

        #[cfg(feature = "wincode")]
        unsafe impl<'de, T, C> wincode::SchemaRead<'de, C> for $name<T>
        where
            C: wincode::config::ConfigCore,
            T: wincode::SchemaRead<'de, C, Dst = T>,
        {
            type Dst = Self;

            const TYPE_META: wincode::TypeMeta = <T as wincode::SchemaRead<'de, C>>::TYPE_META;

            fn read(reader: &mut impl wincode::io::Reader<'de>, dst: &mut core::mem::MaybeUninit<Self::Dst>) -> wincode::ReadResult<()> {
                let mut value = core::mem::MaybeUninit::uninit();
                match <T as wincode::SchemaRead<'de, C>>::read(reader, &mut value) {
                    Ok(()) => {
                        let value = unsafe { value.assume_init() };
                        dst.write(Self {
                            inner: value,
                        });
                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! def_coefficient {
    ($name:ident) => {
        $crate::def_struct_coefficient!($name);
        $crate::impl_display_for_coefficient!($name);
    };
}

#[macro_export]
macro_rules! def_struct_coefficient {
    ($name:ident) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Default, Hash, Clone, Debug)]
        pub struct $name<Unit> {
            unit: Unit,
        }
    };
}

#[macro_export]
macro_rules! impl_display_for_coefficient {
    ($name:ident) => {
        impl<Unit: core::fmt::Display> core::fmt::Display for $name<Unit> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{name}<{unit}>", name = stringify!($name), unit = self.unit)
            }
        }
    };
}

mod scalar;
pub use scalar::*;
