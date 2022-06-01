use crate::Board;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;

pub fn input_handling(
    windows: Res<Windows>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    let window = windows.get_primary().unwrap();

    for event in button_evr.iter() {
        if let ElementState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            info!("Trying to uncover tile on {}", coordinates);
                            // TODO: geate event
                        }
                        MouseButton::Right => {
                            info!("Trying to uncover tile on {}", coordinates);
                            // TODO: geate event
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
