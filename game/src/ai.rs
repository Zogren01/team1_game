use bevy::{prelude::*};
use crate::util::*;
use crate::line_of_sight::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Node {
    pub position: Vec2,
    pub node_id: i8,
    pub connected_nodes: Vec<Node>,
}

impl Node {
    pub fn new(p: Vec2, i: i8, cn: Vec<Node>,) -> Self {
        Self{
            position: p,
            node_id: i,
            connected_nodes: cn,
            
        }
    }
}

#[derive(Component)]
pub struct Graph {
    pub node_list: Vec<Node>,
}

impl Graph {
    pub fn new(l: Vec<Node>) -> Self {
        Self{
            node_list: l,
        }
    }
    pub fn print_graph(&self) {
        println!()
    }
}

pub enum Motion{
    Left,
    Right,
    Jump,
    Idle,
}

pub enum Action{
    Strafe,
    Attack,
    Retreat,
}

#[derive(Component)]
pub struct Enemy{
    pub seen_objects: HashSet<Descriptor>,
    pub action_type: Action,
    pub next_move: Motion,
    nodes: Vec<Vec2>,
    target: Vec2,
}

impl Enemy{
    pub fn new() -> Self {
        Self{
            //we should track what object player and enemies are standing on
            //to account for moving enemies and player, have a different structure that 
            //updates position when it sees the same object again in a new one
            seen_objects: HashSet::new(),
            action_type: Action::Strafe,
            next_move: Motion::Idle,
            nodes: Vec::new(),
            //this can be improved
            target: Vec2::splat(0.),
        }
    }
    pub fn decide_motion(&mut self, pos: &Vec3){
        match self.action_type{
            Action::Strafe => {
                let mut min = Vec2::splat(9000.);
                let mut max = Vec2::splat(-9000.);
                for node in self.nodes.iter(){
                    if node.y == pos.y{
                        if node.x < min.x{
                            min = *node;
                        }
                        if node.x > max.x{
                            max = *node;
                        }
                    }
                }
                match self.next_move{
                    Motion::Left => {
                        if pos.x <= min.x{
                            self.next_move = Motion::Right;
                        }
                    }
                    Motion::Right => {
                        if pos.x >= max.x{
                            self.next_move = Motion::Left;
                        }
                    }
                    Motion::Jump => {}
                    Motion::Idle => {
                        if pos.x <= min.x{
                            self.next_move = Motion::Right;
                        }
                        else{
                            self.next_move = Motion::Left;
                        }
                    }
                }
            }
            Action::Attack => {}
            Action::Retreat => {}
        }
    }
    pub fn check_visible_objects(&self){
        for obj in self.seen_objects.iter(){
            println!("Object with id: {} has been seen by enemy", obj.id);
        }
    }
    pub fn determine_visibility(&mut self, sight: Vec<Line>, obj: Vec<Line>, height: f32) {
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
                self.nodes.push(Vec2::new(l.obj.x_pos, l.obj.y_pos + l.obj.width/2. + height/2.));
            }
        }

    }
}
