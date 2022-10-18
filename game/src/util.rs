use bevy::{prelude::*, window::PresentMode};

pub const TITLE: &str = "Team 1 Game";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const GRAVITY: f32 = -12.;
pub const TERMINAL_VELOCITY: f32 = -500.;
pub const TILE_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Rect {
	pub width: f32,
	pub height: f32,
}

impl Rect {
	pub fn new(w: f32, h: f32) -> Self {
		Self { width: w, height: h}
	}
}

#[derive(Component)]
pub struct Object{
	pub id: i8,
}

impl Object{
	pub fn new(i: i8) -> Self {
		Self { id: i }
	}
}

#[derive(Component)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub obj_id: i8,
}

impl Line {
    pub fn new(s: Vec2, e: Vec2, i: i8) -> Self {
        Self {
            start: s,
            end: e,
            obj_id: i,
        }
    }
    pub fn length_squared(&self) -> f32 {
        (self.end.x - self.start.x) * (self.end.x - self.start.x)
            + (self.end.y - self.start.y) * (self.end.y - self.start.y)
    }
    pub fn print_line(&self) {
        println!(
            "Start: {},{} \n End: {},{} \n",
            self.start.x, self.start.y, self.end.x, self.end.y
        );
    }
}