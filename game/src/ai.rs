use bevy::{prelude::*};
use crate::movement_mesh::*;
use crate::line_of_sight::*;
use crate::active_util::*;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Path {
    vertices: Vec<usize>,
    weight: usize,
}
impl Path {
    pub fn new() -> Self{
        Self{
            vertices: Vec::new(),
            weight: usize::MAX,
        } 
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub enum Type {
    Melee,
    Ranged,
}

pub enum Action {
    Reset,
    Strafe,
    Chase,
    Attack,
    Retreat,
}

pub enum Attack {
    Up,
    Down,
    Left,
    Right,
    None,
    Projectile,
    Melee
}

#[derive(Component)]
pub struct Enemy{
    pub t: Type,
    pub enemy_graph: Graph, 
    pub next_vertex: usize,
    pub current_vertex: usize,
    pub target_vertex: usize, //the end goal of the enemies motion, usize is used because next vertex can be used to check the position
    pub path: Path,
    pub index_in_path: usize,
    pub motion: Motion,
    pub action: Action,
    pub player_seen: bool,
    pub player_pos: Vec2,
    pub old_pos: Vec2,
    pub immobile_frames: usize,
    pub attack: Attack,
    pub recover_health: bool,
}

impl Enemy{
    pub fn new(ty: Type) -> Self {
        Self{
            t: ty,
            //supgraph of the movement mesh that has been seen by enemy
            enemy_graph: Graph::new(),
            //enemy starts at it's first "target" vertex
            next_vertex: MAX_VERT+1,
            current_vertex: MAX_VERT+1,
            target_vertex: MAX_VERT+1,
            //path tracks the vertices to get to target
            path: Path::new(),
            index_in_path: 0,
            //motion is current motion, action is current method for decisionmaking
            motion: Motion::Stop,
            action: Action::Strafe,
            //player_seen is true when player is in sight, and position tracks where the player is if possible
            player_seen: false,
            player_pos: Vec2::splat(f32::MAX),
            old_pos: Vec2::splat(f32::MAX),
            immobile_frames: 0,
            attack: Attack::None,
            recover_health: false,
        }
    }
    pub fn decide_motion(&mut self, pos: Vec2)-> Motion{
        //only update motion if enemy has seen at least one vertex
        self.attack = Attack::None;
        self.recover_health = false;
        //println!("{}", health);
        if self.enemy_graph.vertices.len() > 0 {
            let dist_to_player = distance_squared(pos.x, pos.y, self.player_pos.x, self.player_pos.y);
            if pos == self.old_pos{
                self.immobile_frames += 1;
            }
            //first check is for if player should be attacked
            if self.player_seen && dist_to_player < 10000.{
                self.action = Action::Attack;
            }
            //will need a new check for if enemy is stuck
            else if self.immobile_frames >= 3 || self.current_vertex == MAX_VERT + 1 || matches!(self.action, Action::Attack){
                self.immobile_frames = 0;
                self.current_vertex = MAX_VERT + 1;
                self.action = Action::Reset;
            }
            else if self.player_seen{
                self.action = Action::Chase
            }
            else{
                self.action = Action::Strafe;
            }
            self.update_motion(pos);
        }
        self.old_pos = pos;
        return self.motion;
    }

     //updates enemy motion type if they are at or 
     fn update_motion(&mut self, pos: Vec2){

        match self.action{
            Action::Reset => {

                let mut x_dist = f32::MAX;
                let mut best_vert = MAX_VERT+1;
                for v in self.enemy_graph.vertices.iter_mut() {
                    if v.y <= pos.y + 5. && v.y >= pos.y - 5.{
                        let diff = v.x - pos.x;
                        if diff.abs() < x_dist.abs(){
                            x_dist = diff;
                            best_vert = v.id;
                        }
                    }
                }
                if best_vert == MAX_VERT+1{
                    self.motion = Motion::Fall;
                }
                else{
                    self.next_vertex = best_vert;
                    self.target_vertex = best_vert;
                    if x_dist >= 5. {
                        self.motion = Motion::Right;
                    }
                    else if x_dist <= -5.{
                        self.motion = Motion::Left;
                    }
                    else{
                        self.current_vertex = self.next_vertex;
                    }
                }
            }
            Action::Strafe => {
                //println!("Strafe update");
                let mut x_diff = f32::MAX;
                let mut y_diff = f32::MAX;
                //find the difference in enemies position to the next vertex on the enemies path
                //needed to determine if the enemy is "at" their destination
                for v in self.enemy_graph.vertices.iter_mut() {
                    if v.id == self.next_vertex {
                        x_diff = pos.x - v.x;
                        y_diff = pos.y - v.y;
                        break;
                    }
                }
                if x_diff.abs() <= 5.{
                    if y_diff.abs() <= 5.  {

                        self.current_vertex = self.next_vertex;

                        if self.current_vertex == self.target_vertex{
                            //randomly select a seen vertex
                            let r = self.enemy_graph.vertices.len();
                            let mut rng = rand::thread_rng();

                            let pos: usize = rng.gen_range(0, r);
                            self.target_vertex = self.enemy_graph.vertices[pos].id;
                            //update path to be the path to that vertex
                            self.path = self.shortest_path();
                            self.index_in_path = 0;
                        }
                        //otherwise, destination is not reached
                        else{
                            self.index_in_path += 1;
                        }
                        if self.path.vertices.len() > self.index_in_path{
                            self.next_vertex = self.path.vertices[self.index_in_path];
                            self.motion = self.enemy_graph.edges[self.current_vertex][self.next_vertex].path; 
                        }
                        else{
                            println!("shouldn't get stuck here");
                        }
                    }
                    else{
                        //x position is correct but enemy is still falling to destination
                        self.motion = Motion::Fall;
                    }
                }
            }
            Action::Retreat => {
                println!("Retreat update");
            }
            Action::Chase => {
                //println!("Chase update");
                //check that the current target vertex is still the closest one to the player
                let mut x_diff = f32::MAX;
                let mut y_diff = f32::MAX;
                //find the difference in enemies position to the next vertex on the enemies path
                //needed to determine if the enemy is "at" their destination
                for v in self.enemy_graph.vertices.iter_mut() {
                    if v.id == self.next_vertex {
                        x_diff = pos.x - v.x;
                        y_diff = pos.y - v.y;
                        break;
                    }
                }
                if x_diff.abs() <= 5.{
                    if y_diff.abs() <= 5.  {
                        self.current_vertex = self.next_vertex;
                        let pl_vert = self.nearest_vert(self.player_pos);
                        //if a better vertex is found or the enemy has arrived (second one shouldn't ever happen)
                        if pl_vert != self.target_vertex || self.current_vertex == self.target_vertex{

                            self.target_vertex = pl_vert;
                            self.path = self.shortest_path();
                            self.index_in_path = 0;
                        }
                        else{
                            self.index_in_path += 1;
                        }
                        if self.path.vertices.len() > self.index_in_path{
                            self.next_vertex = self.path.vertices[self.index_in_path];
                            self.motion = self.enemy_graph.edges[self.current_vertex][self.next_vertex].path; 
                        }
                        else{
                            println!("shouldn't get stuck here");
                        }
                    }
                    else {
                        //x position is correct but enemy is still falling to destination
                        self.motion = Motion::Fall;
                    }
                }
            }
            Action::Attack =>{
                //println!("Attack update");
                let x_to_player = pos.x - self.player_pos.x;
                let y_to_player = pos.y - self.player_pos.y;
                if x_to_player.abs() <= PLAYER_SZ{
                    if y_to_player.abs() <= PLAYER_SZ{
                        //within range to attack
                        self.motion = Motion::Stop;
                        if x_to_player.abs() > y_to_player.abs(){
                            if x_to_player > 0.{
                                self.attack = Attack::Left;
                            }
                            else{
                                self.attack = Attack::Right;
                            }
                        }
                        else{
                            if y_to_player > 0.{
                                self.attack = Attack::Down;
                            }
                            else{
                                self.attack = Attack::Up;
                            }
                        }
                    }
                    //below player
                    else{
                        self.motion = Motion::Jump;
                    }
                }
                else{
                    if x_to_player > 0.{
                        self.motion = Motion::Left;
                    }
                    else{
                        self.motion = Motion::Right;
                    }
                }
            }
        }           
    }

    fn nearest_vert(&self, pos: Vec2) -> usize{
        
        let mut distance = f32::MAX;
        let mut result: usize = MAX_VERT+1; 
        for v in self.enemy_graph.vertices.iter() {
            let curr =  distance_squared(pos.x, pos.y, v.x, v.y);
            if curr < distance{
                distance = curr;
                result = v.id;
            }
        }
        return result;
    }

    fn farthest_vert(&self, pos: Vec2) -> usize{
        let mut distance = 0.;
        let mut result: usize = MAX_VERT + 1;
        for v in self.enemy_graph.vertices.iter() {
            let curr = distance_squared(pos.x, pos.y, v.x, v.y);
            if curr > distance{
                distance = curr;
                result = v.id;
            }
        }
        return result;
    }

    fn shortest_path(&mut self) -> Path {
        let mut result = Path::new();
        result.vertices.push(self.current_vertex);
        if self.current_vertex == self.target_vertex {
            return result;
        }

        let mut dist: Vec<Path> = vec!(Path::new(); MAX_VERT);
    
        let mut heap = BinaryHeap::new();
    
        // We're at `start`, with a zero cost
        //dist[self.current_vertex] = 0;
        heap.push(State { cost: 0, position: self.current_vertex });
    
        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, position }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.target_vertex { return dist[position].clone(); }
    
            // Important as we may have already found a better way
            if cost > dist[position].weight { continue; }
    
            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            let mut index: usize = 0;
            //let mut prev: usize = 0;
            for edge in &self.enemy_graph.edges[position] {
                match edge.path {
                    Motion::Left | Motion::Right | Motion::Jump |
                    Motion::JumpRight | Motion::JumpLeft | Motion::Fall => {

                        //replace 1 with edge weight
                        let next = State { cost: cost + 1, position: index };
    
                        // If so, add it to the frontier and continue
                        if next.cost < dist[next.position].weight {
                            heap.push(next);
                            // Relaxation, we have now found a better way
                            dist[next.position].weight = next.cost;
                            dist[next.position].vertices = dist[position].vertices.clone();
                            dist[next.position].vertices.push(next.position);
                        }
    
                    }
                    Motion::Stop => {} 
                }
                index += 1;
            }
        }
        //case for path not found
        return result;
    }

    pub fn update_sight(&mut self, sight: Vec<Line>, obj: Vec<Line>, map_graph: Graph) {
        self.player_seen = false;
        for l in sight.iter() {
            let mut result = true;
            for o in obj.iter() {
                if lines_intersect(l, o){
                    result = false;
                    break;
                }
            }
            if result{
                //case for the player being seen
                if l.id == MAX_VERT + 1 {
                    self.player_seen = true;
                    self.player_pos.x = l.end.x;
                    self.player_pos.y = l.end.y;
                }
                //case for breakable objects
                else if l.id == MAX_VERT + 2 {

                }
                else if l.id == MAX_VERT + 3 {

                }
                else if l.id == MAX_VERT + 4 {

                }
                else {
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

}