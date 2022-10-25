use bevy::{prelude::*};
use crate::util::*;
use crate::line_of_sight::*;
use std::collections::HashSet;

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

#[derive(Component)]
pub struct Enemy{
    pub seen_objects: HashSet<Descriptor>,
}

impl Enemy{
    pub fn new() -> Self {
        Self{
            //to account for moving enemies and player, have a different structure that 
            //updates position when it sees the same object again in a new one
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
        //need to get position as well
        //maybe organize the lines so they are grouped by their object? not sure how that would work.
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
