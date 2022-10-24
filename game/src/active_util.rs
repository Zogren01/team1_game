use bevy::{prelude::*};

pub const PLAYER_SPEED: f32 = 300.;
pub const PLAYER_SZ: f32 = 32.;


#[derive(Component)]
pub struct ClockText;


pub struct Clock{
    pub timer: Timer,
}

impl Clock{
    pub fn new(t: Timer) -> Self {
        Self {
            timer: t,
         }
    }
}

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
    pub facing_left: bool,
    pub velocity: Vec2,
    pub max_health: i32,
    pub health: i32,
    pub damage: i32,
    pub projected_position: Vec3,
}

impl ActiveObject {
    pub fn new(h: i32, d: i32) -> Self{
        Self {
            grounded: false, 
            facing_left: false,
            velocity: Vec2::splat(0.),
            max_health: h,
            health: h,
            damage: d,
            projected_position: Vec3::splat(0.),
        }
    }
    pub fn take_damage(&self, damage: i32){
        //implement taking damage
    }
    pub fn heal(&self, gain: i32) {
        //implement healing
    }
}

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Enemy;
