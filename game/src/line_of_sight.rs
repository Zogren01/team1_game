use bevy::{prelude::*};

#[derive(Component)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub id: usize
}

impl Line {
    pub fn new(s: Vec2, e: Vec2, i: usize) -> Self {
        Self {
            start: s,
            end: e,
            id: i,
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

fn helper(i: Vec2, j: Vec2, k: Vec2) -> bool {
    (k.y - i.y) * (j.x - i.x) > (j.y - i.y) * (k.x - i.x)
}

pub fn lines_intersect(a: &Line, b: &Line) -> bool {
    (helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end))
        && (helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}

pub fn find_vertices(x1:f32, y1:f32, x2:f32, y2:f32, width:f32, height:f32) -> (Vec2, Vec2, Vec2){
    
    let v1: Vec2;
    let v2: Vec2;
    let v3: Vec2;

    if x1 > x2 {
        if y1 >= y2 {
            //top left point
            v1 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
            //bottom right point
            v2 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
            //top right point
            v3 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
        } else {
            //top right point
            v1 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
            //bottom left point
            v2 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
            //bottom right point
            v3 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
        }
    } else {
        if y1 > y2 {
            //top right point
            v1 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
            //bottom left point
            v2 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
            //top left point
            v3 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
        } else {
            //top left point
            v1 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
            //bottom right point
            v2 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
            //bottom left point
            v3 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
        }
    }
    return (v1, v2, v3);
}