use std::time::Duration;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::GameState;

#[derive(Component)]
struct SplashTextComponent {
    timer: Timer
}

pub struct SplashPlugin;

#[derive(TypeUuid)]
#[uuid = "e33faaf7-c08d-45b6-8fee-86f93b195484"]
pub struct SplashAssets {
    font_handle: Handle<Font>,
}

impl FromWorld for SplashAssets {
    fn from_world(world: &mut World) -> Self {
        let font_handle = world.resource::<AssetServer>().load("fonts/burnstown dam.ttf");

        SplashAssets { font_handle }
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SplashAssets>()
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_enter))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(splash_update));
    }
}

fn splash_enter(
    mut commands: Commands,
    spash_a: Res<SplashAssets>,
) {
    commands.spawn_bundle(
        TextBundle {
            text: 
                Text { 
                    sections: 
                        vec![
                            TextSection {
                                style: 
                                    TextStyle { 
                                        font: spash_a.font_handle.clone(),
                                        font_size: 100.0,
                                        ..default()
                                    },
                                value: String::from("YOU WIN!"),
                            }], 
                    alignment: 
                        TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center }
                },
            style:
                Style {
                    position_type: PositionType::Relative,
                    margin: Rect {
                        top: Val::Auto,
                        left: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Auto,
                    },
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
            
                    ..default()
                },
            ..default()
        })
        .insert(
            SplashTextComponent {
                timer: Timer::new(Duration::from_secs(3), false),
            });
    
}

fn splash_update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SplashTextComponent)>,
    mut exit: EventWriter<AppExit>,
) {
    for (ent, mut spc) in query.iter_mut() {
        spc.timer.tick(time.delta());
        if spc.timer.finished() {
            commands.entity(ent).despawn();
            exit.send(AppExit);
        }
    }
}