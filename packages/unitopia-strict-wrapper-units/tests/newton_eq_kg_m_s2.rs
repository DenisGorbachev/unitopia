use unitopia_strict_wrapper_units::{Kilogram, Meter, Newton, Second};

#[test]
fn newton_eq_kg_m_s2() {
    let kilogram = Kilogram::new(1u32);
    let meter = Meter::new(1u32);
    let second = Second::new(1u32);
    let computed: Newton<u32> = {
        let numerator = kilogram * meter;
        let denominator = second * second;
        numerator / denominator
    };
    let expected = Newton::from(1u32);
    assert_eq!(computed.inner, expected.inner);
}
