use bevy::prelude::*;

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .run();
}
