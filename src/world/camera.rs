use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, controls);
    }
}

fn setup(mut commands: Commands, window: Single<&Window>) {
    // let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn(Camera2d);
}

fn controls(
    mut camera_query: Query<(&mut Camera, &mut Transform, &mut Projection), With<Camera>>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time<Fixed>>,
) {
    let Ok((_, mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    for mouse_wheel_event in mouse_wheel_events.read() {
        if let Projection::Orthographic(projection_2d) = &mut *projection {
            if mouse_wheel_event.y < 0.0 {
                projection_2d.scale *= 1.1;
            } else {
                projection_2d.scale /= 1.1;
            }
        }
    }
}
