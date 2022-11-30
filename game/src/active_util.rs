use crate::util::*;
use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 5.;
pub const PLAYER_SZ: f32 = 32.;

pub const ENEMY_HEALTH: i32 = 100;

#[derive(Component)]
pub struct ClockText;
#[derive(Component)]
pub struct CreditText;

pub struct Clock {
    pub timer: Timer,
}

/*
impl Clock{
    pub fn new(t: Timer) -> Self {
        Self {
            timer: t,
         }
    }
}
*/

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}
impl Velocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

#[derive(Component)]
pub struct ActiveObject {
    pub grounded: bool,
    pub facing_left: bool,
    pub velocity: Vec2,
    pub max_health: i32,
    pub health: i32,
    pub projected_position: Vec3,
}

impl ActiveObject {
    pub fn new(h: i32, d: i32) -> Self {
        Self {
            grounded: false,
            facing_left: false,
            velocity: Vec2::splat(0.),
            max_health: h,
            health: h,
            projected_position: Vec3::splat(0.),
        }
    }
    pub fn take_damage(&mut self, damage: i32) {
        //implement taking damage
        self.health -= damage;
    }
    pub fn heal(&self, gain: i32) {
        //implement healing
    }
}
#[derive(PartialEq)]
pub enum ItemType {
    None,
    Jetpack,
    Umbrella,
    Boots,
}

#[derive(Component)]
pub struct Player {
    pub credits: i8,
    pub items: Vec<ItemType>,
    pub active_item: usize,
    pub health: i8,
    //temp variable
}

impl Player {
    pub fn new() -> Self {
        Self {
            credits: 100,
            items: vec![ItemType::None],
            active_item: 0,
            health: 100,
        }
    }
}
