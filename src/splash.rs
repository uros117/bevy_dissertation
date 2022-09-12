use std::time::Duration;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::Instant;

use super::GameState;

#[derive(Component)]
struct SplashTextComponent {
    timer: Timer,
    start_time: Option<Instant>,
}

pub struct SplashPlugin;

pub struct SplashRes {
    font_handle: Handle<Font>,
}

impl FromWorld for SplashRes {
    fn from_world(world: &mut World) -> Self {
        let font_handle = world.resource::<AssetServer>().load("fonts/arial.ttf");

        SplashRes { font_handle }
    }
}

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SplashRes>()
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_enter))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(splash_update))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(fade_animation));
    }
}

fn splash_enter(
    mut commands: Commands,
    spash_a: Res<SplashRes>,
) {
    commands.spawn_bundle(
        TextBundle::from_sections(
            vec![
                TextSection {
                    style: 
                        TextStyle { 
                            font: spash_a.font_handle.clone(),
                            font_size: 100.0,
                            ..default()
                        },
                    value: String::from("YOU WIN!"),
                }]).with_style(
                    Style {
                        position_type: PositionType::Relative,
                        margin: UiRect {
                            top: Val::Auto,
                            left: Val::Auto,
                            right: Val::Auto,
                            bottom: Val::Auto,
                        },
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }
                )
            )
        .insert(
            SplashTextComponent {
                timer: Timer::new(Duration::from_secs(5), false),
                start_time: Some(Instant::now()),
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

fn fade_animation(
    _commands: Commands,
    mut query: Query<(&mut Text, &mut SplashTextComponent)>,
) {
    query.for_each_mut(|iter| {
        let (mut text, mut stc) = iter;

        if let None = stc.start_time {
            stc.start_time = Some(Instant::now());
        }

        if let Some(start_instant) = stc.start_time {
            let delta = Instant::now() - start_instant;
            
            let ratio = (5.0 - delta.as_secs_f32()).clamp(0.0, 1.0);

            for i in text.sections.iter_mut() {
                i.style.color.set_a(ratio);
            }
        }
    });
}