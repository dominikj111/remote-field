use crate::prelude::*;
use bevy::prelude::*;
use std::io::Result;

pub fn update_colours_system(
    query: Query<&MeshMaterial2d<ColorMaterial>, With<tags::BoardItem>>,
    state: Res<crate::state::State>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_colour_update: Local<f64>,
) -> Result<()> {
    // info!("{:?}", query.iter().count());
    // info!("board speed change: {:?}", state.board_speed_change.get());

    if state.board_speed_change.get() < *last_colour_update {
        for material_handle in query.iter() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.color = random_color();
            }
        }
        *last_colour_update = 0.0;
    }
    *last_colour_update += time.delta().as_secs_f64();

    Ok(())
}
