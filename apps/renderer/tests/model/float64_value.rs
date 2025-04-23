use renderer::Float64Value;

#[test]
fn allows_to_increment_value_by_step_to_max() {
    let mut value = Float64Value::new(0.0, -10.0, 8.0, 3.2);
    value.increment();
    value.increment();
    assert_eq!(value.get(), 6.4);
    value.increment();
    assert_eq!(value.get(), 8.0);
    value.increment();
    assert_eq!(value.get(), 8.0);
}

#[test]
fn need_to_decrement_value_by_step_to_min() {
    let mut value = Float64Value::new(1.0, -8.0, 10.0, 3.2);
    value.decrement();
    value.decrement();
    assert_eq!(value.get(), -5.4);
    value.decrement();
    assert_eq!(value.get(), -8.0);
    value.decrement();
    assert_eq!(value.get(), -8.0);
}
