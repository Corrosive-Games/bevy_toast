use std::time::Duration;

use bevy::{prelude::*, window::WindowMode};
use bevy_toast::{ShowToast, ToastPlugin};

#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugins(DefaultPlugins)
        .add_plugin(ToastPlugin)
        .insert_resource(WindowDescriptor {
            title: "bevy_toast: Hello world!".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(key_handler)
        .run();
}

/// Adding an UI camera and Helper text
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            color: Color::rgba_u8(0, 0, 0, 0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Press 'E' to show toast",
                    TextStyle {
                        font: asset_server.load("Roboto-Regular.ttf"),
                        font_size: 48.,
                        color: Color::WHITE.into(),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        });
}

/// handler for keyboard key presses
fn key_handler(mut toast_evt: EventWriter<ShowToast>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::E) {
        toast_evt.send(ShowToast {
            title: "Achievement reached!".to_string(),
            subtitle: "Hello, World".to_string(),
            duration: Duration::from_secs(2),
        });
    }
}
