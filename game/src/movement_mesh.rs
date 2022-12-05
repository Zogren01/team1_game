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

    if id == 3{
        //starting panel vertices
        graph.vertices.push(Vertex::new_scaled(13.5, 10.5, 0));
        graph.vertices.push(Vertex::new_scaled(-13.5, 10.5, 1));
        graph.edges[0][1] = Edge::new(Motion::Left);
        graph.edges[1][0] = Edge::new(Motion::Right);

        
        
        //left side:

        //jump box
        graph.vertices.push(Vertex::new_scaled(-18.5, 7.25, 2));
        graph.edges[1][2] = Edge::new(Motion::Left);
        graph.edges[2][1] = Edge::new(Motion::JumpRight);
        //left side of box
        graph.vertices.push(Vertex::new_scaled(-20.5, 4.5, 3));
        graph.edges[2][3] = Edge::new(Motion::Left);
        graph.edges[3][2] = Edge::new(Motion::JumpRight);
        //far left of left platform
        graph.vertices.push(Vertex::new_scaled(-28.5, 4.5, 4));
        graph.edges[3][4] = Edge::new(Motion::Left);
        graph.edges[4][3] = Edge::new(Motion::Right);
        //right side of box
        graph.vertices.push(Vertex::new_scaled(-15.5, 4.5, 5));
        graph.edges[2][5] = Edge::new(Motion::Right);
        graph.edges[5][2] = Edge::new(Motion::JumpLeft);
        //far right of left platform
        graph.vertices.push(Vertex::new_scaled(-1., 4.5, 6));
        graph.edges[5][6] = Edge::new(Motion::Right);
        graph.edges[6][5] = Edge::new(Motion::Left);




        //right side

        //first side platform on right (left edge of platform)
        graph.vertices.push(Vertex::new_scaled(17., 7.5, 7));
        graph.edges[0][7] = Edge::new(Motion::Right);
        graph.edges[7][0] = Edge::new(Motion::JumpLeft);
        //first side platform on right (right edge of platform)
        graph.vertices.push(Vertex::new_scaled(28.5, 7.5, 8));
        graph.edges[7][8] = Edge::new(Motion::Right);
        graph.edges[8][7] = Edge::new(Motion::Left);
        //second side platform on right (right edge of platform)
        graph.vertices.push(Vertex::new_scaled(11., 4.5, 9));
        graph.edges[7][9] = Edge::new(Motion::Left);
        graph.edges[9][7] = Edge::new(Motion::JumpRight);
        //second side platform on right (left edge of platform)
        graph.vertices.push(Vertex::new_scaled(1., 4.5, 10));
        graph.edges[9][10] = Edge::new(Motion::Left);
        graph.edges[10][9] = Edge::new(Motion::Right);
        //top of block on large middle platform
        graph.vertices.push(Vertex::new_scaled(15., 2.5, 11));
        graph.edges[11][9] = Edge::new(Motion::JumpLeft);
        graph.edges[9][11] = Edge::new(Motion::Right);
        //right of block on large middle platform
        graph.vertices.push(Vertex::new_scaled(19.5, -0.5, 12));
        graph.edges[11][12] = Edge::new(Motion::Right);
        graph.edges[12][11] = Edge::new(Motion::JumpLeft);
        //far right on large middle platform
        graph.vertices.push(Vertex::new_scaled(28.5, -0.5, 13));
        graph.edges[13][12] = Edge::new(Motion::Left);
        graph.edges[12][13] = Edge::new(Motion::Right);
        //left of block on large middle platform
        graph.vertices.push(Vertex::new_scaled(10.5, -0.5, 14));
        graph.edges[11][14] = Edge::new(Motion::Left);
        graph.edges[14][11] = Edge::new(Motion::JumpRight);
        //right edge of large middle platform
        graph.vertices.push(Vertex::new_scaled(-9.5, -0.5, 15));
        graph.edges[15][14] = Edge::new(Motion::Right);
        graph.edges[14][15] = Edge::new(Motion::Left);



        //lower left platform

        //middle
        graph.vertices.push(Vertex::new_scaled(-15., -5.5, 16));
        //graph.edges[15][16] = Edge::new(Motion::Left);
        //left edge
        graph.vertices.push(Vertex::new_scaled(-28.5, -5.5, 17));
        graph.edges[17][16] = Edge::new(Motion::Right);
        graph.edges[16][17] = Edge::new(Motion::Left);
        //right edge
        graph.vertices.push(Vertex::new_scaled(-12., -5.5, 18));
        graph.edges[18][16] = Edge::new(Motion::Left);
        graph.edges[16][18] = Edge::new(Motion::Right);


        //bottom platform


        //right top of left block
        graph.vertices.push(Vertex::new_scaled(-6., -8.25, 19));
        graph.edges[18][19] = Edge::new(Motion::Right);
        //left top of left block
        graph.vertices.push(Vertex::new_scaled(-8., -8.25, 20));
        graph.edges[20][19] = Edge::new(Motion::Right);
        graph.edges[19][20] = Edge::new(Motion::Left);
        graph.edges[20][18] = Edge::new(Motion::JumpLeft);
        //left of left block
        graph.vertices.push(Vertex::new_scaled(-12.5, -11.5, 21));
        graph.edges[21][20] = Edge::new(Motion::JumpRight);
        graph.edges[20][21] = Edge::new(Motion::Left);
        //right of left block
        graph.vertices.push(Vertex::new_scaled(-1.5, -11.5, 22));
        graph.edges[22][19] = Edge::new(Motion::JumpLeft);
        graph.edges[19][22] = Edge::new(Motion::Right);
        
        //right top of right block
        graph.vertices.push(Vertex::new_scaled(8., -8.25, 23));
        //left top of right block
        graph.vertices.push(Vertex::new_scaled(6., -8.25, 24));
        graph.edges[23][24] = Edge::new(Motion::Left);
        graph.edges[24][23] = Edge::new(Motion::Right);
        //left of right block
        graph.vertices.push(Vertex::new_scaled(1.5, -11.5, 25));
        graph.edges[25][24] = Edge::new(Motion::JumpRight);
        graph.edges[25][22] = Edge::new(Motion::Left);
        graph.edges[22][25] = Edge::new(Motion::Right);
        graph.edges[24][25] = Edge::new(Motion::Left);
        //right of right block
        graph.vertices.push(Vertex::new_scaled(12.5, -11.5, 26));
        graph.edges[26][23] = Edge::new(Motion::JumpLeft);
        graph.edges[23][26] = Edge::new(Motion::Right);

        //lower right platform

        //middle
        graph.vertices.push(Vertex::new_scaled(15., -5.5, 27));
        //left edge
        graph.vertices.push(Vertex::new_scaled(28.5, -5.5, 28));
        graph.edges[23][28] = Edge::new(Motion::JumpRight);
        graph.edges[28][24] = Edge::new(Motion::Left);
        graph.edges[28][27] = Edge::new(Motion::Right);
        graph.edges[27][28] = Edge::new(Motion::Left);
        //right edge
        graph.vertices.push(Vertex::new_scaled(12., -5.5, 29));
        graph.edges[29][27] = Edge::new(Motion::Left);
        graph.edges[27][29] = Edge::new(Motion::Right);

    }

    if id == 4{
        //starting panel vertices
        graph.vertices.push(Vertex::new_scaled(12.5, 10.5, 0));
        graph.vertices.push(Vertex::new_scaled(-13.5, 10.5, 1));
        graph.edges[0][1] = Edge::new(Motion::Left);
        graph.edges[1][0] = Edge::new(Motion::Right);



        //first drop down platform

        //far left of platform
        graph.vertices.push(Vertex::new_scaled(-28.5, 4.5, 2));
        //top of left box
        graph.vertices.push(Vertex::new_scaled(-20., 7., 3));
        //top of middle box
        graph.vertices.push(Vertex::new_scaled(-7., 7., 4));
        //top of right box
        graph.vertices.push(Vertex::new_scaled(5., 7., 5));
        //left of left box
        graph.vertices.push(Vertex::new_scaled(-24.5, 4.5, 6));
        graph.edges[2][6] = Edge::new(Motion::Right);
        graph.edges[6][2] = Edge::new(Motion::Left);
        graph.edges[3][6] = Edge::new(Motion::Left);
        graph.edges[6][3] = Edge::new(Motion::JumpRight);
        //right of left box
        graph.vertices.push(Vertex::new_scaled(-15.5, 4.5, 7));
        graph.edges[3][7] = Edge::new(Motion::Right);
        graph.edges[7][3] = Edge::new(Motion::JumpLeft);
        //left of middle box
        graph.vertices.push(Vertex::new_scaled(-11.5, 4.5, 8));
        graph.edges[7][8] = Edge::new(Motion::Right);
        graph.edges[8][7] = Edge::new(Motion::Left);
        graph.edges[4][8] = Edge::new(Motion::Left);
        graph.edges[8][4] = Edge::new(Motion::JumpRight);
        //right of middle box
        graph.vertices.push(Vertex::new_scaled(-2.5, 4.5, 9));
        graph.edges[4][9] = Edge::new(Motion::Right);
        graph.edges[9][4] = Edge::new(Motion::JumpLeft);
        //left of right box
        graph.vertices.push(Vertex::new_scaled(0.5, 4.5, 10));
        graph.edges[9][10] = Edge::new(Motion::Right);
        graph.edges[10][9] = Edge::new(Motion::Left);
        graph.edges[5][10] = Edge::new(Motion::Left);
        graph.edges[10][5] = Edge::new(Motion::JumpRight);
        //right of right box
        graph.vertices.push(Vertex::new_scaled(9.5, 4.5, 11));
        graph.edges[5][11] = Edge::new(Motion::Right);
        graph.edges[11][5] = Edge::new(Motion::JumpLeft);
        //right side of platform
        graph.vertices.push(Vertex::new_scaled(15., 4.5, 12));
        graph.edges[12][11] = Edge::new(Motion::Left);
        graph.edges[11][12] = Edge::new(Motion::Right);


        //second drop down platform - right side

        //left edge of right most platform
        graph.vertices.push(Vertex::new_scaled(17.5, -0.5, 13));
        //right edge of right most platform
        graph.vertices.push(Vertex::new_scaled(28.5, -0.5, 14));
        graph.edges[13][14] = Edge::new(Motion::Right);
        graph.edges[14][13] = Edge::new(Motion::Left);
        //top of block
        graph.vertices.push(Vertex::new_scaled(11., -2.25, 15));
        graph.edges[13][15] = Edge::new(Motion::Left);
        graph.edges[15][13] = Edge::new(Motion::JumpRight);
        //right of block
        graph.vertices.push(Vertex::new_scaled(15.75, -4.5, 16));
        graph.edges[16][15] = Edge::new(Motion::JumpLeft);
        graph.edges[15][16] = Edge::new(Motion::Right);
        //far right wall
        graph.vertices.push(Vertex::new_scaled(28.5, -4.5, 17));
        graph.edges[16][17] = Edge::new(Motion::Right);
        graph.edges[17][16] = Edge::new(Motion::JumpLeft);
        //left of block
        graph.vertices.push(Vertex::new_scaled(6.25, -4.5, 18));
        graph.edges[18][15] = Edge::new(Motion::JumpRight);
        graph.edges[15][18] = Edge::new(Motion::Left);
        //left corner of cave
        graph.vertices.push(Vertex::new_scaled(1., -4.5, 19));
        graph.edges[18][19] = Edge::new(Motion::Left);
        graph.edges[19][18] = Edge::new(Motion::Right);

        //second drop down platform - middle
        
        //right edge
        graph.vertices.push(Vertex::new_scaled(5., -0.5, 20));
        graph.edges[15][20] = Edge::new(Motion::JumpLeft);
        graph.edges[20][15] = Edge::new(Motion::JumpRight);
        //left edge
        graph.vertices.push(Vertex::new_scaled(-7., -0.5, 21));
        graph.edges[21][20] = Edge::new(Motion::Right);
        graph.edges[20][21] = Edge::new(Motion::Left);


        //second drop down platform - left side upper
        //left edge
        graph.vertices.push(Vertex::new_scaled(-28.5, -0.5, 22));
        //right edge
        graph.vertices.push(Vertex::new_scaled(-15.5, -0.5, 23));
        graph.edges[22][23] = Edge::new(Motion::Right);
        graph.edges[23][22] = Edge::new(Motion::Left);

        //second drop down platform - left side lower
        //left edge
        graph.vertices.push(Vertex::new_scaled(-28.5, -4.5, 24));
        //right edge
        graph.vertices.push(Vertex::new_scaled(-5., -4.5, 25));
        graph.edges[24][25] = Edge::new(Motion::Right);
        graph.edges[25][24] = Edge::new(Motion::Left);


        //floor
        //top of left block
        graph.vertices.push(Vertex::new_scaled(-11., -9., 26));
        //top of right block
        graph.vertices.push(Vertex::new_scaled(11., -9., 27));
        //left of left block
        graph.vertices.push(Vertex::new_scaled(-15.5, -11.5, 28));
        graph.edges[28][26] = Edge::new(Motion::JumpRight);
        graph.edges[26][28] = Edge::new(Motion::Left);
        //right of left block
        graph.vertices.push(Vertex::new_scaled(-6.5, -11.5, 29));
        graph.edges[26][29] = Edge::new(Motion::Right);
        graph.edges[29][26] = Edge::new(Motion::JumpLeft);
        //left of right block
        graph.vertices.push(Vertex::new_scaled(6.5, -11.5, 30));
        graph.edges[30][27] = Edge::new(Motion::JumpRight);
        graph.edges[27][30] = Edge::new(Motion::Left);
        graph.edges[30][29] = Edge::new(Motion::Left);
        graph.edges[29][30] = Edge::new(Motion::Right);
        //right of right block
        graph.vertices.push(Vertex::new_scaled(15.5, -11.5, 31));
        graph.edges[27][31] = Edge::new(Motion::Right);
        graph.edges[31][27] = Edge::new(Motion::JumpLeft);

    }

    if id == 5{
        //top half

        //starting panel vertices
        graph.vertices.push(Vertex::new_scaled(3.5, 10.5, 0));
        graph.vertices.push(Vertex::new_scaled(-18.5, 10.5, 1));
        graph.edges[0][1] = Edge::new(Motion::Left);
        graph.edges[1][0] = Edge::new(Motion::Right);

        //top right platform
        graph.vertices.push(Vertex::new_scaled(28.5, 10.5, 2));
        graph.vertices.push(Vertex::new_scaled(11.5, 10.5, 3));
        graph.edges[2][3] = Edge::new(Motion::Left);
        graph.edges[3][2] = Edge::new(Motion::Right);
        //top of first block middle platform
        graph.vertices.push(Vertex::new_scaled(7.5, 7.25, 4));
        graph.edges[4][0] = Edge::new(Motion::JumpLeft);
        graph.edges[4][3] = Edge::new(Motion::JumpRight);
        graph.edges[0][4] = Edge::new(Motion::Right);
        graph.edges[3][4] = Edge::new(Motion::Left);
        //right side of top block
        graph.vertices.push(Vertex::new_scaled(12., 4.5, 5));
        graph.edges[4][5] = Edge::new(Motion::Right);
        graph.edges[5][4] = Edge::new(Motion::JumpLeft);
        //left side of top block
        graph.vertices.push(Vertex::new_scaled(3., 4.5, 6));
        graph.edges[4][6] = Edge::new(Motion::Left);
        graph.edges[6][4] = Edge::new(Motion::JumpRight);
        //far right on second level platform
        graph.vertices.push(Vertex::new_scaled(28.5, 4.5, 7));
        graph.edges[7][5] = Edge::new(Motion::Left);
        graph.edges[5][7] = Edge::new(Motion::Right);
        //far left on second level platform
        graph.vertices.push(Vertex::new_scaled(-20.5, 4.5, 8));
        graph.edges[6][8] = Edge::new(Motion::Left);
        graph.edges[8][6] = Edge::new(Motion::Right);


        //lower level

        //left wall
        graph.vertices.push(Vertex::new_scaled(-28.5, -6.5, 9));
        //jump to lower floating block from left
        graph.vertices.push(Vertex::new_scaled(-22., -6.5, 10));
        graph.edges[10][9] = Edge::new(Motion::Left);
        graph.edges[9][10] = Edge::new(Motion::Right);
        //top of lower floating block
        graph.vertices.push(Vertex::new_scaled(-17.5, -3.25, 11));
        graph.edges[10][11] = Edge::new(Motion::JumpRight);
        graph.edges[11][10] = Edge::new(Motion::Left);
        //jump to lower floating block from right
        graph.vertices.push(Vertex::new_scaled(-13., -6.5, 12));
        graph.edges[12][11] = Edge::new(Motion::JumpLeft);
        graph.edges[11][12] = Edge::new(Motion::Right);
        graph.edges[12][10] = Edge::new(Motion::Left);
        graph.edges[10][12] = Edge::new(Motion::Right);
        //top of upper floating block
        graph.vertices.push(Vertex::new_scaled(-13., -1.25, 13));
        graph.edges[13][11] = Edge::new(Motion::Left);
        graph.edges[11][13] = Edge::new(Motion::JumpRight);
        //floating platform from jumps - left side
        graph.vertices.push(Vertex::new_scaled(-9., 0.5, 14));
        graph.edges[13][14] = Edge::new(Motion::JumpRight);
        graph.edges[14][13] = Edge::new(Motion::Left);
        //floating platform from jumps - right side
        graph.vertices.push(Vertex::new_scaled(4.5, 0.5, 15));
        graph.edges[15][14] = Edge::new(Motion::Left);
        graph.edges[14][15] = Edge::new(Motion::Right);
        //right side of lower level
        graph.vertices.push(Vertex::new_scaled(-5.5, -6.5, 16));
        graph.edges[12][16] = Edge::new(Motion::Right);
        graph.edges[16][12] = Edge::new(Motion::Left);


        //bottom floor level

        //top of smaller block
        graph.vertices.push(Vertex::new_scaled(10., -8.25, 17));
        //top of larger block
        graph.vertices.push(Vertex::new_scaled(13., -5.5, 18));
        graph.edges[17][18] = Edge::new(Motion::JumpRight);
        //left of smaller block
        graph.vertices.push(Vertex::new_scaled(5.5, -11.5, 19));
        graph.edges[19][17] = Edge::new(Motion::JumpRight);
        graph.edges[17][19] = Edge::new(Motion::Left);
        graph.edges[18][19] = Edge::new(Motion::Left);
        //left side of bottom floor
        graph.vertices.push(Vertex::new_scaled(-22., -11.5, 20));
        graph.edges[19][20] = Edge::new(Motion::Left);
        graph.edges[20][19] = Edge::new(Motion::Right);
    }
    if id == 6{
        //starting panel vertices
        graph.vertices.push(Vertex::new_scaled(10., 10.5, 0));
        graph.vertices.push(Vertex::new_scaled(-10., 10.5, 1));
        graph.edges[0][1] = Edge::new(Motion::Left);
        graph.edges[1][0] = Edge::new(Motion::Right);

        //top platforms

        //top of right box
        graph.vertices.push(Vertex::new_scaled(14.5, 8.75, 2));
        //jump up to right box
        graph.vertices.push(Vertex::new_scaled(19., 6., 3));
        graph.edges[3][2] = Edge::new(Motion::JumpLeft);
        graph.edges[2][3] = Edge::new(Motion::Right);
        graph.edges[0][3] = Edge::new(Motion::Right);
        graph.edges[2][0] = Edge::new(Motion::JumpLeft);

        //top of left box
        graph.vertices.push(Vertex::new_scaled(-14.5, 8.75, 4));
        //jump up to left box
        graph.vertices.push(Vertex::new_scaled(-19., 6., 5));
        graph.edges[5][4] = Edge::new(Motion::JumpRight);
        graph.edges[4][5] = Edge::new(Motion::Left);
        graph.edges[1][5] = Edge::new(Motion::Left);
        graph.edges[4][1] = Edge::new(Motion::JumpRight);

        //edge of right top platform
        graph.vertices.push(Vertex::new_scaled(22.5, 6., 6));
        graph.edges[3][6] = Edge::new(Motion::Right);
        graph.edges[6][3] = Edge::new(Motion::Left);

        //edge of left top platform
        graph.vertices.push(Vertex::new_scaled(-22.5, 6., 7));
        graph.edges[7][5] = Edge::new(Motion::Right);
        graph.edges[5][7] = Edge::new(Motion::Left);

        //second levels

        //top of far right side block
        graph.vertices.push(Vertex::new_scaled(27.5, 3.5, 8));
        graph.edges[8][6] = Edge::new(Motion::JumpLeft);
        graph.edges[6][8] = Edge::new(Motion::JumpRight);
        //top of far left side block
        graph.vertices.push(Vertex::new_scaled(-27.5, 3.5, 9));
        graph.edges[9][7] = Edge::new(Motion::JumpRight);
        graph.edges[7][9] = Edge::new(Motion::JumpLeft);

        //jump up to right block
        graph.vertices.push(Vertex::new_scaled(23., 1., 10));
        graph.edges[8][10] = Edge::new(Motion::Left);
        graph.edges[10][8] = Edge::new(Motion::JumpRight);
        //jump up to left block
        graph.vertices.push(Vertex::new_scaled(-23., 1., 11));
        graph.edges[9][11] = Edge::new(Motion::Right);
        graph.edges[11][9] = Edge::new(Motion::JumpLeft);

        //left edge of right platform
        graph.vertices.push(Vertex::new_scaled(10.5, 1., 12));
        graph.edges[12][10] = Edge::new(Motion::Right);
        graph.edges[10][12] = Edge::new(Motion::Left);
        //top of middle box on left platform
        graph.vertices.push(Vertex::new_scaled(-9., 3.5, 13));
        //jump up to middle box from left
        graph.vertices.push(Vertex::new_scaled(-13.5, 1., 14));
        graph.edges[13][14] = Edge::new(Motion::Left);
        graph.edges[14][13] = Edge::new(Motion::JumpRight);
        graph.edges[11][14] = Edge::new(Motion::Right);
        graph.edges[14][11] = Edge::new(Motion::Left);
        //jump up to middle box from right
        graph.vertices.push(Vertex::new_scaled(-4.5, 1., 15));
        graph.edges[15][13] = Edge::new(Motion::JumpLeft);
        graph.edges[13][15] = Edge::new(Motion::Right);
        //right edge of left platform
        graph.vertices.push(Vertex::new_scaled(-3., 1., 35));
        graph.edges[15][35] = Edge::new(Motion::Right);
        graph.edges[35][15] = Edge::new(Motion::Left);

        //middle box
        
        //from the left
        graph.vertices.push(Vertex::new_scaled(0.5, -1.5, 16));
        graph.edges[35][16] = Edge::new(Motion::Right);
        graph.edges[16][35] = Edge::new(Motion::JumpLeft);
        //from the right
        graph.vertices.push(Vertex::new_scaled(6.5, -1.5, 17));
        graph.edges[17][12] = Edge::new(Motion::JumpRight);
        graph.edges[12][17] = Edge::new(Motion::Left);

        //lower platform

        //left side right edge
        graph.vertices.push(Vertex::new_scaled(-3., -4.5, 18));
        graph.edges[18][16] = Edge::new(Motion::JumpRight);
        graph.edges[16][18] = Edge::new(Motion::Left);
        //left side left edge
        graph.vertices.push(Vertex::new_scaled(-28.5, -4.5, 19));

        //top of left block
        graph.vertices.push(Vertex::new_scaled(-20., -2., 20));
        //right side jump up
        graph.vertices.push(Vertex::new_scaled(-15.5, -4.5, 21));
        graph.edges[21][20] = Edge::new(Motion::JumpLeft);
        graph.edges[20][21] = Edge::new(Motion::Right);
        graph.edges[18][21] = Edge::new(Motion::Left);
        graph.edges[21][18] = Edge::new(Motion::Right);
        //left side jump up
        graph.vertices.push(Vertex::new_scaled(-24.5, -4.5, 22));
        graph.edges[22][20] = Edge::new(Motion::JumpRight);
        graph.edges[20][22] = Edge::new(Motion::Left);
        graph.edges[22][19] = Edge::new(Motion::Left);
        graph.edges[19][22] = Edge::new(Motion::Right);

        //right platform jump up to block
        graph.vertices.push(Vertex::new_scaled(11., -4.5, 23));
        graph.edges[23][17] = Edge::new(Motion::JumpLeft);
        graph.edges[17][23] = Edge::new(Motion::Right);
        //right edge of right lower platform
        graph.vertices.push(Vertex::new_scaled(18., -4.5, 24));
        graph.edges[24][23] = Edge::new(Motion::Left);
        graph.edges[23][24] = Edge::new(Motion::Right);

        //bottom floor not connected

        //left side

        //right edge of left side
        graph.vertices.push(Vertex::new_scaled(3., -11.5, 25));
        //top of box on left side
        graph.vertices.push(Vertex::new_scaled(-7., -9., 26));
        //jump up from right side
        graph.vertices.push(Vertex::new_scaled(-2.5, -11.5, 27));
        graph.edges[25][27] = Edge::new(Motion::Left);
        graph.edges[27][25] = Edge::new(Motion::Right);
        graph.edges[26][27] = Edge::new(Motion::Right);
        graph.edges[27][26] = Edge::new(Motion::JumpLeft);
        //jump up from left side
        graph.vertices.push(Vertex::new_scaled(-11.5, -11.5, 28));
        graph.edges[26][28] = Edge::new(Motion::Left);
        graph.edges[28][26] = Edge::new(Motion::JumpRight);
        //left bottom edge
        graph.vertices.push(Vertex::new_scaled(-24.5, -11.5, 29));
        graph.edges[29][28] = Edge::new(Motion::Right);
        graph.edges[28][29] = Edge::new(Motion::Left);

        //right side
        //left edge of right side
        graph.vertices.push(Vertex::new_scaled(5., -11.5, 30));
        //top of box on right side
        graph.vertices.push(Vertex::new_scaled(12., -9., 31));
        //jump up from left side
        graph.vertices.push(Vertex::new_scaled(7.5, -11.5, 32));
        graph.edges[32][30] = Edge::new(Motion::Left);
        graph.edges[30][32] = Edge::new(Motion::Right);
        graph.edges[32][31] = Edge::new(Motion::JumpRight);
        graph.edges[31][32] = Edge::new(Motion::Left);
        //jump up from right side
        graph.vertices.push(Vertex::new_scaled(16.5, -11.5, 33));
        graph.edges[33][31] = Edge::new(Motion::JumpLeft);
        graph.edges[31][33] = Edge::new(Motion::Right);
        //right bottom edge
        graph.vertices.push(Vertex::new_scaled(24.5, -11.5, 34));
        graph.edges[33][34] = Edge::new(Motion::Right);
        graph.edges[34][33] = Edge::new(Motion::Left);


    }


    return graph;
}