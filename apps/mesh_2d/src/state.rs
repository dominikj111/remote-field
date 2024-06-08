use bevy::asset::Handle;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::ColorMaterial;
use bevy::prelude::Resource;
use std::collections::HashSet;

use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct State {
    pub board: Board<Entity<&'static Handle<ColorMaterial>>>,
    pub keyboard_pressed: HashSet<KeyCode>,
    pub board_speed_change: Float64Value,
}

impl Default for State {
    fn default() -> Self {
        Self {
            board: Board::default(),
            keyboard_pressed: HashSet::new(),
            board_speed_change: Float64Value::new(1.0, 0.0, 10.0, 1.0),
        }
    }
}

#[derive(Default, Resource, Debug)]
pub struct StateTrack(pub (State, State), pub bool);
