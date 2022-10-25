use bevy::{prelude::*};
use crate::util::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    //pub obj: i32,
    pub obj: Object,
}


#[derive(Component)]
pub struct Node {
    pub position: Vec2,
    pub node_id: i8,
}

impl Node {
    pub fn new(p: Vec2, i: i8) -> Self {
        Self{
            position: p,
            node_id: i,
        }
    }
}


#[derive(Component)]
pub struct Graph {
    pub node_list: Vec<Vec<Node>>,
}

impl Graph {
    pub fn new(l: Vec<Vec<Node>>) -> Self {
        Self{
            node_list: l,
        }
    }
    pub fn print_graph(&self) {
        println!()
    }
}

impl Line {
    pub fn new(s: Vec2, e: Vec2, o: &Object) -> Self {
        Self {
            start: s,
            end: e,
            obj: *o,
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

fn lines_intersect(a: &Line, b: &Line) -> bool {
    (helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end))
        && (helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}

pub fn find_vertices(x1:f32, y1:f32, x2:f32, y2:f32, width:f32, height:f32) -> (Vec2, Vec2, Vec2){
    
    let v1: Vec2;
    let v2: Vec2;
    //v3 is the third point for the two sides to be used for collision
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
    //MAYBE code for when y's are equal
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
        //MAYBE code for when y's are equal
    }
    return (v1, v2, v3);
}

#[derive(Component)]
pub struct Enemy{
    pub seen_objects: HashSet<Object>,
}

impl Enemy{
    pub fn new() -> Self {
        Self{
            seen_objects: HashSet::new(),
        }
    }
    pub fn check_visible_objects(&self){
        for obj in self.seen_objects.iter(){
            println!("Object with id: {} has been seen by enemy", obj.id);
        }
    }
    pub fn determine_visibility(&mut self, sight: Vec<Line>, obj: Vec<Line>) {
        //this can definitely be done better
        for l in sight.iter() {
            let mut result = true;
            for o in obj.iter() {
                if o.obj == l.obj{
                    if lines_intersect(l, o){
                        result = false;
                        break;
                    }
                }

            }
            if result {
                self.seen_objects.insert(l.obj);
            }
        }

    }
}
