use bevy::prelude::*;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;
pub const TITLE: &str = "Team 1 Game";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const GRAVITY: f32 = -12.;
pub const TERMINAL_VELOCITY: f32 = -500.;
pub const TILE_SIZE: f32 = 32.;

#[derive(Component, Copy, Clone)]
pub struct Object{
	pub id: i32,
    pub width: f32,
    pub height: f32,
}

impl Object{
	pub fn new(i: i32, w: f32, h: f32,) -> Self {
		Self { 
            id: i,
            width: w,
            height: h,
        }
	}
}

impl Hash for Object{

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Object {}

pub struct Descriptor{
    pub width: f32,
    pub height: f32,
    pub x_pos: f32,
    pub y_pos: f32,
}
impl Descriptor{
    fn new(w:f32, h:f32, x:f32, y:f32,) -> Self{
        Self{
            width: w, 
            height: h, 
            x_pos: x, 
            y_pos: y,
        } 
    }
}

pub fn get_level(id: i8) -> Vec<Descriptor>{
    let mut result = Vec::new();
    if id == 1 {
        //main floor
        result.push(Descriptor::new(1856., 32., 912., 0.,));
        //ceiling
        result.push(Descriptor::new(1920., 32., 912., 1024.,));
        //left wall
        result.push(Descriptor::new(32., 1024., -32., 496.,));
        //right wall
        result.push(Descriptor::new(32., 1024., 1856., 496.,));
        //first 1x2 wall on first floor
        result.push(Descriptor::new(32., 64., 192., 48.,));
        //second 1x2 wall on first floor
        result.push(Descriptor::new(32., 64., 576., 48.,));
        //2x3 block on first floor
        result.push(Descriptor::new(64., 92., 1040., 64.,));
        //last 1x2 block on first floor
        result.push(Descriptor::new(32., 64., 1312., 48.,));
        //first floating 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1472., 64.,));
        //second floating 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1632., 64.,));
        //grounded 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1728., 32.,));
        //second floor from left side
        result.push(Descriptor::new(992., 32., 480., 192.,));
        //second floor from right side
        result.push(Descriptor::new(768., 32., 1456., 192.,));
        //first 1x2 wall on second floor
        result.push(Descriptor::new(32., 64., 224., 24.,));
        //2x3 block on second floor
        result.push(Descriptor::new(64., 96., 848., 256.,));
        //platform above second floor jutting out from left wall
        result.push(Descriptor::new(192., 32., 80., 352.,));
        //platform floating above second floor in left-middle
        result.push(Descriptor::new(512., 32., 528., 384.,));
        //floor of box on upper mid right side
        result.push(Descriptor::new(768., 32., 1456., 352.,));
        //left wall of box on upper mid right side
        result.push(Descriptor::new(32., 384., 1088., 464.,));
        //ceiling of box on upper mid right side
        result.push(Descriptor::new(512., 32., 1360., 640.,));
        //floating platform in box on upper mid side
        result.push(Descriptor::new(512., 32., 1424., 480.,));
        //1x1 on floating platform in box on upper mid side
        result.push(Descriptor::new(32., 32., 1376., 512.,));
        //1x1 on right wall in box on upper mid side
        result.push(Descriptor::new(32., 32., 1824., 576.,));
        //1x1 on right side corner in box on upper mid side
        result.push(Descriptor::new(32., 32., 1824., 384.,));
        //1x1 on right corder in box on upper mid side
        result.push(Descriptor::new(32., 32., 144., 384.,));
        //1x1 in left corner of box on upper mid side
        result.push(Descriptor::new(32., 32., 1120., 384.,));
        //1x2 floating in air
        result.push(Descriptor::new(32., 64., 960., 432.,));
        //1x3 platform above second floor
        result.push(Descriptor::new(32., 96., 608., 448.,));
        //wall beside 1x3
        result.push(Descriptor::new(32., 192., 512., 496.,));
        //floor connected to ^ wall
        result.push(Descriptor::new(512., 32., 240., 576.,));
        //1x1 sitting on ^ floor
        result.push(Descriptor::new(32., 32., 0., 608.,));
        //1x1 floating in air below enemy floor
        result.push(Descriptor::new(32., 32., 896., 608.,));
        //1x1 floating in air above enemy floor
        result.push(Descriptor::new(32., 32., 928., 800.,));
        //enemy floor
        result.push(Descriptor::new(736., 32., 1472., 768.,));
        //1x1 on wall below floating 1x1
        result.push(Descriptor::new(32., 32., 1056., 512.,));
        //floor to room left of enemy floor
        result.push(Descriptor::new(768., 32., 400., 704.,));
        //1x1 in rooom to left of enemy floor
        result.push(Descriptor::new(32., 32., 448., 736.,));
        //1x1 on right side of wall in room to left of enemy floor
        result.push(Descriptor::new(32., 32., 320., 862.,));
        //right wall of room to left of enemy floor
        result.push(Descriptor::new(32., 288., 512., 864.,));
    }
    return result;
}


