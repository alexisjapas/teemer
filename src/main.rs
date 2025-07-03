use bevy::prelude::*;

/// Window & playground constants
const WINDOW_WIDTH:  f32 = 540.0;
const WINDOW_HEIGHT: f32 = 960.0;
const BALL_RADIUS:   f32 = 15.0;
const BALL_SPEED:    f32 = 250.0;

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

    commands
        .spawn((
            Sprite {
                color: Color::linear_rgb(255.0, 255.0, 0.0),
                custom_size: Some(Vec2::splat(BALL_RADIUS * 2.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity(Vec2::splat(BALL_SPEED)),
        ));
}

/// Move the ball and reflect its velocity when it hits a wall
fn move_ball(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let half_w = WINDOW_WIDTH  / 2.0 - BALL_RADIUS;
    let half_h = WINDOW_HEIGHT / 2.0 - BALL_RADIUS;

    for (mut transform, mut vel) in &mut query {
        transform.translation += vel.0.extend(0.0) * dt;

        if transform.translation.x >  half_w {
            transform.translation.x =  half_w;
            vel.0.x = -vel.0.x;
        }
        if transform.translation.x < -half_w {
            transform.translation.x = -half_w;
            vel.0.x = -vel.0.x;
        }
        if transform.translation.y >  half_h {
            transform.translation.y =  half_h;
            vel.0.y = -vel.0.y;
        }
        if transform.translation.y < -half_h {
            transform.translation.y = -half_h;
            vel.0.y = -vel.0.y;
        }
    }
}
