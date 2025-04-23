mod utils;
pub use utils::random_color;

mod systems;
use systems::{system_listeners::keyboard_event_system, update_colours_system};

mod model;
mod state;
mod prelude {
    pub use crate::model::{tags, Board, Entity, Float64Value};
    #[allow(unused_imports)]
    pub use crate::state::{State, StateTrack};
    pub use crate::utils::random_color;
    pub use std::io::Result;
}

use crate::model::tags::BoardItem;
pub use crate::model::{Board, Entity, Float64Value};
pub use std::io::{ErrorKind, Result};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn set_custom_panic_hook() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("Application panicked!");
        default_panic(info);
    }));
}

pub fn main() {
    set_custom_panic_hook();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup.pipe(error_handler))
        .add_systems(FixedPreUpdate, keyboard_event_system.pipe(error_handler))
        .add_systems(FixedUpdate, update_colours_system.pipe(error_handler))
        .add_systems(FixedPostUpdate, panic_system_test.pipe(error_handler))
        .run();
}

fn panic_system_test() -> Result<()> {
    // Err(Error::new(ErrorKind::Other, "oh no"))
    Ok(())
}

fn error_handler(In(result): In<Result<()>>, mut _commands: Commands) {
    match result {
        Ok(_) => {}
        Err(err) => error!("System failed: {:?}", err),
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result<()> {
    commands.spawn(Camera2dBundle::default());

    let app_state = state::State::default();

    for i in 0..3 {
        for j in 0..3 {
            let material = materials.add(random_color());

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: Transform {
                        translation: Vec3::new((i * 10) as f32, (j * 10) as f32, 1.),
                        scale: Vec3::splat(10.),
                        ..default()
                    },
                    material,
                    ..default()
                },
                BoardItem,
            ));
        }
    }

    commands.insert_resource(app_state);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::PURPLE),
        ..default()
    });

    Ok(())
}
