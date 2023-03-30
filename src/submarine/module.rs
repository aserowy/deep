use bevy::prelude::Component;

#[derive(Component)]
pub struct ModuleComponent;

pub struct Module {
    pub id: String,
    pub icon: String,
    pub action: ModuleAction,
    pub cooldown: f32,
    pub current_cooldown: Option<f32>,
}

impl Module {
    pub fn new_mining_base() -> Self {
        Module {
            id: "mining_base".into(),
            icon: "󰜐".into(),
            action: ModuleAction::MiningMagnatide,
            cooldown: 4.0,
            current_cooldown: None,
        }
    }

    pub fn new_resource_scanner_base() -> Self {
        Module {
            id: "resource_scanner_base".into(),
            icon: "󰐷".into(),
            action: ModuleAction::ResourceScan,
            cooldown: 10.0,
            current_cooldown: None,
        }
    }
}

pub enum ModuleAction {
    MiningMagnatide,
    ResourceScan,
}
