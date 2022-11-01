use bevy::prelude::*;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
pub const TITLE: &str = "Team 1 Game";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const GRAVITY: f32 = -20.;
pub const TERMINAL_VELOCITY: f32 = -500.;
pub const TILE_SIZE: f32 = 32.;

pub const UMBRELLA_PRICE : i8 = 30;
pub const JETPACK_PRICE: i8 = 70;

pub const HEALTHBAR_SZ: Vec2 = Vec2::new(50.,6.);

#[derive(Component, Copy, Clone)]
pub struct Object {
    pub id: i32,
    pub width: f32,
    pub height: f32,
    pub obj_type: ObjectType,
}

impl Object {
    pub fn new(i: i32, w: f32, h: f32, t: ObjectType) -> Self {
        Self {
            id: i,
            width: w,
            height: h,
            obj_type: t,
        }
    }
}

impl Hash for Descriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Descriptor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Descriptor {}


#[derive(Copy, Clone)]
pub struct Descriptor {
    pub width: f32,
    pub height: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub obj_type: ObjectType,
    pub id: i32,
}
impl Descriptor {
    fn new(w: f32, h: f32, x: f32, y: f32, t: ObjectType) -> Self {
        Self {
            width: w,
            height: h,
            x_pos: x,
            y_pos: y,
            obj_type: t,
            id: -50,
        }
    }
    pub fn new2(w: f32, h: f32, x: f32, y: f32, t:ObjectType, i: i32) -> Self {
        Self{
            width: w,
            height: h,
            x_pos: x,
            y_pos: y,
            obj_type: t,
            id: i,
        }  
    }
}

pub fn get_level(id: i8) -> Vec<Descriptor> {
    let mut result = Vec::new();
    //smaller map for testing AI stuff
    if id == 0 {
        result.push(Descriptor::new(512., 32., 16., 0., ObjectType::Block));
    }
    if id == 1 {
        let mut x = 16.;
        let mut y = 0.;
        //main floor
        for i in 0..29{
            result.push(Descriptor::new(64., 32., x, 0., ObjectType::Block));
            x = x+64.;
        }
        //main floor
        //result.push(Descriptor::new(1856., 32., 912., 0., ObjectType::Block));
        //ceiling
        result.push(Descriptor::new(1920., 32., 912., 1024., ObjectType::Block));
        //left wall
        result.push(Descriptor::new(32., 1024., -32., 496., ObjectType::Block));
        //right wall
        result.push(Descriptor::new(32., 1024., 1856., 496., ObjectType::Block));
        //first 1x2 wall on first floor
        result.push(Descriptor::new(32., 64., 192., 48., ObjectType::Block));
        //second 1x2 wall on first floor
        result.push(Descriptor::new(32., 64., 576., 48., ObjectType::Cobweb));
        //2x3 ObjectType::Block on first floor
        result.push(Descriptor::new(64., 92., 1040., 64., ObjectType::Block));
        //last 1x2 ObjectType::Block on first floor
        result.push(Descriptor::new(32., 64., 1312., 48., ObjectType::Block));
        //first floating 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1472., 64., ObjectType::Block));
        //second floating 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1632., 64., ObjectType::Block));
        //grounded 1x1 on first floor
        result.push(Descriptor::new(32., 32., 1728., 32., ObjectType::Block));
        //second floor from left side
        result.push(Descriptor::new(992., 32., 480., 192., ObjectType::Block));
        //second floor from right side
        result.push(Descriptor::new(768., 32., 1456., 192., ObjectType::Block));
        //first 1x2 wall on second floor
        result.push(Descriptor::new(32., 64., 224., 24., ObjectType::Block));
        //2x3 ObjectType::Block on second floor
        result.push(Descriptor::new(64., 96., 848., 256., ObjectType::Block));
        //platform above second floor jutting out from left wall
        result.push(Descriptor::new(192., 32., 80., 352., ObjectType::Block));
        //platform floating above second floor in left-middle
        result.push(Descriptor::new(512., 32., 528., 384., ObjectType::Block));
        //floor of box on upper mid right side
        result.push(Descriptor::new(768., 32., 1456., 352., ObjectType::Block));
        //left wall of box on upper mid right side
        result.push(Descriptor::new(32., 384., 1088., 464., ObjectType::Block));
        //ceiling of box on upper mid right side
        result.push(Descriptor::new(512., 32., 1360., 640., ObjectType::Block));
        //floating platform in box on upper mid side
        result.push(Descriptor::new(512., 32., 1424., 480., ObjectType::Block));
        //1x1 on floating platform in box on upper mid side
        result.push(Descriptor::new(32., 32., 1376., 512., ObjectType::Block));
        //1x1 on right wall in box on upper mid side
        result.push(Descriptor::new(32., 32., 1824., 576., ObjectType::Block));
        //1x1 on right side corner in box on upper mid side
        result.push(Descriptor::new(32., 32., 1824., 384., ObjectType::Block));
        //1x1 on right corder in box on upper mid side
        result.push(Descriptor::new(32., 32., 144., 384., ObjectType::Block));
        //1x1 in left corner of box on upper mid side
        result.push(Descriptor::new(32., 32., 1120., 384., ObjectType::Block));
        //1x2 floating in air
        result.push(Descriptor::new(32., 64., 960., 432., ObjectType::Block));
        //1x3 platform above second floor
        result.push(Descriptor::new(32., 96., 608., 448., ObjectType::Block));
        //wall beside 1x3
        result.push(Descriptor::new(32., 192., 512., 496., ObjectType::Block));
        //floor connected to ^ wall
        result.push(Descriptor::new(512., 32., 240., 576., ObjectType::Block));
        //1x1 sitting on ^ floor
        result.push(Descriptor::new(32., 32., 0., 608., ObjectType::Block));
        //1x1 floating in air below enemy floor
        result.push(Descriptor::new(32., 32., 896., 608., ObjectType::Block));
        //1x1 floating in air above enemy floor
        result.push(Descriptor::new(32., 32., 928., 800., ObjectType::Block));
        //enemy floor
        result.push(Descriptor::new(736., 32., 1472., 768., ObjectType::Block));
        //1x1 on wall below floating 1x1
        result.push(Descriptor::new(32., 32., 1056., 512., ObjectType::Block));
        //floor to room left of enemy floor
        result.push(Descriptor::new(768., 32., 400., 704., ObjectType::Block));
        //1x1 in rooom to left of enemy floor
        result.push(Descriptor::new(32., 32., 448., 736., ObjectType::Block));
        //1x1 on right side of wall in room to left of enemy floor
        result.push(Descriptor::new(32., 32., 320., 862., ObjectType::Block));
        //right wall of room to left of enemy floor
        result.push(Descriptor::new(32., 288., 512., 864., ObjectType::Block));
    }
    // shop platform spawns below level
    result.push(Descriptor::new(250., 32., -150., -500., ObjectType::Block));
    result.push(Descriptor::new(250., 32., 150., -500., ObjectType::Block));
    result.push(Descriptor::new(25., 32., 0., -500., ObjectType::Block));

    return result;
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Block,
    Spike,
    Cobweb,
    Active,
}