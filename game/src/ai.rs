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
    fn at_destination(&self, pos: Vec2) -> bool{
        let x_diff = pos.x - self.next_vertex.x;
        //will probably need to do something with the y position as well but idk what
        //let y_diff = pos.y - self.next_vertex.y;
        //println!("target: {}",self.next_vertex.x);
        //println!("Current: {}", pos.x);
        //println!("Difference: {}", x_diff.abs());
        if x_diff.abs() <= 32.{
            return true;
        }
        return false;
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
                let mut rng = rand::thread_rng();
                start = rng.gen_range(0,seen_vertices);
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
            self.motion = self.enemy_graph.edges[start][self.next_vertex.id].path;
            self.target_vertex = self.next_vertex.id;
        }
    }
    pub fn decide_motion(&mut self, pos: Vec2, target: usize)-> Motion{

        if self.at_destination(pos) || self.motion == Motion::Stop{
            
            self.choose_target(target);
        }
        return self.motion;
    }
    pub fn update_sight(&mut self, sight: Vec<Line>, obj: Vec<Line>, map_graph: Graph) {
        //this can definitely be done better
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