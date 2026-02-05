use core::marker::PhantomData;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Prod<A, B, T> {
    pub inner: T,
    unit: PhantomData<(A, B)>,
}

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Quot<A, B, T> {
    pub inner: T,
    unit: PhantomData<(A, B)>,
}

macro_rules! impl_open_wrapper_arith_output {
    ($name:ident) => {
        impl<A, B, T> core::ops::Deref for $name<A, B, T>
        where
            T: core::ops::Deref,
        {
            type Target = <T as core::ops::Deref>::Target;

            fn deref(&self) -> &Self::Target {
                core::ops::Deref::deref(&self.inner)
            }
        }

        impl<A, B, T> core::ops::DerefMut for $name<A, B, T>
        where
            T: core::ops::DerefMut,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                core::ops::DerefMut::deref_mut(&mut self.inner)
            }
        }

        impl<A, B, T, U> core::convert::AsRef<U> for $name<A, B, T>
        where
            T: core::convert::AsRef<U>,
        {
            fn as_ref(&self) -> &U {
                core::convert::AsRef::as_ref(&self.inner)
            }
        }

        impl<A, B, T> core::borrow::Borrow<T> for $name<A, B, T> {
            fn borrow(&self) -> &T {
                &self.inner
            }
        }

        impl<A, B, T> From<T> for $name<A, B, T> {
            fn from(inner: T) -> Self {
                Self {
                    inner,
                    unit: PhantomData,
                }
            }
        }
    };
}

impl_open_wrapper_arith_output!(Prod);
impl_open_wrapper_arith_output!(Quot);

impl<A, B, C, D, LhsValue, RhsValue, OutValue> core::ops::Div<Prod<C, D, RhsValue>> for Prod<A, B, LhsValue>
where
    LhsValue: core::ops::Div<RhsValue, Output = OutValue>,
{
    type Output = Quot<Prod<A, B, LhsValue>, Prod<C, D, RhsValue>, OutValue>;

    fn div(self, rhs: Prod<C, D, RhsValue>) -> Self::Output {
        Quot::from(core::ops::Div::div(self.inner, rhs.inner))
    }
}
