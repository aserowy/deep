use std::time::Duration;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    time::{Timer, TimerMode},
};
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Nishita},
    system_param::AtmosphereMut,
};
use bevy_water::{WaterPlugin, WaterSettings};

const WATER_HEIGHT: f32 = 5.0;

pub struct SkyPlugin {}

impl Default for SkyPlugin {
    fn default() -> Self {
        SkyPlugin {}
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            // sky
            .insert_resource(AtmosphereModel::default())
            .insert_resource(CycleTimer(Timer::new(
                Duration::from_millis(50),
                TimerMode::Repeating,
            )))
            .add_plugin(AtmospherePlugin)
            .add_system(setup.on_startup())
            .add_system(daylight_cycle)
            // water
            .insert_resource(WaterSettings {
                height: WATER_HEIGHT,
            })
            .add_plugin(WaterPlugin);
    }
}
#[derive(Component)]
struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

fn setup(mut commands: Commands) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                num_cascades: 8,
                ..default()
            }
            .build(),
            ..default()
        })
        .insert(Sun);
}

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        // TODO: / 20.0 manipulates the duration: refactor into Duration of one day/night cycle
        let t = time.elapsed_seconds_wrapped() as f32 / 20.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t.sin().atan2(t.cos()));
            directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
        }
    }
}
