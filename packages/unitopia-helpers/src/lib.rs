#![no_std]

/// Defines a vanilla marker struct.
#[macro_export]
macro_rules! define_marker_struct {
    ($name:ident) => {
        #[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
        pub struct $name;
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
