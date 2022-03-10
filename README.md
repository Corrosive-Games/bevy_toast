# 🍞 Bevy Toast

[![License: MIT/Apache](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](https://opensource.org/licenses/MIT)
[![Doc](https://docs.rs/bevy_toast/badge.svg)](https://docs.rs/bevy_tweening)
[![Crate](https://img.shields.io/crates/v/bevy_toast.svg)](https://crates.io/crates/bevy_tweening)
[![Coverage Status](https://coveralls.io/repos/github/Nightlyside/bevy_toast/badge.svg?branch=main&kill_cache=1)](https://coveralls.io/github/Nightlyside/bevy_toast?branch=main)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.6-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A bevy plugin to easily show toast notifications to the player

## Features

-   [x] Add in-game notifications to your bevy project
-   [x] Customize the duration and the text of the toast

## Demo

The source code for the following example is available here: [/examples/hello_world.rs](https://github.com/NightlySide/bevy_toast/blob/main/examples/hello_world.rs)

![Hello world toast GIF](.readme_assets/hello_world_toast.gif)

## Usage

### Plugin setup

Add the following plugins to your project:

```rust
use bevy_tweening::TweeningPlugin;
use bevy_toast::ToastPlugin;

fn main() {
    App::new()
        .add_plugin(TweeningPlugin)
        .add_plugin(ToastPlugin)
        .run();
}
```

### Send a toast 🍞

Sending a Toast is a simple as sending a `ShowToast` event:

```rust
// send a toast when pressing 'E'
fn keyboard_handler(
    keyboard: Res<Input<KeyCode>>,
    toast_sender: EventWriter<ShowToast>,
) {
    if keyboard.just_pressed(KeyCode::E) {
        toast_evt.send(ShowToast {
            title: "Achievement reached!".to_string(),
            subtitle: "You pressed 'E'".to_string(),
            duration: Duration::from_secs(2),
        });
    }
}
```

### Compatible versions

| bevy | bevy_toast |
| ---- | ---------- |
| 0.6  | 0.1        |
