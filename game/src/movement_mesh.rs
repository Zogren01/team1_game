use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Vertex{
    x: f32,
    y: f32,
    id: usize,
}
impl Vertex{
    fn new(a: f32, b: f32, i: usize) -> Self{
        Self{
            x: a,
            y: b,
            id: i,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Motion{
    Left,
    Right,
    Jump,
    JumpRight,
    JumpLeft,
    Stop,
}

#[derive(Copy, Clone)]
pub struct Edge{
    pub path: Motion,
}

impl Edge{
    
    fn new(m: Motion) -> Self{
        Self{
            path: m,
        }
    }
}

pub struct Graph{
    pub vertices: Vec<Vertex>,
    pub edges: [[Edge; 50]; 50],
}

impl Graph{
    pub fn new() -> Self {
        Self{
            vertices: Vec::new(),
            edges: [[Edge::new(Motion::Stop); 50];50],
        }
    }
}

pub fn get_level_mesh(id: i8) -> Graph{

    let mut graph = Graph::new();
    //level id for testing
    if id == 0{
        graph.vertices.push(Vertex::new(-224., 0., 0));
        graph.vertices.push(Vertex::new(256., 0., 1));
        
        graph.edges[0][1] = Edge::new(Motion::Right);
        graph.edges[1][0] = Edge::new(Motion::Left);
    }
    return graph;
}