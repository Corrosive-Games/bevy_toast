use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;
use bevy_tweening::{
    lens::UiPositionLens, Animator, Delay, EaseFunction, Sequence, Tween, TweeningPlugin,
    TweeningType,
};

const TOAST_WIDTH: f32 = 300.;
const TOAST_HEIGHT: f32 = 75.;

// --------------- PLUGIN --------------- //

pub struct ToastPlugin;

impl Plugin for ToastPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TweeningPlugin);

        app.add_event::<ShowToast>()
            .insert_resource(ToastQueue::default())
            .add_startup_system(build_ui)
            .add_system(always_on_top)
            .add_system(toast_evt_reader)
            .add_system(display_toast);
    }
}

// --------------- RESOURCES --------------- //

/// Event which represent the data sent to a toast
#[derive(Clone)]
pub struct ShowToast {
    pub title: String,
    pub subtitle: String,
    pub duration: Duration,
}

impl ShowToast {
    pub fn get_animation(&self) -> Sequence<Style> {
        let close_animation = Tween::new(
            EaseFunction::CubicInOut,
            TweeningType::Once,
            std::time::Duration::from_secs(1),
            UiPositionLens {
                start: Rect {
                    left: Val::Auto,
                    top: Val::Px(5.),
                    right: Val::Px(5.),
                    bottom: Val::Auto,
                },
                end: Rect {
                    left: Val::Auto,
                    top: Val::Px(-100.),
                    right: Val::Px(5.),
                    bottom: Val::Auto,
                },
            },
        );

        let delay = Delay::new(self.duration);

        let open_animation = Tween::new(
            EaseFunction::CubicInOut,
            TweeningType::Once,
            std::time::Duration::from_secs_f32(0.5),
            UiPositionLens {
                end: Rect {
                    left: Val::Auto,
                    top: Val::Px(5.),
                    right: Val::Px(5.),
                    bottom: Val::Auto,
                },
                start: Rect {
                    left: Val::Auto,
                    top: Val::Px(-100.),
                    right: Val::Px(5.),
                    bottom: Val::Auto,
                },
            },
        );

        open_animation.then(delay.then(close_animation))
    }
}

/// Queue of toast to display
struct ToastQueue {
    queue: VecDeque<ShowToast>,
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

// --------------- COMPONENTS --------------- //

#[derive(Component)]
struct ToastUI;

#[derive(Component)]
struct ToastTitle;

#[derive(Component)]
struct ToastSubtitle;

// --------------- SYSTEMS --------------- //

/// system which puts the toast always on top of everything
/// in order to fight the default impl of bevy's ui
fn always_on_top(mut query: Query<&mut GlobalTransform, With<ToastUI>>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = f32::MAX;
    }
}

/// reads events and put the toasts into the queue
fn toast_evt_reader(mut evt_reader: EventReader<ShowToast>, mut queue: ResMut<ToastQueue>) {
    for toast_info in evt_reader.iter() {
        // anti spam
        let matching_toast = queue
            .queue
            .iter()
            .filter(|toast| toast.subtitle.eq(&toast_info.subtitle))
            .collect::<Vec<_>>();

        // if the toast is already in the queue, drop it
        if !matching_toast.is_empty() {
            continue;
        }

        // adding the toast to the queue
        queue.queue.push_back(toast_info.clone());
    }
}

fn display_toast(
    mut queue: ResMut<ToastQueue>,
    mut anim_query: Query<&mut Animator<Style>, With<ToastUI>>,
    mut title_query: Query<&mut Text, (With<ToastTitle>, Without<ToastSubtitle>)>,
    mut subtitle_query: Query<&mut Text, (With<ToastSubtitle>, Without<ToastTitle>)>,
) {
    let mut animator = anim_query.get_single_mut().unwrap();
    let mut title = title_query.get_single_mut().unwrap();
    let mut subtitle = subtitle_query.get_single_mut().unwrap();

    // if the animation is finished, then the previous toast is hidden
    // we can show the next one if the queue is not empty
    if (animator.progress() == 0.0 || animator.progress() == 1.0) && !queue.queue.is_empty() {
        let next_toast = queue.queue.pop_front().unwrap();
        title.sections[0].value = next_toast.title.clone();
        subtitle.sections[0].value = next_toast.subtitle.clone();
        animator.set_tweenable(next_toast.get_animation());
        animator.rewind();
    }
}

// --------------- STARTUP SYSTEMS --------------- //

/// building a toast component
fn build_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI Components
    let container = ImageBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(-100.),
                right: Val::Px(5.),
                ..Default::default()
            },
            size: Size::new(Val::Px(TOAST_WIDTH), Val::Px(TOAST_HEIGHT)),
            ..Default::default()
        },
        color: Color::rgba_u8(0, 0, 0, 0).into(),
        ..Default::default()
    };

    let background_image = ImageBundle {
        image: asset_server.load("toast_background.png").into(),
        style: Style {
            align_self: AlignSelf::Center,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            flex_grow: 1.,
            padding: Rect {
                left: Val::Px(20.),
                right: Val::Px(20.),
                top: Val::Px(15.),
                bottom: Val::Px(15.),
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let toast_title_text = TextBundle {
        text: Text::with_section(
            "Advancement Made!",
            TextStyle {
                font_size: 24.,
                font: asset_server.load("Roboto-Bold.ttf"),
                color: Color::rgb_u8(205, 205, 100).into(),
                ..Default::default()
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                vertical: VerticalAlign::Center,
            },
        ),
        ..Default::default()
    };

    let toast_subtitle_text = TextBundle {
        text: Text::with_section(
            "Iron tools",
            TextStyle {
                font_size: 24.,
                font: asset_server.load("Roboto-Regular.ttf"),
                color: Color::rgb_u8(205, 205, 205).into(),
                ..Default::default()
            },
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                vertical: VerticalAlign::Center,
            },
        ),
        ..Default::default()
    };

    // building ui tree
    commands
        .spawn_bundle(container)
        .with_children(|parent| {
            parent
                .spawn_bundle(background_image)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(toast_title_text)
                        .insert(Name::new("Toast title"))
                        .insert(ToastTitle);

                    parent
                        .spawn_bundle(toast_subtitle_text)
                        .insert(Name::new("Toast subtitle"))
                        .insert(ToastSubtitle);
                })
                .insert(Name::new("Toast background"));
        })
        .insert(Name::new("Toast container"))
        .insert(ToastUI)
        .insert(Animator::<Style>::default());
}
