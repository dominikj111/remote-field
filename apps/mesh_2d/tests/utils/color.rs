use mesh_2d::random_color;

#[test]
fn provide_random_color() {
    let rc = random_color();
    assert_eq!(rc.as_rgba_f32().len(), 4);
    assert_eq!(rc.as_rgba_f32()[3], 1.0);
    assert_ne!(random_color(), random_color());
}
