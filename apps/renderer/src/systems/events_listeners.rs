pub mod system_listeners {
    use bevy::input::{keyboard::KeyboardInput, ButtonState};
    use bevy::prelude::*;

    pub fn keyboard_event_system(
        mut keyboard_input_events: EventReader<KeyboardInput>,
        mut state: ResMut<crate::state::State>,
    ) -> crate::prelude::Result<()> {
        for event in keyboard_input_events.read() {
            match event.state {
                ButtonState::Pressed => {
                    state.keyboard_pressed.insert(event.key_code);

                    match event.key_code {
                        KeyCode::KeyQ => {
                            state.board_speed_change.increment();
                        }
                        KeyCode::KeyA => {
                            state.board_speed_change.decrement();
                        }
                        _ => {}
                    }
                }
                ButtonState::Released => {
                    state.keyboard_pressed.remove(&event.key_code);
                }
            }
        }

        Ok(())
    }
}
