use bevy::prelude::*;

#[derive(Component)]
pub struct SettingsComponent {
    pub enabled: bool,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self {
            enabled: true,
        }
    }
}

#[derive(Component)]
pub struct KeyMapComponent {
    pub key_actions: Vec<KeyActionMap>,
}

#[derive(Clone)]
pub struct KeyActionMap {
    pub key_code: KeyCode,
    pub key_action: KeyAction,
}

#[derive(Clone)]
pub struct KeyActionEvent {
    pub key_map: KeyActionMap,
    pub key_press: KeyPress,
}

#[derive(Clone)]
pub enum KeyPress {
    Down(),
    Hold(),
    Release(),
}

#[derive(Clone)]
pub enum KeyAction {
    ThrustPositiv,
    ThrustNegative,
    ThrustZero,
    ThrustUp,
    ThrustDown,
    ModuleActivation01,
    ModuleActivation02,
}

pub fn handle_key_presses(
    key_input: Res<Input<KeyCode>>,
    query: Query<&KeyMapComponent, With<Camera>>,
    mut key_action_event_writer: EventWriter<KeyActionEvent>,
) {
    if let Ok(key_map) = query.get_single() {
        for key_action_map in key_map.key_actions.iter() {
            if key_input.just_released(key_action_map.key_code) {
                key_action_event_writer.send(KeyActionEvent {
                    key_map: key_action_map.clone(),
                    key_press: KeyPress::Release(),
                });

                continue;
            }

            let key_press = get_standard_key_press_for_action_event(&key_action_map.key_action);

            match key_press {
                KeyPress::Down() => {
                    if key_input.just_pressed(key_action_map.key_code) {
                        key_action_event_writer.send(KeyActionEvent {
                            key_map: key_action_map.clone(),
                            key_press: KeyPress::Down(),
                        });
                    }
                }
                KeyPress::Hold() => {
                    if key_input.pressed(key_action_map.key_code) {
                        key_action_event_writer.send(KeyActionEvent {
                            key_map: key_action_map.clone(),
                            key_press: KeyPress::Hold(),
                        });
                    }
                }
                KeyPress::Release() => (),
            };
        }
    };
}

fn get_standard_key_press_for_action_event(event: &KeyAction) -> KeyPress {
    match event {
        KeyAction::ThrustPositiv => KeyPress::Hold(),
        KeyAction::ThrustNegative => KeyPress::Hold(),
        KeyAction::ThrustZero => KeyPress::Down(),
        KeyAction::ThrustUp => KeyPress::Down(),
        KeyAction::ThrustDown => KeyPress::Down(),
        KeyAction::ModuleActivation01 => KeyPress::Hold(),
        KeyAction::ModuleActivation02 => KeyPress::Hold(),
    }
}

