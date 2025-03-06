use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "dude with a sword".to_string(),
            resolution: (512.0, 512.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }));
    app.run();
}
    
