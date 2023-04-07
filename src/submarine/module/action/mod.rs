use super::*;

use crate::submarine::power::PowerUsageComponent;

pub mod ressource_scanner;

#[derive(Clone, Component, Default)]
pub struct ChannelingComponent {
    pub current_duration: Option<f32>,
    pub duration: f32,
    pub watt_per_second: f32,
}

pub fn update_module_channeling_state_transition(
    time: Res<Time>,
    mut query: Query<(&mut ModuleStateComponent, &mut ChannelingComponent)>,
) {
    let dt = time.delta_seconds();

    for (mut state, mut channel) in query.iter_mut() {
        if let Some(duration) = channel.current_duration {
            let duration = duration + dt;
            channel.current_duration = Some(duration);

            if duration >= channel.duration {
                state.state.next(ModuleStatus::Aftercast);
                channel.current_duration = None;
            }
        }
    }
}

pub fn set_power_usage_for_channels(
    time: Res<Time>,
    mut query: Query<(&ChannelingComponent, &mut PowerUsageComponent)>,
) {
    let dt = time.delta_seconds();

    for (channel, mut usage) in query.iter_mut() {
        if channel.current_duration.is_some() {
            usage.watt_per_second = channel.watt_per_second * dt;
        }
    }
}

pub fn handle_module_state_for_channels(
    mut query: Query<(&mut ModuleStateComponent, &mut ChannelingComponent)>,
) {
    for (state, mut channel) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Passive => (),
            ModuleStatus::StartingUp => channel.current_duration = None,
            ModuleStatus::Active => channel.current_duration = None,
            ModuleStatus::Triggered => {
                if channel.current_duration.is_none() {
                    channel.current_duration = Some(0.0);
                }
            }
            ModuleStatus::Aftercast => channel.current_duration = None,
            ModuleStatus::ShuttingDown => channel.current_duration = None,
            ModuleStatus::Inactive => channel.current_duration = None,
        }
    }
}

/* pub fn new_miner_magnatide_basic() -> (ModuleBundle, ActionComponent, PowerUsageComponent) {
    (
        ModuleBundle {
            details: ModuleDetailsComponent {
                id: Uuid::new_v4(),
                icon: "󰜐".into(),
            },
            state: ModuleStateComponent {
                state: ModuleState::new(),
            },
        },
        ActionComponent {},
        PowerUsageComponent::default(),
    )
} */
