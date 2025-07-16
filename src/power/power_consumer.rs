use bevy::prelude::Component;

#[derive(Component)]
pub struct PowerConsumer {
    pub powered: bool,
}

impl Default for PowerConsumer {
    fn default() -> Self {
        Self { powered: false }
    }
}
