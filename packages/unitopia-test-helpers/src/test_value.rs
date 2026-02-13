use derive_more::From;
use ruint::aliases::U256;

#[derive(From, PartialOrd, PartialEq, Clone, Copy, Debug)]
pub enum TestValue {
    U32(u32),
    U64(u64),
    F64(f64),
    Decimal(rust_decimal::Decimal),
    U256(U256),
}

impl TestValue {}
