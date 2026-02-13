use crate::{TestQuantity, TestScale, TestUnit, TestValue};

#[derive(PartialOrd, PartialEq, Clone, Copy, Debug)]
pub enum TestInput {
    QuantityValue { value: TestValue, quantity: TestQuantity, scale: TestScale, unit: TestUnit },
    Scalar { value: TestValue },
}
