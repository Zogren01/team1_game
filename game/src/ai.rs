use bevy::{prelude::*};
use crate::movement_mesh::*;
use crate::line_of_sight::*;
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
    Attack,
    Retreat,
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
}

impl Enemy{
    pub fn new(v: usize) -> Self {
        Self{
            enemy_graph: Graph::new(),
            next_vertex: MAX_VERT+1,
            current_vertex: v,
            target_vertex: MAX_VERT+1,
            path: Path::new(),
            index_in_path: 0,
            motion: Motion::Stop,
            action: Action::Strafe,
        }
    }
    //updates enemy motion type if they are at or 
    fn at_destination(&mut self, pos: Vec2){
        let mut found: bool = false;
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
                if self.next_vertex != self.target_vertex{
                    self.index_in_path += 1;
                    self.next_vertex = self.path.vertices[self.index_in_path];
                    self.motion = self.enemy_graph.edges[self.current_vertex][self.next_vertex].path;
                }
                else{
                    println!("Enemy arrived at destination");
                    self.motion = Motion::Stop;
                }
            }
            else {
                self.motion = Motion::Fall;
            }
        }
    }
    //for now, the target will be passed from the user
    fn choose_target(&mut self){
        /*
        this is where pathfinding and enemy decisionmaking will occur
        a list (or some other structure) of what vertices must be traversed will be created
        this will be called every time a destination is reached, or the target should be changed
        */
        match self.action {
            Action::Strafe => {
                //select a random seen vertex
                let r = self.enemy_graph.vertices.len();
                let mut rng = rand::thread_rng();
                let pos: usize = rng.gen_range(0, r);
                self.target_vertex = self.enemy_graph.vertices[pos].id;
                self.path = self.shortest_path();
                self.index_in_path = 0;
            }
            Action::Attack => {
                //select vertex closest to the player
            }
            Action::Retreat => {
                //select vertex farthest from the player
            }
        }
        
        println!("Distance from {} to {} is: {}", self.current_vertex, self.target_vertex, self.path.weight);
        println!("Path is: ");
        for v in self.path.vertices.iter_mut(){
            println!("{}", v);
        }

        self.next_vertex = self.path.vertices[self.index_in_path];
        self.motion = self.enemy_graph.edges[self.current_vertex][self.next_vertex].path;
        println!("Next vertex is: {}", self.next_vertex);
    }
    pub fn decide_motion(&mut self, pos: Vec2)-> Motion{
        if self.enemy_graph.vertices.len() > 0 {
            if matches!(self.motion, Motion::Stop){
                self.choose_target();
            }
            self.at_destination(pos);
        }
        return self.motion;
    }

    fn find_self(&mut self) {
        //find what vertex the enemy is at or close to
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