use crate::prelude::*;
use bevy::prelude::*;

pub fn update_colours_system(
    handels: Query<&Handle<ColorMaterial>, With<tags::BoardItem>>,
    state: Res<crate::state::State>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_colour_update: Local<f64>,
) -> Result<()> {
    // info!("{:?}", handels.iter().count());
    // info!("board speed change: {:?}", state.board_speed_change.get());
    // info!("unix socket: {:?}", shared_data.0.lock().unwrap().len());

    if state.board_speed_change.get() < *last_colour_update {
        for handle in handels.iter() {
            if let Some(material) = materials.get_mut(handle) {
                material.color = random_color();
            }
        }
        *last_colour_update = 0.0;
    }
    *last_colour_update += time.delta().as_secs_f64();

    Ok(())
}
