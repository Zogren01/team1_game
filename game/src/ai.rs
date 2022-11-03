use bevy::{prelude::*};
use crate::movement_mesh::*;
use crate::line_of_sight::*;
use rand::Rng;


#[derive(Component)]
pub struct Enemy{
    pub enemy_graph: Graph,
    pub next_vertex: Vertex, //the next vertex for the enemy to reach
    //some variable "path" to track the enemies target path
    pub target_vertex: usize, //the end goal of the enemies motion, usize is used because next vertex can be used to check the position
    pub motion: Motion,
}

impl Enemy{
    pub fn new(x: f32, y: f32) -> Self {
        Self{
            enemy_graph: Graph::new(),
            next_vertex: Vertex::new(x, y, 51),
            target_vertex: 51,
            motion: Motion::Stop,
        }
    }
    //updates enemy motion type if they are at or 
    fn at_destination(&mut self, pos: Vec2){
        let x_diff = pos.x - self.next_vertex.x;
        let y_diff = pos.y - self.next_vertex.y;
        //println!("target: {}",self.next_vertex.x);
        //println!("Current: {}", pos.x);
        //println!("Difference: {}", x_diff.abs());
        //println!("target: {}",self.next_vertex.y);
        //println!("Current: {}", pos.y);
        //println!("Difference: {}", y_diff.abs());
        if x_diff.abs() <= 1.{
            if y_diff.abs() <= 1.  {
                self.motion = Motion::Stop;
            }
            else {
                self.motion = Motion::Fall;
            }
        }
    }
    //for now, the target will be passed from the user
    fn choose_target(&mut self, target: usize){
        /*
        this is where pathfinding and enemy decisionmaking will occur
        a list (or some other structure) of what vertices must be traversed will be created
        this will be called every time a destination is reached, or the target should be changed
        */

        //51 is a placeholder for when an enemy is spawned in (maybe spawn in with a first vertex?)
        let start: usize;
        if target == 51{
            self.motion = Motion::Stop;
            return;
        }
        if self.next_vertex.id == 51{
            let seen_vertices = self.enemy_graph.vertices.len();
            if seen_vertices > 0{
                start = 0;
            }
            else{
                return;
            }
        }
        else{
            start = self.next_vertex.id;
        }
        let mut found = false;
        for vertex in self.enemy_graph.vertices.iter_mut(){
            if vertex.id == target{
                found = true;
                self.next_vertex = *vertex;
                break;
            }
        }
        if found{
            println!("travelling from {} to {}", start, self.next_vertex.id);
            self.motion = self.enemy_graph.edges[start][self.next_vertex.id].path;
            self.target_vertex = self.next_vertex.id;
        }
    }
    pub fn decide_motion(&mut self, pos: Vec2, target: usize)-> Motion{
        if self.target_vertex != 51{
            self.at_destination(pos);
        }
        if self.motion == Motion::Stop{
            if target == 1{
                println!("target 1 selected");
            }
            self.choose_target(target);
        }
        return self.motion;
    }
    pub fn update_sight(&mut self, sight: Vec<Line>, obj: Vec<Line>, map_graph: Graph) {

        for l in sight.iter() {
            let mut result = true;
            for o in obj.iter() {
                if lines_intersect(l, o){
                    result = false;
                    break;
                }
            }
            if result{

                let vertex = Vertex::new(l.end.x, l.end.y, l.id);
                let mut seen_before = false;
                for seen_vertex in self.enemy_graph.vertices.iter_mut(){
                    if seen_vertex.id == vertex.id{
                        seen_before = true;
                    }
                    self.enemy_graph.edges[seen_vertex.id][vertex.id] = map_graph.edges[seen_vertex.id][vertex.id];
                    self.enemy_graph.edges[vertex.id][seen_vertex.id] = map_graph.edges[vertex.id][seen_vertex.id];
                }
                if !seen_before{
                    self.enemy_graph.vertices.push(vertex);
                }
            }
        }

    }
}