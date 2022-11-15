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

pub enum Action {
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
}

#[derive(Component)]
pub struct Enemy{
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
}

impl Enemy{
    pub fn new() -> Self {
        Self{
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
        }
    }
    fn reset(&mut self, pos: Vec2) {
        self.immobile_frames = 0;
        let v = self.nearest_vert(pos);
        self.current_vertex = v;
        for i in 0..MAX_VERT{
            match self.enemy_graph.edges[v][i].path{
                Motion::Left | Motion::Right=>{
                    self.next_vertex = i;
                    self.target_vertex = i;
                    self.motion = self.enemy_graph.edges[v][i].path;
                    break;
                }
                Motion::Jump=>{}
                Motion::JumpRight | Motion::JumpLeft=>{
                    self.next_vertex = i;
                    self.target_vertex = i;
                    self.motion = self.enemy_graph.edges[v][i].path;
                }
                Motion::Fall=>{}
                Motion::Stop=>{}
            }
        }
        self.update_path();
    }
    pub fn decide_motion(&mut self, pos: Vec2)-> Motion{
        //only update motion if enemy has seen at least one vertex
        self.attack = Attack::None;
        if self.enemy_graph.vertices.len() > 0 {
            //checks if enemy is stuck
            if self.old_pos == pos && !matches!(self.motion, Motion::Stop){
                self.immobile_frames += 1;
            }
            else{
                self.immobile_frames = 0;
            }
            if self.current_vertex == MAX_VERT+1 || self.immobile_frames >= 3 || matches!(self.action, Action::Attack){
                self.reset(pos);
            }
            //if enemy has seen player
            if self.player_seen{
                let dist_to_player = distance_squared(pos.x, pos.y, self.player_pos.x, self.player_pos.y);
                if dist_to_player < 10000.{
                    self.action = Action::Attack;
                }
                else{
                    self.action = Action::Chase;
                }              
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
        if matches!(self.action, Action::Attack){
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
                    //eventually need code to jump
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
        else{
            let mut x_diff = f32::MAX;
            let mut y_diff = f32::MAX;
            for v in self.enemy_graph.vertices.iter_mut() {
                if v.id == self.next_vertex {
                    x_diff = pos.x - v.x;
                    y_diff = pos.y - v.y;
                }
            }
            if x_diff.abs() <= 5.{
                if y_diff.abs() <= 5.  {
                    self.current_vertex = self.next_vertex;
                    match self.action{
                        Action::Strafe => {
                            println!("strafing");
                            //if still travelling towards goal
                            if self.current_vertex != self.target_vertex{
                                self.index_in_path += 1;
                            }
                            //if a new goal is needed
                            else{
                                println!("Enemy selecting new destination of strafe");
                                let r = self.enemy_graph.vertices.len();
                                let mut rng = rand::thread_rng();
                                let pos: usize = rng.gen_range(0, r);
                                self.target_vertex = self.enemy_graph.vertices[pos].id;
                                self.update_path()
                            }
                        }
                        Action::Chase => {

                            let pl_vert = self.nearest_vert(self.player_pos);
                            //do a check here to see if the player is within range of an attack
                            println!("player seen at: {}, {}", self.player_pos.x/32., self.player_pos.y/32.);
                            println!("vertex closest to player is: {}", pl_vert);
                            if pl_vert != self.target_vertex{
                                self.target_vertex = pl_vert;
                                self.update_path();
                            }
                            else{
                                if self.current_vertex != self.target_vertex{
                                    self.index_in_path += 1;
                                }
                                else{
                                    println!("Enemy arrived at destination of attack");
                                    //temporary code
                                    self.motion = Motion::Stop;
                                    //determine what to do to get to the player
                                }
                            }
                        }
                        Action::Attack => {
                            //the updates for this action are independent of pathfinding,
                            //so behavior is determined at beginning of method
                        }
                        Action::Retreat => {
                            let ret_vert = self.farthest_vert(self.player_pos);
                            //do a check here to see if the player is within range of an attack
                            if ret_vert != self.target_vertex{
                                self.target_vertex = ret_vert;
                                self.update_path();
                            }
                            else{
                                if self.current_vertex != self.target_vertex{
                                    self.index_in_path += 1;
                                }
                                else{
                                    println!("Enemy arrived at destination of retreat");
                                    //temporary code
                                    self.motion = Motion::Stop;
                                    //determine what to do after retreating
                                }
                            }
                        }
                    }
                    if self.path.vertices.len() > self.index_in_path{
                        self.next_vertex = self.path.vertices[self.index_in_path];
                    }
                    println!("Enemy has arrived at vertex: {}\nIs heading to vertex: {}", self.current_vertex, self.next_vertex);
                    self.motion = self.enemy_graph.edges[self.current_vertex][self.next_vertex].path;   
                }

                else {
                    self.motion = Motion::Fall;
                }
            }
        }
    }
    //for now, the target will be passed from the user
    fn update_path(&mut self){
        println!("Finding new path");

        self.path = self.shortest_path();
        self.index_in_path = 0;
        println!("Distance from {} to {} is: {}", self.current_vertex, self.target_vertex, self.path.weight);
        println!("Path is: ");
        for v in self.path.vertices.iter_mut(){
            println!("{}", v);
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
                if l.id == MAX_VERT +1 {
                    self.player_seen = true;
                    self.player_pos.x = l.end.x;
                    self.player_pos.y = l.end.y;
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