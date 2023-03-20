use std::time::Duration;

use bevy::{
    prelude::*,
    time::{Stopwatch, Timer, TimerMode},
};
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Nishita},
    system_param::AtmosphereMut,
};

pub struct SkyPlugin {}

impl Default for SkyPlugin {
    fn default() -> Self {
        SkyPlugin {}
    }
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .insert_resource(AtmosphereModel::default())
            .insert_resource(CycleTimer::new(Duration::from_millis(50), 10.0))
            .add_plugin(AtmospherePlugin)
            .add_system(setup.on_startup())
            .add_system(daylight_cycle);
    }
}
#[derive(Component)]
struct Sun;

#[derive(Resource)]
struct CycleTimer {
    update: Timer,
    time: Stopwatch,
    speed: f32,
}

impl CycleTimer {
    pub fn new(duration: Duration, speed: f32) -> Self {
        Self {
            update: Timer::new(duration, TimerMode::Repeating),
            time: Stopwatch::new(),
            speed,
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.update.tick(delta);
        self.time.tick(delta.mul_f32(self.speed));
    }

    pub fn time(&self) -> f32 {
        self.time.elapsed().as_millis() as f32 / 2000.0
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
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
    timer.tick(time.delta());

    let mut pos = atmosphere.sun_position;
    let t = timer.time();
    pos.y = t.sin();
    pos.z = t.cos();
    atmosphere.sun_position = pos;

    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        light_trans.rotation = Quat::from_rotation_x(-pos.y.atan2(pos.z));
        directional.illuminance = t.sin().max(0.0).powf(2.0) * 100000.0;
    }
}
