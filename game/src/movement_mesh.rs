use bevy::prelude::*;

pub const MAX_VERT: usize = 50;
#[derive(Copy, Clone)]
pub struct Vertex{
    pub x: f32,
    pub y: f32,
    pub id: usize,
}
impl Vertex{
    pub fn new(a: f32, b: f32, i: usize) -> Self{
        Self{
            x: a,
            y: b,
            id: i,
        }
    }
    pub fn new_scaled(a: f32, b: f32, i: usize) -> Self{
        Self{
            x: a * 32.,
            y: b * 32.,
            id: i,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Motion{
    Left,
    Right,
    Jump,
    JumpRight,
    JumpLeft,
    Fall,
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

#[derive(Component, Clone)]
pub struct Graph{
    pub vertices: Vec<Vertex>,
    pub edges: [[Edge; MAX_VERT]; MAX_VERT],
}

impl Graph{
    pub fn new() -> Self {
        Self{
            vertices: Vec::new(),
            edges: [[Edge::new(Motion::Stop); MAX_VERT];MAX_VERT],
        }
    }
}

pub fn get_level_mesh(id: i8) -> Graph{

    let mut graph = Graph::new();
    //level id for testing
    if id == 0{
        //outtated vertex positions
        graph.vertices.push(Vertex::new(-224., 32., 0));
        graph.vertices.push(Vertex::new(208., 32., 1));
        graph.vertices.push(Vertex::new(256., 96., 2));

        graph.edges[0][1] = Edge::new(Motion::Right);
        graph.edges[1][0] = Edge::new(Motion::Left);
        graph.edges[1][2] = Edge::new(Motion::JumpRight);
        graph.edges[2][1] = Edge::new(Motion::Left);
    }
    if id == 1{
        //starting panel vertices
        graph.vertices.push(Vertex::new_scaled(4., 10.5, 0));
        graph.vertices.push(Vertex::new_scaled(-4., 10.5, 1));
        graph.edges[0][1] = Edge::new(Motion::Left);
        graph.edges[1][0] = Edge::new(Motion::Right);
        //vertices on two columns
        graph.vertices.push(Vertex::new_scaled(6.5, 12.5, 2));
        graph.edges[1][3] = Edge::new(Motion::JumpLeft);
        graph.edges[3][1] = Edge::new(Motion::Right);
        graph.vertices.push(Vertex::new_scaled(-6.5, 12.5, 3));
        graph.edges[0][2] = Edge::new(Motion::JumpRight);
        graph.edges[2][0] = Edge::new(Motion::Left);
        //vertices between columns and breakable objects
        graph.vertices.push(Vertex::new_scaled(9.5, 10.5, 4));
        graph.edges[2][4] = Edge::new(Motion::Right);
        graph.edges[4][2] = Edge::new(Motion::JumpLeft);
        graph.edges[0][4] = Edge::new(Motion::JumpRight);
        graph.edges[4][0] = Edge::new(Motion::JumpLeft);
        graph.vertices.push(Vertex::new_scaled(-9.5, 10.5, 5));
        graph.edges[3][5] = Edge::new(Motion::Left);
        graph.edges[5][3] = Edge::new(Motion::JumpRight);
        graph.edges[1][5] = Edge::new(Motion::JumpLeft);
        graph.edges[5][1] = Edge::new(Motion::JumpRight);
        //vertices outside of breakable objects
        graph.vertices.push(Vertex::new_scaled(13.5, 10.5, 6));
        graph.edges[4][6] = Edge::new(Motion::Right);
        graph.edges[6][4] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-13.5, 10.5, 7));
        graph.edges[5][7] = Edge::new(Motion::Left);
        graph.edges[7][5] = Edge::new(Motion::Right);
        //vertices on bottom ledges
        graph.vertices.push(Vertex::new_scaled(15.5, 8.5, 8));
        graph.edges[6][8] = Edge::new(Motion::Right);
        graph.edges[8][6] = Edge::new(Motion::JumpLeft);
        graph.vertices.push(Vertex::new_scaled(-15.5, 8.5, 9));
        graph.edges[7][9] = Edge::new(Motion::Left);
        graph.edges[9][7] = Edge::new(Motion::JumpRight);
        //outer vertices on bottom ledges
        graph.vertices.push(Vertex::new_scaled(18.5, 8.5, 10));
        graph.edges[6][10] = Edge::new(Motion::Right);
        graph.edges[8][10] = Edge::new(Motion::Right);
        graph.edges[10][8] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-18.5, 8.5, 11));
        graph.edges[7][11] = Edge::new(Motion::Left);
        graph.edges[9][11] = Edge::new(Motion::Left);
        graph.edges[11][9] = Edge::new(Motion::Right);
        //vertices on enemy panels
        graph.vertices.push(Vertex::new_scaled(20.5, 11.5, 12));
        graph.edges[10][12] = Edge::new(Motion::JumpRight);
        graph.edges[12][10] = Edge::new(Motion::Left);
        graph.edges[12][6] = Edge::new(Motion::JumpLeft);
        graph.vertices.push(Vertex::new_scaled(-20.5, 11.5, 13));
        graph.edges[11][13] = Edge::new(Motion::JumpLeft);
        graph.edges[13][11] = Edge::new(Motion::Right);
        graph.edges[13][7] = Edge::new(Motion::JumpRight);
        graph.vertices.push(Vertex::new_scaled(24.5, 11.5, 14));
        graph.edges[12][14] = Edge::new(Motion::Right);
        graph.edges[14][12] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-24.5, 11.5, 15));
        graph.edges[13][15] = Edge::new(Motion::Left);
        graph.edges[15][13] = Edge::new(Motion::Right);
    }
    return graph;
}