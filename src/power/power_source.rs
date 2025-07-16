use bevy::prelude::Component;

#[derive(Component)]
pub struct PowerSource {
    pub powered: bool,
}

impl Default for PowerSource {
    fn default() -> Self {
        Self { powered: true }
    }
}
