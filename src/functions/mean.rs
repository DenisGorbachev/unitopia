use crate::Measure;
use num_traits::Zero;
use std::ops::{Add, Div};

// TODO: integrate with `statsrs` crate
pub fn mean<'a, Quantity: 'a, Value: 'a>(values: impl IntoIterator<Item = &'a Measure<Quantity, Value>>) -> Option<Measure<Quantity, Value>>
where
    Measure<Quantity, Value>: for<'b> Add<&'b Measure<Quantity, Value>, Output = Measure<Quantity, Value>> + Div<usize, Output = Measure<Quantity, Value>> + Zero,
{
    let (sum, count) = values
        .into_iter()
        .fold((Measure::zero(), 0usize), |(sum, count), value| (sum + value, count + 1));

    if count == 0 { None } else { Some(sum / count) }
}
