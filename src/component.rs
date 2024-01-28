use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::Rng;

use crate::{BOX_RANGE, BOX_SIZE};

#[derive(Component)]
pub struct Snake {
    pub body: Body,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Component, Debug, Clone)]
pub enum Body {
    Some(Vec2, Direction, Box<Body>, bool),
    None,
}

impl Body {
    pub fn new(x: f32, y: f32) -> Self {
        Body::Some(
            Vec2::new(x, y),
            Direction::Left,
            Box::new(Body::None),
            false,
        )
    }

    pub fn push(&mut self, body: Body) -> Self {
        if let Body::Some(pos, dir, ref mut next, _) = self {
            let next = Box::new(next.push(body));
            *self = Body::Some(*pos, *dir, next, false);
            return self.clone();
        }
        body
    }

    // pub fn len(&self) -> u32 {
    //     if let Body::Some(_, _, ref next, _) = &self {
    //         return 1 + next.len();
    //     }
    //     0
    // }

    pub fn spawn(&self, mut func: impl FnMut(Vec2)) {
        if let Body::Some(pos, _, ref next, _) = *self {
            func(pos);
            next.spawn(func);
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if let Body::Some(_, ref mut dir, _, _) = self {
            *dir = direction;
        }
    }

    pub fn last(&self) -> Self {
        if let Body::Some(_, _, next, _) = self {
            let res = next.last();
            if let Body::Some(_, _, _, _) = res {
                res.last()
            } else {
                self.clone()
            }
        } else {
            self.clone()
        }
    }

    pub fn r#move(&mut self, size: f32) {
        if let Body::Some(ref mut pos, dir, next, ref mut wait) = self {
            let range = Vec2::new(
                BOX_RANGE.x as f32 * BOX_SIZE.x,
                BOX_RANGE.y as f32 * BOX_SIZE.x,
            );

            if *wait {
                *wait = false
            } else {
                match dir {
                    Direction::Up => {
                        if pos.y + size > range.y {
                            pos.y = -(pos.y);
                        } else {
                            pos.y += size;
                        }
                    }
                    Direction::Left => {
                        if pos.x - size < range.x {
                            pos.x = -(pos.x);
                        } else {
                            pos.x -= size;
                        }
                    }
                    Direction::Right => {
                        if pos.x + size > range.y {
                            pos.x = -(pos.x);
                        } else {
                            pos.x += size;
                        }
                    }
                    Direction::Down => {
                        if pos.y - size < range.x {
                            pos.y = -(pos.y);
                        } else {
                            pos.y -= size
                        }
                    }
                }
                next.r#move(size);
                next.set_direction(*dir);
            }
        }
    }

    pub fn position(&self) -> Vec2 {
        if let Body::Some(pos, _, _, _) = &self {
            return *pos;
        }
        Vec2::ZERO
    }

    pub fn direction(&self) -> Direction {
        if let Body::Some(_, dir, _, _) = &self {
            return *dir;
        }
        Direction::Left
    }

    pub fn wait(&mut self) {
        if let Body::Some(_, _, _, ref mut wait) = self {
            *wait = true;
        }
    }

    pub fn collide(&self, fruit_pos: &Vec2) -> bool {
        if let Body::Some(pos, _, next, _) = self {
            let is_collide =
                collide(pos.extend(1.), BOX_SIZE, fruit_pos.extend(1.), BOX_SIZE).is_some();
            return is_collide || next.collide(fruit_pos);
        }
        false
    }

    pub fn collide_self(&self) -> bool {
        if let Body::Some(pos, _, next, _) = self {
            let is_collide = next.collide(pos);
            return is_collide;
        }
        false
    }
}

#[derive(Component)]
pub struct Fruit {
    pub positon: Vec2,
}

impl Fruit {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(BOX_RANGE.x..=BOX_RANGE.y);
        let y = rng.gen_range(BOX_RANGE.x..=BOX_RANGE.y);

        info!("{} {}", x, y);
        Fruit {
            positon: Vec2::new(x as f32 * 50., y as f32 * 50.),
        }
    }
}
