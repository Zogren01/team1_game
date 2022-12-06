use bevy::{prelude::*};
use crate::movement_mesh::*;
use crate::ai::*;
use crate::util::*;
use crate::active_util::*;

#[derive(Component)]
pub struct Line {
    pub start: Vec2,
    pub end: Vec2,
    pub id: usize
}

impl Line {
    pub fn new(s: Vec2, e: Vec2, i: usize) -> Self {
        Self {
            start: s,
            end: e,
            id: i,
        }
    }
    pub fn length_squared(&self) -> f32 {
        (self.end.x - self.start.x) * (self.end.x - self.start.x)
            + (self.end.y - self.start.y) * (self.end.y - self.start.y)
    }
}

fn helper(i: Vec2, j: Vec2, k: Vec2) -> bool {
    (k.y - i.y) * (j.x - i.x) > (j.y - i.y) * (k.x - i.x)
}

pub fn lines_intersect(a: &Line, b: &Line) -> bool {
    (helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end))
        && (helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}

pub fn find_vertices(x1:f32, y1:f32, x2:f32, y2:f32, width:f32, height:f32) -> (Vec2, Vec2, Vec2){
    
    let v1: Vec2;
    let v2: Vec2;
    let v3: Vec2;

    if x1 > x2 {
        if y1 >= y2 {
            //top left point
            v1 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
            //bottom right point
            v2 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
            //top right point
            v3 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
        } else {
            //top right point
            v1 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
            //bottom left point
            v2 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
            //bottom right point
            v3 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
        }
    } else {
        if y1 > y2 {
            //top right point
            v1 = Vec2::new(
                x2 + width / 2.,
                y2 + height / 2.,
            );
            //bottom left point
            v2 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
            //top left point
            v3 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
        } else {
            //top left point
            v1 = Vec2::new(
                x2 - width / 2.,
                y2 + height / 2.,
            );
            //bottom right point
            v2 = Vec2::new(
                x2 + width / 2.,
                y2 - height / 2.,
            );
            //bottom left point
            v3 = Vec2::new(
                x2 - width / 2.,
                y2 - height / 2.,
            );
        }
    }
    return (v1, v2, v3);
}

pub fn distance_squared(x1: f32, y1: f32, x2: f32, y2:f32) -> f32 {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
}

pub fn calculate_sight(
    graph: Query<&Graph, With<Graph>>,
    //player: Query<(&Object, &Transform), (With<ActiveObject>, With<Player>)>,
    mut enemies: Query<(&Transform, &mut Enemy), (With<ActiveObject>, With<Enemy>)>,
    objects: Query<(&Object, &Transform), With<Object>>,
) {
    let sight_distance = 800.0;

    for (tr, mut en) in enemies.iter_mut() {
        let pos = tr.translation;
        let mut sight_lines = Vec::new();
        let mut object_lines = Vec::new();

        //add lines for objects to used to determine if an object is blocked form view
        for (o, t) in objects.iter() {
            //v1 and v2 and v3 hold the three vertices visible to the player
            match o.obj_type {
                ObjectType::Block | ObjectType::Spike => {
                    //blocks and spikes are the only two objects that block line of sight
                    let (v1, v2, v3) = find_vertices(
                        pos.x,
                        pos.y,
                        t.translation.x,
                        t.translation.y,
                        o.width,
                        o.height,
                    );
                    //if the object is within range, add its lines to object lines so that they are checked for line of sight
                    let l1 = Line::new(Vec2::new(pos.x, pos.y), v3, 0);
                    if l1.length_squared() < sight_distance * sight_distance {
                        let o1 = Line::new(v1, v3, 0);
                        let o2 = Line::new(v2, v3, 0);
                        object_lines.push(o1);
                        object_lines.push(o2);
                    }
                }
                ObjectType::Breakable | ObjectType::Barrel => {
                    let (v1, v2, v3) = find_vertices(
                        pos.x,
                        pos.y,
                        t.translation.x,
                        t.translation.y,
                        o.width,
                        o.height,
                    );
                    //if the object is within range, add its lines to object lines so that they are checked for line of sight
                    let l1 = Line::new(Vec2::new(pos.x, pos.y), v3, 0);
                    if l1.length_squared() < sight_distance * sight_distance {

                        let sight_line = Line::new(
                            Vec2::new(pos.x, pos.y),
                            Vec2::new(t.translation.x, t.translation.y),
                            MAX_VERT + 2,
                        );
                        sight_lines.push(sight_line);
                    }
                    //also need to add temporary vertices so enemy can destroy them
                }
                ObjectType::Bullet => {
                    //enemy will avoid these
                }
                ObjectType::MeleeEnemy => {
                    let sight_line = Line::new(
                        Vec2::new(pos.x, pos.y),
                        Vec2::new(t.translation.x, t.translation.y),
                        MAX_VERT + 3,
                    );
                    if sight_line.length_squared() < sight_distance * sight_distance {
                        sight_lines.push(sight_line);
                    }
                }
                ObjectType::RangedEnemy => {
                    let sight_line = Line::new(
                        Vec2::new(pos.x, pos.y),
                        Vec2::new(t.translation.x, t.translation.y),
                        MAX_VERT + 4,
                    );
                    if sight_line.length_squared() < sight_distance * sight_distance {
                        sight_lines.push(sight_line);
                    }
                }
                ObjectType::Player => {
                    let sight_line = Line::new(
                        Vec2::new(pos.x, pos.y),
                        Vec2::new(t.translation.x, t.translation.y),
                        MAX_VERT + 1,
                    );
                    if sight_line.length_squared() < sight_distance * sight_distance {
                        sight_lines.push(sight_line);
                    }
                }
                _ => {}
            }
        }
        let g = graph.single();
        for vertex in &g.vertices {
            let sight_line = Line::new(
                Vec2::new(pos.x, pos.y),
                Vec2::new(vertex.x, vertex.y),
                vertex.id,
            );
            if sight_line.length_squared() < sight_distance * sight_distance {
                sight_lines.push(sight_line);
            }
        }
        //cloning the graph for each enemy is expensive but I don't know how to avoid it
        //copmuter fan go brrrr
        en.update_sight(sight_lines, object_lines, g.clone());
    }
}