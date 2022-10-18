use bevy::{prelude::*};

pub const PLAYER_SPEED: f32 = 300.;
pub const PLAYER_SZ: f32 = 32.;

#[derive(Component)]
pub struct Velocity {
	pub velocity: Vec2,
}
impl Velocity {
	pub fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

#[derive(Component)]
pub struct ActiveObject {
    pub grounded: bool,
    pub velocity: Velocity,
}

pub enum ActiveType {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Player {
    pub grounded: bool,
}

impl Player{
    pub fn new()-> Self {
        Self {grounded: false}
    }
}
