use crate::{GameState, network::{HostAddress, NetworkState}};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.insert_resource(IpJoinInput::default());

        app.add_enter_system(GameState::Menu, setup_menu);
        app.add_enter_system(GameState::JoinMenu, setup_join_menu);

        app.add_exit_system(GameState::Menu, despawn_ui);
        app.add_exit_system(GameState::JoinMenu, despawn_ui);

        app.add_system_set(
            ConditionSet::new()
                .run_not_in_state(GameState::Playing)
                .with_system(button_system)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::JoinMenu)
                .with_system(ip_input_system)
                .into(),
        );
    }
}

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct IpJoinInput(String);

#[derive(Component)]
enum MenuButton {
    Host,
    Join,
    JoinGo,
}

#[derive(Component)]
struct IpInputText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiAssets {
        font: asset_server.load("roboto.ttf"),
    });
}

fn setup_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButton::Host,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Host",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButton::Join,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn setup_join_menu(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Ip Address",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ),
                IpInputText,
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::horizontal(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButton::JoinGo,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join",
                        TextStyle {
                            font: ui_assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut host_ip: ResMut<HostAddress>,
    ip_input: Res<IpJoinInput>,
) {
    for (interaction, mut color, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match menu_button {
                    MenuButton::Host => {
                        commands.insert_resource(NextState(GameState::Playing));
                        commands.insert_resource(NextState(NetworkState::Host));
                    }
                    MenuButton::Join => {
                        commands.insert_resource(NextState(GameState::JoinMenu));
                    }
                    MenuButton::JoinGo => {
                        **host_ip = ip_input.to_owned();
                        commands.insert_resource(NextState(GameState::Playing));
                        commands.insert_resource(NextState(NetworkState::Client));
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

const BACKSPACE_CHAR: char = 8 as char;
fn ip_input_system(
    mut key_events: EventReader<ReceivedCharacter>,
    mut input: ResMut<IpJoinInput>,
    mut query: Query<&mut Text, With<IpInputText>>,
) {
    for key in key_events.iter() {
        match key.char {
            BACKSPACE_CHAR => {
                let len = input.len();
                if len == 0 {
                    break;
                }
                input.remove(len - 1);
            }
            '.' => {
                input.push('.');
            }
            _ => {
                if key.char.is_numeric() {
                    input.push(key.char);
                }
            }
        }
    }
    if input.len() == 0 {
        return;
    }
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = input.to_owned();
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
