use bevy::prelude::*;

pub const MAX_VERT: usize = 75;
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
        //vertices on lower panels
        graph.vertices.push(Vertex::new_scaled(22.5, 6.5, 16));
        graph.edges[10][16] = Edge::new(Motion::Right);
        graph.edges[10][22] = Edge::new(Motion::Right);
        graph.edges[16][10] = Edge::new(Motion::JumpLeft);
        graph.vertices.push(Vertex::new_scaled(-22.5, 6.5, 17));
        graph.edges[11][17] = Edge::new(Motion::Left);
        graph.edges[11][23] = Edge::new(Motion::Left);
        graph.edges[17][11] = Edge::new(Motion::JumpRight);
        graph.vertices.push(Vertex::new_scaled(25.5, 6.5, 18));
        graph.edges[16][18] = Edge::new(Motion::Right);
        graph.edges[18][16] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-25.5, 6.5, 19));
        graph.edges[17][19] = Edge::new(Motion::Left);
        graph.edges[19][17] = Edge::new(Motion::Right);
        //vertices along floors
        graph.vertices.push(Vertex::new_scaled(27.5, 3.5, 20));
        graph.edges[18][20] = Edge::new(Motion::Right);
        graph.edges[20][18] = Edge::new(Motion::JumpLeft);
        graph.edges[14][20] = Edge::new(Motion::Right);
        graph.vertices.push(Vertex::new_scaled(21., 3.5, 22));
        graph.edges[20][22] = Edge::new(Motion::Left);
        graph.edges[22][20] = Edge::new(Motion::Right);
        graph.edges[16][22] = Edge::new(Motion::Left);
        graph.edges[22][16] = Edge::new(Motion::JumpRight);
        graph.vertices.push(Vertex::new_scaled(16.5, 3.5, 24));
        graph.edges[22][24] = Edge::new(Motion::Left);
        graph.edges[24][22] = Edge::new(Motion::Right);
        graph.edges[16][24] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(9.5, 3.5, 26));
        graph.edges[24][26] = Edge::new(Motion::Left);
        graph.edges[26][24] = Edge::new(Motion::Right);
        graph.vertices.push(Vertex::new_scaled(3.5, 3.5, 28));
        graph.edges[26][28] = Edge::new(Motion::Left);
        graph.edges[28][26] = Edge::new(Motion::Right);
        
        graph.vertices.push(Vertex::new_scaled(-27.5, 3.5, 21));
        graph.edges[19][21] = Edge::new(Motion::Left);
        graph.edges[21][19] = Edge::new(Motion::JumpRight);
        graph.edges[15][21] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-21., 3.5, 23));
        graph.edges[21][23] = Edge::new(Motion::Right);
        graph.edges[23][21] = Edge::new(Motion::Left);
        graph.edges[17][23] = Edge::new(Motion::Right);
        graph.edges[23][17] = Edge::new(Motion::JumpLeft);
        graph.vertices.push(Vertex::new_scaled(-16.5, 3.5, 25));
        graph.edges[23][25] = Edge::new(Motion::Right);
        graph.edges[25][23] = Edge::new(Motion::Left);
        graph.edges[17][25] = Edge::new(Motion::Right);
        graph.vertices.push(Vertex::new_scaled(-9.5, 3.5, 27));
        graph.edges[25][27] = Edge::new(Motion::Right);
        graph.edges[27][25] = Edge::new(Motion::Left);
        graph.vertices.push(Vertex::new_scaled(-3.5, 3.5, 29));
        graph.edges[27][29] = Edge::new(Motion::Right);
        graph.edges[29][27] = Edge::new(Motion::Left);

        //graph.edges[29][28] = Edge::new(Motion::JumpRight);
        //graph.edges[28][29] = Edge::new(Motion::JumpLeft);

        //vertices on platform before bottom half
        graph.vertices.push(Vertex::new_scaled(1.5, 1.5, 30));
        graph.edges[28][30] = Edge::new(Motion::Left);
        //only here because it can't see 30 initially
        graph.edges[28][31] = Edge::new(Motion::Left);
        graph.edges[30][28] = Edge::new(Motion::JumpRight);
        graph.vertices.push(Vertex::new_scaled(-1.5, 1.5, 31));
        graph.edges[29][31] = Edge::new(Motion::Right);
        graph.edges[29][30] = Edge::new(Motion::Right);
        graph.edges[31][29] = Edge::new(Motion::JumpLeft);
        graph.edges[30][31] = Edge::new(Motion::Left);
        graph.edges[31][30] = Edge::new(Motion::Right);

        //vertices in air above second half so enemy can get down there
        graph.vertices.push(Vertex::new_scaled(2.5, 1.5, 34));
        graph.vertices.push(Vertex::new_scaled(-2.5, 1.5, 35));
        graph.edges[30][34] = Edge::new(Motion::Right);
        graph.edges[31][35] = Edge::new(Motion::Left);


        //vertices on floor of bottom half
        graph.vertices.push(Vertex::new_scaled(8.5, -7.5, 32));
        graph.vertices.push(Vertex::new_scaled(-8.5, -7.5, 33));
        //intermediate node bc enemy cannot see that far
        graph.vertices.push(Vertex::new_scaled(0., -7.5, 36));
        graph.edges[32][36] = Edge::new(Motion::Left);
        graph.edges[33][36] = Edge::new(Motion::Right);
        graph.edges[36][32] = Edge::new(Motion::Right);
        graph.edges[36][33] = Edge::new(Motion::Left);
        graph.edges[34][32] = Edge::new(Motion::Right);
        graph.edges[35][33] = Edge::new(Motion::Left);

        //vertices on first 1x1 blocks
        graph.vertices.push(Vertex::new_scaled(11.5, -4.5, 38));
        graph.vertices.push(Vertex::new_scaled(-11.5, -4.5, 37));
        graph.edges[32][38] = Edge::new(Motion::JumpRight);
        graph.edges[38][32] = Edge::new(Motion::Left);
        graph.edges[33][37] = Edge::new(Motion::JumpLeft);
        graph.edges[37][33] = Edge::new(Motion::Right);

        //vertices on second 1x1 blocks
        graph.vertices.push(Vertex::new_scaled(14.5, -1.5, 40));
        graph.vertices.push(Vertex::new_scaled(-14.5, -1.5, 39));
        graph.edges[38][40] = Edge::new(Motion::JumpRight);
        graph.edges[40][38] = Edge::new(Motion::Left);
        graph.edges[37][39] = Edge::new(Motion::JumpLeft);
        graph.edges[39][37] = Edge::new(Motion::Right);

        //vertices on platform after second 1x1s
        graph.vertices.push(Vertex::new_scaled(16.5, -4.5, 42));
        graph.vertices.push(Vertex::new_scaled(-16.5, -4.5, 41));
        graph.edges[40][42] = Edge::new(Motion::Right);
        graph.edges[42][40] = Edge::new(Motion::JumpLeft);
        graph.edges[39][41] = Edge::new(Motion::Left);
        graph.edges[41][39] = Edge::new(Motion::JumpRight);
        graph.vertices.push(Vertex::new_scaled(20.5, -4.5, 44));
        graph.vertices.push(Vertex::new_scaled(-20.5, -4.5, 43));
        graph.edges[42][44] = Edge::new(Motion::Right);
        graph.edges[44][42] = Edge::new(Motion::Left);
        graph.edges[41][43] = Edge::new(Motion::Left);
        graph.edges[43][41] = Edge::new(Motion::Right);

        //vertices on small wall on side of ^ platform
        graph.vertices.push(Vertex::new_scaled(22.5, -2.5, 46));
        graph.vertices.push(Vertex::new_scaled(-22.5, -2.5, 45));
        graph.edges[44][46] = Edge::new(Motion::JumpRight);
        graph.edges[46][44] = Edge::new(Motion::Left);
        graph.edges[43][45] = Edge::new(Motion::JumpLeft);
        graph.edges[45][43] = Edge::new(Motion::Right);

        //vertices on bar graph lookin thing
        //floor between wall and pillar 1
            //graph.vertices.push(Vertex::new_scaled(9.5, -11.5, 48));
            //graph.vertices.push(Vertex::new_scaled(-9.5, -11.5, 47));
            graph.vertices.push(Vertex::new_scaled(10.5, -11.5, 50));
            graph.vertices.push(Vertex::new_scaled(-10.5, -11.5, 49));
            //graph.edges[48][50] = Edge::new(Motion::Right);
            //graph.edges[50][48] = Edge::new(Motion::Left);
           //graph.edges[47][49] = Edge::new(Motion::Left);
           // graph.edges[49][47] = Edge::new(Motion::Right);
            graph.edges[32][50] = Edge::new(Motion::Right);
            graph.edges[33][49] = Edge::new(Motion::Left);
            graph.edges[38][50] = Edge::new(Motion::Left);
            graph.edges[37][49] = Edge::new(Motion::Right);
        //pillar 1
            graph.vertices.push(Vertex::new_scaled(12.5, -9.5, 52));
            graph.vertices.push(Vertex::new_scaled(-12.5, -9.5, 51));
            graph.edges[50][52] = Edge::new(Motion::JumpRight);
            graph.edges[52][50] = Edge::new(Motion::Left);
            graph.edges[49][51] = Edge::new(Motion::JumpLeft);
            graph.edges[51][49] = Edge::new(Motion::Right);
            //falling from above platforms
            //graph.edges[38][52] = Edge::new(Motion::Right);
            graph.edges[42][52] = Edge::new(Motion::Left);
            //graph.edges[37][51] = Edge::new(Motion::Left);
            graph.edges[41][51] = Edge::new(Motion::Right);
            //jumping back to main floor
            graph.edges[52][32] = Edge::new(Motion::JumpLeft);
            graph.edges[51][33] = Edge::new(Motion::JumpRight);
        //floor between pillars 1 & 2
            graph.vertices.push(Vertex::new_scaled(14.5, -11.5, 54));
            graph.vertices.push(Vertex::new_scaled(-14.5, -11.5, 53));
            graph.edges[54][52] = Edge::new(Motion::JumpLeft);
            graph.edges[52][54] = Edge::new(Motion::Right);
            graph.edges[53][51] = Edge::new(Motion::JumpRight);
            graph.edges[51][53] = Edge::new(Motion::Left);
        //pillar 2
            graph.vertices.push(Vertex::new_scaled(16.5, -7.5, 56));
            graph.vertices.push(Vertex::new_scaled(-16.5, -7.5, 55));
            graph.edges[56][54] = Edge::new(Motion::Left);
            graph.edges[55][53] = Edge::new(Motion::Right);
            graph.edges[56][52] = Edge::new(Motion::Left);
            graph.edges[52][56] = Edge::new(Motion::JumpRight);
            graph.edges[55][51] = Edge::new(Motion::Right);
            graph.edges[51][55] = Edge::new(Motion::JumpLeft);
        //floor between pillars 2 & 3
            graph.vertices.push(Vertex::new_scaled(18., -11.5, 58));
            graph.vertices.push(Vertex::new_scaled(-18., -11.5, 57));
            graph.edges[56][58] = Edge::new(Motion::Right);
            graph.edges[55][57] = Edge::new(Motion::Left);
        //pillar 3
            graph.vertices.push(Vertex::new_scaled(19.5, -9.5, 60));
            graph.vertices.push(Vertex::new_scaled(-19.5, -9.5, 59));
            //between pillar and floor
            graph.edges[58][60] = Edge::new(Motion::JumpRight);
            graph.edges[60][58] = Edge::new(Motion::Left);
            graph.edges[57][59] = Edge::new(Motion::JumpLeft);
            graph.edges[59][57] = Edge::new(Motion::Right);
            //between pillar and pillar
            graph.edges[60][56] = Edge::new(Motion::JumpLeft);
            graph.edges[56][60] = Edge::new(Motion::Right);
            graph.edges[59][55] = Edge::new(Motion::JumpRight);
            graph.edges[55][59] = Edge::new(Motion::Left);
        //floor between pillars 3 and wall
        graph.vertices.push(Vertex::new_scaled(21., -11.5, 62));
        graph.vertices.push(Vertex::new_scaled(-21., -11.5, 61));
        graph.edges[60][62] = Edge::new(Motion::Right);
        graph.edges[62][60] = Edge::new(Motion::JumpLeft);
        graph.edges[59][61] = Edge::new(Motion::Left);
        graph.edges[61][59] = Edge::new(Motion::JumpRight);
    }
    return graph;
}