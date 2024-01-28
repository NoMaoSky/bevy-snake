use std::time::Duration;

use bevy::{prelude::*, sprite::collide_aabb::collide, window::WindowResolution};
use component::{Body, Fruit, Snake};

use crate::component::Direction;

mod component;

const BOX_SIZE: Vec2 = Vec2::new(50., 50.);
const BOX_RANGE: IVec2 = IVec2::new(-5, 5);
const WINDOW_SIZE: f32 = 550.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_SIZE, WINDOW_SIZE),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(0.5)))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                draw_snake_system,
                draw_fruit_system,
                spawn_fruit_system,
                control_snake_system,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                move_snake_system,
                snake_eat_fruit_system,
                snake_collide_self_system,
            ),
        )
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Snake {
        body: Body::new(0., 0.)
            .push(Body::new(BOX_SIZE.x * 1., 0.))
            .push(Body::new(BOX_SIZE.x * 2., 0.)),
    });
}

fn draw_snake_system(snake_query: Query<&Snake>, mut gizmos: Gizmos) {
    if let Ok(snake) = snake_query.get_single() {
        let spawn_body_func = |pos: Vec2| gizmos.rect_2d(pos, 0., BOX_SIZE, Color::GREEN);

        snake.body.spawn(spawn_body_func);

        gizmos.rect_2d(
            Vec2::ZERO,
            0.,
            Vec2::new(WINDOW_SIZE, WINDOW_SIZE),
            Color::BLUE,
        );
    }
}

fn control_snake_system(mut snake_query: Query<&mut Snake>, input: Res<Input<KeyCode>>) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        let direction = snake.body.direction();

        if input.just_pressed(KeyCode::W) && Direction::Down != direction {
            snake.body.set_direction(Direction::Up)
        }
        if input.just_pressed(KeyCode::A) && Direction::Right != direction {
            snake.body.set_direction(Direction::Left)
        }
        if input.just_pressed(KeyCode::D) && Direction::Left != direction {
            snake.body.set_direction(Direction::Right)
        }
        if input.just_pressed(KeyCode::S) && Direction::Up != direction {
            snake.body.set_direction(Direction::Down)
        }
    }
}

fn move_snake_system(mut snake_query: Query<&mut Snake>) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        snake.body.r#move(BOX_SIZE.x);
    }
}

fn spawn_fruit_system(
    mut commands: Commands,
    snake_query: Query<&Snake>,
    fruit_query: Query<&Fruit>,
) {
    if fruit_query.get_single().is_ok() {
        return;
    }

    let snake = snake_query.get_single().unwrap();

    loop {
        let fruit = Fruit::random();

        if !snake.body.collide(&fruit.positon) {
            commands.spawn(fruit);
            break;
        }
    }
}

fn draw_fruit_system(fruit_query: Query<&Fruit>, mut gizmos: Gizmos) {
    if let Ok(fruit) = fruit_query.get_single() {
        gizmos.circle_2d(fruit.positon, 20., Color::RED);
    }
}

fn snake_eat_fruit_system(
    mut commands: Commands,
    mut snake_query: Query<&mut Snake>,
    fruit_query: Query<(Entity, &mut Fruit)>,
) {
    if let Ok(mut snake) = snake_query.get_single_mut() {
        if let Ok((entity, fruit)) = fruit_query.get_single() {
            if collide(
                snake.body.position().extend(1.),
                BOX_SIZE,
                fruit.positon.extend(1.),
                BOX_SIZE,
            )
            .is_some()
            {
                commands.entity(entity).despawn();
                let mut last = snake.body.last();
                last.wait();
                snake.body.push(last);
            }
        }
    }
}

fn snake_collide_self_system(mut commands: Commands, snake_query: Query<(Entity, &Snake)>) {
    if let Ok((entity, snake)) = snake_query.get_single() {
        if snake.body.collide_self() {
            commands.entity(entity).despawn();
        }
    }
}
