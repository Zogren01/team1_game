use crate::physics::*;
use bevy::prelude::*;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
pub const TITLE: &str = "Team 1 Game";
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;
pub const MAP_W: f32 = 1920.;
pub const MAP_H: f32 = 1056.;

pub const GRAVITY: f32 = -0.5;
pub const TERMINAL_VELOCITY: f32 = -500.;
pub const TILE_SIZE: f32 = 32.;
pub const UMBRELLA_VELOCITY: f32 = -0.75;

pub const UMBRELLA_PRICE: i8 = 30;
pub const JETPACK_PRICE: i8 = 70;
pub const BOOTS_PRICE: i8 = 30;

pub const ATTACK_HITBOX: Vec2 = Vec2::new(32., 16.,);
pub const HEALTHBAR_SZ: Vec2 = Vec2::new(50., 6.);

#[derive(Component, Copy, Clone)]
pub struct Object {
    pub id: i32,
    pub width: f32,
    pub height: f32,
    pub obj_type: ObjectType,
    pub velocity: Vec2,
}

impl Object {
    pub fn new(i: i32, w: f32, h: f32, t: ObjectType) -> Self {
        Self {
            id: i,
            width: w,
            height: h,
            obj_type: t,
            velocity: Vec2::splat(0.0),
           // project_pos: Vec3::splat(0.),

        }
    }
}

#[derive(Component)]
pub struct Explosive {
    pub lifespan: Timer,
}

impl Explosive {
    pub fn new(lifespan: Timer) -> Self {
        Self { lifespan: lifespan }
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
            width: w * 32.,
            height: h * 32.,
            x_pos: x * 32.,
            y_pos: y * 32.,
            obj_type: t,
            id: -50,
        }
    }
    pub fn new2(w: f32, h: f32, x: f32, y: f32, t: ObjectType, i: i32) -> Self {
        Self {
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
        result.push(Descriptor::new(1., 2., 8., 2., ObjectType::Block));
        result.push(Descriptor::new(16., 1., 0.5, 0., ObjectType::Block));
    }
    if id == 1 {
        //ceiling
        result.push(Descriptor::new(60., 1., 0., 16.5, ObjectType::Block));
        //starting platform
        result.push(Descriptor::new(28., 1., 0., 9.5, ObjectType::Block));
        //small 1x2s on left and right
        result.push(Descriptor::new(1., 2., -6.5, 11., ObjectType::Block));
        result.push(Descriptor::new(1., 2., 6.5, 11., ObjectType::Block));
        //breakable objects on left and right
        //result.push(Descriptor::new(1., 4., -11.5, 12., ObjectType::Breakable));
        //result.push(Descriptor::new(1., 4., 11.5, 12., ObjectType::Breakable));
        //block under main panel
        result.push(Descriptor::new(1., 1., -13.5, 8.5, ObjectType::Block));
        result.push(Descriptor::new(1., 1., 13.5, 8.5, ObjectType::Block));
        //ledges under main panel
        result.push(Descriptor::new(6., 1., -16., 7.5, ObjectType::Block));
        result.push(Descriptor::new(6., 1., 16., 7.5, ObjectType::Block));
        //smaller lowered panels
        result.push(Descriptor::new(4., 1., -24., 5.5, ObjectType::Block));
        result.push(Descriptor::new(4., 1., 24., 5.5, ObjectType::Block));
        //panels with enemimes on them
        result.push(Descriptor::new(5., 1., -22.5, 10.5, ObjectType::Block));
        result.push(Descriptor::new(5., 1., 22.5, 10.5, ObjectType::Block));
        //enemies
        //result.push(Descriptor::new(1., 1., -22.5, 11.5, ObjectType::Enemy));
        result.push(Descriptor::new(1., 1., 22.5, 11.5, ObjectType::Enemy));
        //left and right floors
        result.push(Descriptor::new(27., 1., -16.5, 2.5, ObjectType::Block));
        result.push(Descriptor::new(27., 1., 16.5, 2.5, ObjectType::Block));
        result.push(Descriptor::new(4., 1., 0., 0.5, ObjectType::Block));
        //left and right walls
        result.push(Descriptor::new(1., 23., -29.5, 5.5, ObjectType::Block));
        result.push(Descriptor::new(1., 23., 29.5, 5.5, ObjectType::Block));
        //bottom floor
        result.push(Descriptor::new(60., 5., 0., -14.5, ObjectType::Block));
        //block on bottom floor
        result.push(Descriptor::new(18., 4., 0., -10., ObjectType::Block));
        //innermost bottom pillars
        result.push(Descriptor::new(1., 2., -13.5, -11., ObjectType::Block));
        result.push(Descriptor::new(1., 2., 13.5, -11., ObjectType::Block));
        //middle bottom pillars
        result.push(Descriptor::new(1., 4., -16.5, -10., ObjectType::Block));
        result.push(Descriptor::new(1., 4., 16.5, -10., ObjectType::Block));
        //outer bottom pillars
        result.push(Descriptor::new(1., 2., -19.5, -11., ObjectType::Block));
        result.push(Descriptor::new(1., 2., 19.5, -11., ObjectType::Block));
        //inner floating blocks
        result.push(Descriptor::new(1., 1., -11.5, -5.5, ObjectType::Block));
        result.push(Descriptor::new(1., 1., 11.5, -5.5, ObjectType::Block));
        //outer floating blocks
        result.push(Descriptor::new(1., 1., -14.5, -2.5, ObjectType::Block));
        result.push(Descriptor::new(1., 1., 14.5, -2.5, ObjectType::Block));
        //top panels
        result.push(Descriptor::new(7., 1., -19.5, -5.5, ObjectType::Block));
        result.push(Descriptor::new(7., 1., 19.5, -5.5, ObjectType::Block));
        //small walls
        result.push(Descriptor::new(1., 6., -22.5, -9., ObjectType::Block));
        result.push(Descriptor::new(1., 6., 22.5, -9., ObjectType::Block));
        //pillars on panels
        result.push(Descriptor::new(1., 2., -22.5, -4., ObjectType::Block));
        result.push(Descriptor::new(1., 2., 22.5, -4., ObjectType::Block));
    }
    if id == 2 {
        let mut x = 16.;
        let mut y = 0.;
        //main floor
        for i in 0..29 {
            result.push(Descriptor::new(2., 1., x / 32., 0., ObjectType::Block));
            x = x + 64.;
        }
        //main floor
        //result.push(Descriptor::new(1856., 32., 912., 0., ObjectType::Block));
        //ceiling
        result.push(Descriptor::new(60., 1., 28.5, 32., ObjectType::Block));
        //left wall
        result.push(Descriptor::new(1., 32., -1., 15.5, ObjectType::Block));
        //right wall
        result.push(Descriptor::new(1., 32., 58., 15.5, ObjectType::Item));
        //first 1x2 wall on first floor
        result.push(Descriptor::new(1., 2., 6., 1.5, ObjectType::Breakable));
        //second 1x2 wall on first floor
        result.push(Descriptor::new(1., 2., 18., 1.5, ObjectType::Cobweb));
        //2x3 ObjectType::Block on first floor
        result.push(Descriptor::new(2., 2.875, 32.5, 2., ObjectType::Block));
        //last 1x2 ObjectType::Block on first floor
        result.push(Descriptor::new(1., 2., 41., 1.5, ObjectType::Block));
        //first floating 1x1 on first floor
        result.push(Descriptor::new(1., 1., 46., 2., ObjectType::Block));
        //second floating 1x1 on first floor
        result.push(Descriptor::new(1., 1., 51., 2., ObjectType::Block));
        //grounded 1x1 on first floor
        result.push(Descriptor::new(1., 1., 54., 1., ObjectType::Block));
        //second floor from left side
        result.push(Descriptor::new(31., 1., 15., 6., ObjectType::Block));
        //second floor from right side
        result.push(Descriptor::new(24., 1., 45.5, 6., ObjectType::Block));
        //first 1x2 wall on second floor
        // result.push(Descriptor::new(1., 2., 7., 0.75, ObjectType::Block));
        //2x3 ObjectType::Block on second floor
        result.push(Descriptor::new(2., 3., 26.5, 8., ObjectType::Block));
        //platform above second floor jutting out from left wall
        result.push(Descriptor::new(6., 1., 2.5, 11., ObjectType::Block));
        //platform floating above second floor in left-middle
        result.push(Descriptor::new(16., 1., 16.5, 12., ObjectType::Block));
        //floor of box on upper mid right side
        result.push(Descriptor::new(24., 1., 45.5, 11., ObjectType::Block));
        //left wall of box on upper mid right side
        result.push(Descriptor::new(1., 12., 34., 14.5, ObjectType::Block));
        //ceiling of box on upper mid right side
        result.push(Descriptor::new(16., 1., 42.5, 20., ObjectType::Block));
        //floating platform in box on upper mid side
        result.push(Descriptor::new(16., 1., 44.5, 15., ObjectType::Block));
        //1x1 on floating platform in box on upper mid side
        result.push(Descriptor::new(1., 1., 43., 16., ObjectType::Block));
        //1x1 on right wall in box on upper mid side
        result.push(Descriptor::new(1., 1., 57., 18., ObjectType::Block));
        //1x1 on right side corner in box on upper mid side
        result.push(Descriptor::new(1., 1., 57., 12., ObjectType::Block));
        //1x1 on right corder in box on upper mid side
        result.push(Descriptor::new(1., 1., 4.5, 12., ObjectType::Block));
        //1x1 in left corner of box on upper mid side
        result.push(Descriptor::new(1., 1., 35., 12., ObjectType::Block));
        //1x2 floating in air
        result.push(Descriptor::new(1., 2., 30., 13.5, ObjectType::Block));
        //1x3 platform above second floor
        result.push(Descriptor::new(1., 3., 19., 14., ObjectType::Block));
        //wall beside 1x3
        result.push(Descriptor::new(1., 6., 16., 15.5, ObjectType::Block));
        //floor connected to ^ wall
        result.push(Descriptor::new(16., 1., 7.5, 18., ObjectType::Block));
        //1x1 sitting on ^ floor
        result.push(Descriptor::new(1., 1., 0., 19., ObjectType::Block));
        //1x1 floating in air below enemy floor
        result.push(Descriptor::new(1., 1., 28., 19., ObjectType::Block));
        //1x1 floating in air above enemy floor
        result.push(Descriptor::new(1., 1., 29., 25., ObjectType::Block));
        //enemy floor
        result.push(Descriptor::new(23., 1., 46., 24., ObjectType::Block));
        //1x1 on wall below floating 1x1
        result.push(Descriptor::new(1., 1., 33., 16., ObjectType::Block));
        //floor to room left of enemy floor
        result.push(Descriptor::new(24., 1., 12.5, 22., ObjectType::Block));
        //1x1 in rooom to left of enemy floor
        result.push(Descriptor::new(1., 1., 14., 23., ObjectType::Block));
        //1x1 on right side of wall in room to left of enemy floor
        result.push(Descriptor::new(1., 1., 10., 26., ObjectType::Block));
        //right wall of room to left of enemy floor
        result.push(Descriptor::new(1., 9., 16., 27., ObjectType::Block));

        result.push(Descriptor::new(1., 3., 7., 7., ObjectType::Block));

        result.push(Descriptor::new(1., 6., 9., 25.5, ObjectType::Block));

        result.push(Descriptor::new(1., 1., 8., 25., ObjectType::Block));

        result.push(Descriptor::new(1., 1., 45., 12., ObjectType::Block));

        result.push(Descriptor::new(1., 1., 0., -27., ObjectType::Block));
    }

    if id == 7 {
        //spawn hallway
        result.push(Descriptor::new(40., 1., 19., 0., ObjectType::Block)); //spawn floor
        result.push(Descriptor::new(1., 32., -1., 0., ObjectType::Block)); //spawn left wall
        result.push(Descriptor::new(32., 1., 15., 6., ObjectType::Block)); //spawn ceiling

        result.push(Descriptor::new(1., 4., 31., 1.5, ObjectType::Block)); //first obstacle
        result.push(Descriptor::new(1., 4., 32., 1.5, ObjectType::Block));

        result.push(Descriptor::new(1., 20., 30.5, 16.5, ObjectType::Block)); //left side upper wall + exit

        //upper level
        result.push(Descriptor::new(60., 1., 64., 6., ObjectType::Block)); //upper floor

        result.push(Descriptor::new(1., 4., 54., 7.5, ObjectType::Block)); //obstacle
        result.push(Descriptor::new(1., 4., 55., 7.5, ObjectType::Block));

        result.push(Descriptor::new(2., 1., 32., 11.5, ObjectType::Block)); //platform for ranged enemy

        result.push(Descriptor::new(1., 6., 66., 8.5, ObjectType::Block)); //obstacle at end of platform
        result.push(Descriptor::new(1., 6., 67., 8.5, ObjectType::Block));
        result.push(Descriptor::new(1., 4., 68., 7.5, ObjectType::Block));
        result.push(Descriptor::new(1., 2., 69., 6.5, ObjectType::Block));

        result.push(Descriptor::new(1., 20., 73.5, 16., ObjectType::Block)); //right wall

        //lower level
        result.push(Descriptor::new(1., 6., 39., -2.5, ObjectType::Block)); //stairs downwards
        result.push(Descriptor::new(1., 6., 40., -2.5, ObjectType::Block));
        result.push(Descriptor::new(1., 4., 41., -3.5, ObjectType::Block));
        result.push(Descriptor::new(1., 4., 42., -3.5, ObjectType::Block));
        result.push(Descriptor::new(1., 2., 43., -4.5, ObjectType::Block));
        result.push(Descriptor::new(1., 2., 44., -4.5, ObjectType::Block));

        result.push(Descriptor::new(10., 1., 49., -5., ObjectType::Block)); //floor extending from stairs

        result.push(Descriptor::new(37., 1., 82., -5., ObjectType::Block)); //floor for enemy across

        result.push(Descriptor::new(70., 1., 69.5, -20., ObjectType::Block)); //bottom floor

        result.push(Descriptor::new(1., 12., 104., -13.5, ObjectType::Block)); //stairs upwards
        result.push(Descriptor::new(1., 12., 103., -13.5, ObjectType::Block));
        result.push(Descriptor::new(1., 12., 102., -14.5, ObjectType::Block));
        result.push(Descriptor::new(1., 12., 101., -14.5, ObjectType::Block));
        result.push(Descriptor::new(1., 10., 100., -15.5, ObjectType::Block));
        result.push(Descriptor::new(1., 10., 99., -15.5, ObjectType::Block));
        result.push(Descriptor::new(1., 8., 98., -16.5, ObjectType::Block));
        result.push(Descriptor::new(1., 8., 97., -16.5, ObjectType::Block));
        result.push(Descriptor::new(1., 6., 96., -17.5, ObjectType::Block));
        result.push(Descriptor::new(1., 6., 95., -17.5, ObjectType::Block));
        result.push(Descriptor::new(1., 4., 94., -18.5, ObjectType::Block));
        result.push(Descriptor::new(1., 4., 93., -18.5, ObjectType::Block));
        result.push(Descriptor::new(1., 2., 92., -19.5, ObjectType::Block));
        result.push(Descriptor::new(1., 2., 91., -19.5, ObjectType::Block));

        result.push(Descriptor::new(1., 20., 35., -10., ObjectType::Block)); //bottom left wall

        result.push(Descriptor::new(1., 40., 105., -0.5, ObjectType::Block)); //bottom right wall

        result.push(Descriptor::new(1., 3., 62., -4., ObjectType::Block)); //obstacle for ranged enemy
        result.push(Descriptor::new(1., 3., 63., -4., ObjectType::Block));

        result.push(Descriptor::new(1., 4., 58., -18.5, ObjectType::Block)); //bottom floor obstacle
        result.push(Descriptor::new(1., 4., 59., -18.5, ObjectType::Block));

        result.push(Descriptor::new(2., 1., 36., -10., ObjectType::Block)); //platform for item

        //top level
        result.push(Descriptor::new(32., 1., 47., 20., ObjectType::Block)); //top floor
    }

    // shop platform spawns below level
    result.push(Descriptor::new(32., 1., 0., -32., ObjectType::Block)); // shop box code start
    result.push(Descriptor::new(32., 1., 0., -21., ObjectType::Block));
    result.push(Descriptor::new(1., 12., 16., -26.5, ObjectType::Block));
    result.push(Descriptor::new(1., 12., -16., -26.5, ObjectType::Block)); // shop box code end

    result.push(Descriptor::new(8., 1., 12., -29., ObjectType::Block)); // platform to hold umbrella
    result.push(Descriptor::new(8., 1., -12., -29., ObjectType::Block)); // platform to hold another item
    result.push(Descriptor::new(8., 1., 0., -26., ObjectType::Block)); // platform to hold jet pack

    result.push(Descriptor::new(1., 2., 0., -7., ObjectType::Barrel)); // platform to hold another item
    result.push(Descriptor::new(1., 2., 5., -7., ObjectType::Barrel)); // platform to hold another item
    
    result.push(Descriptor::new(1., 2., 2., -20., ObjectType::Breakable)); // platform to hold another item
    result.push(Descriptor::new(1., 2., 4., -20., ObjectType::Breakable)); // platform to hold another item

    result.push(Descriptor::new(
        1.,
        1.,
        -12.,
        -28.,
        ObjectType::UmbrellaItem,
    )); // placeholders for landmarker items start
    result.push(Descriptor::new(1., 1., 12., -28., ObjectType::Item)); // Eventually you can buy the items through these boxes
    result.push(Descriptor::new(1., 1., 0., -25., ObjectType::JetpackItem)); // placeholders for landmaker items end

    return result;
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Block,
    Spike,
    Cobweb,
    Active,
    Enemy, //enemy and player types will be necessary for enemy line of sight
    Player,
    Item,
    UmbrellaItem,
    JetpackItem,
    Bullet,
    Breakable,
    Barrel,
    Credit,
}
