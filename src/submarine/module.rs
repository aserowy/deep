use bevy::prelude::Component;

#[derive(Component)]
pub struct ModuleComponent {
}

pub struct Module {
    pub icon: String,
    pub cooldown: f32,
    pub current_cooldown: f32,
}
