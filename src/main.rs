use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    
    app
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
        primary_window: Some(Window {
            title: "dude with a sword".to_string(),
            resolution: (512.0, 512.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))

    .add_systems(Startup, abc)
    .run();
}
    
fn abc(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawn camera
    commands.spawn(Camera2d::default());


    let img = asset_server.load("Dude.png");
    commands.spawn(
        (Sprite {
            image: img,
            color: Color::from(Srgba::new(1.0, 1.0, 1.0, 1.0)),

            ..Default::default()
        }, 
        Transform::default()
    )
    );
}

