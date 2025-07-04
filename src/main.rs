use bevy::prelude::*;
use rand::random;

/// Window & playground constants
const WINDOW_WIDTH: f32 = 540.0;
const WINDOW_HEIGHT: f32 = 960.0;
const NB_ENTITIES: i32 = 4444;
const SQUARE_LEN: f32 = 44.0;
const MAX_SPEED: f32 = 444.0;

/// Simple velocity component
#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "Bouncing Ball".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_ball)
        .run();
}

/// Spawn the 2-D camera and the ball entity
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    for _i in 0..NB_ENTITIES {
        commands.spawn((
            Sprite {
                color: Color::linear_rgb(random(), random(), random()),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
        ));
    }
}

/// Move the ball and reflect its velocity when it hits a wall
fn move_ball(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    let dt = time.delta_secs();
    let half_w = WINDOW_WIDTH / 2.0 - SQUARE_LEN / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0 - SQUARE_LEN / 2.0;

    for (mut position, mut vel) in &mut query {
        position.translation += vel.0.extend(0.0) * dt;

        if position.translation.x > half_w {
            position.translation.x = half_w;
            vel.0.x = -vel.0.x;
        }
        if position.translation.x < -half_w {
            position.translation.x = -half_w;
            vel.0.x = -vel.0.x;
        }
        if position.translation.y > half_h {
            position.translation.y = half_h;
            vel.0.y = -vel.0.y;
        }
        if position.translation.y < -half_h {
            position.translation.y = -half_h;
            vel.0.y = -vel.0.y;
        }
    }
}
