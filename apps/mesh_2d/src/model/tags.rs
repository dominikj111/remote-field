use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Component)]
pub struct BoardItem;

#[derive(Resource)]
pub struct SharedData(pub Arc<Mutex<Vec<String>>>);
