use bevy::{prelude::*};

use std::collections::HashSet;

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

pub fn determine_visibility(sight: Vec<Line>, obj: Vec<Line>) {
    println!("Determining objects in view...");

    let mut ids: HashSet<i8> = HashSet::new();
    for l in sight.iter() {
        let mut result = true;
        for o in obj.iter() {
            let intersect = lines_intersect(l, o);
            if l.obj_id == 2 && o.obj_id == 1 {
                l.print_line();
                o.print_line();
            }
            if intersect && (o.obj_id != l.obj_id) {
                result = false;
                break;
            }
        }
        if result {
            ids.insert(l.obj_id);
        }
    }
    for id in ids.iter() {
        println!("Object with id {} is visible", id);
    }
}

fn helper(i: Vec2, j: Vec2, k: Vec2) -> bool {
    (k.y - i.y) * (j.x - i.x) > (j.y - i.y) * (k.x - i.x)
}

fn lines_intersect(a: &Line, b: &Line) -> bool {
    (helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end))
        && (helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}