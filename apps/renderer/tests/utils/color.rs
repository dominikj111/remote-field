use bevy::color::ColorToComponents;
use renderer::random_color;

#[test]
fn provide_random_color() {
    let rc = random_color();
    assert_eq!(rc.to_srgba().to_f32_array()[3], 1.0);
    assert_ne!(random_color(), random_color());
}
