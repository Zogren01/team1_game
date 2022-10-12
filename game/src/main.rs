use bevy::{prelude::*, window::PresentMode};
use std::collections::HashSet;
use std::convert::From;

const TITLE: &str = "Team 1 Game";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const GRAVITY: f32 = -20.;
const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const PLAYER_SZ: f32 = 32.;

const TILE_SIZE: f32 = 32.;

const SCROLL_SPEED: f32 = 120.;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component)]
struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

#[derive(Component)]
struct Line {
    start: Vec2,
    end: Vec2,
    obj_id: i8,
}

impl Line {
    fn new(s: Vec2, e: Vec2, i: i8) -> Self {
        Self {
            start: s,
            end: e,
            obj_id: i,
        }
    }
    fn length_squared(&self) -> f32 {
        (self.end.x - self.start.x) * (self.end.x - self.start.x)
            + (self.end.y - self.start.y) * (self.end.y - self.start.y)
    }
    fn print_line(&self) {
        println!(
            "Start: {},{} \n End: {},{} \n",
            self.start.x, self.start.y, self.end.x, self.end.y
        );
    }
}

#[derive(Component)]
struct Object {
    id: i8,
}

impl Object {
    fn new(i: i8) -> Self {
        Self { id: i }
    }
}
#[derive(Component)]
struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn new(w: f32, h: f32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

#[derive(Component)]
struct Player;

/*
#[derive(Component)]
struct Ground;
*/

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Hello World!"),
            width: 1280.,
            height: 720.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(show_popup)
        .add_system(move_player)
        .add_system(calculate_sight)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let images = &[
        "jacob.png",
        "bailey.png",
        "brian.png",
        "ethan.png",
        "jack.png",
        "gio.png",
        "zach.png",
    ];
    commands.spawn_bundle(Camera2dBundle::default());
    let mut time: f32 = 0.0;
    for image in images {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(*image),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            })
            .insert(PopupTimer(Timer::from_seconds(time, false)));
        time += 5.0;
    }

    //load in ground textures
    /*
    let ground_handle = asset_server.load("ground.png");
    let ground_atlas = TextureAtlas::from_grid(ground_handle, Vec2::splat(TILE_SIZE), 2, 2);
    let ground_atlas_len = ground_atlas.textures.len();
    let ground_atlas_handle = texture_atlases.add(ground_atlas);

    let x_bound = WIN_W/2. - TILE_SIZE/2.;
    let y_bound = WIN_H/2. - TILE_SIZE/2.;
    */

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..default()
            },
            ..default()
        })
        .insert(Velocity::new())
        .insert(Player);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 200.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(50., -200., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 200.))
        .insert(Object::new(1));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(100., -200., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(2));
    /*
        let mut count = 0.;
        for i in 0..41 {
            let t = Vec3::new(
                -x_bound + TILE_SIZE*count,
                -y_bound,
                900.,
            );
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: ground_atlas_handle.clone(),
                transform: Transform {
                translation: t,
            ..default()
            },
                sprite: TextureAtlasSprite {
                index: i % ground_atlas_len,
            ..default()
            },
            ..default()
        })
        .insert(Ground);
        count = count + 1.;
        println!("{}", count);
    }
    */
}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    let mut count = 1.0;
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = count;
        }
        count += 1.0;
    }
}

fn calculate_sight(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    objects: Query<(&Object, &Rect, &Transform), With<Object>>,
    input: Res<Input<KeyCode>>,
) {
    //TODO: make a struct for all of the sight lines for a given object
    // hold a Vec containing lines
    // hold a reference to the object
    // loop through each of these when doing checks
    let origin = player.single();
    let x_pos = origin.translation.x;
    let y_pos = origin.translation.y;

    if input.pressed(KeyCode::Space) {
        println!("\nSpacebar pressed\n");
        let sight_distance = 300.0;
        let mut sight_lines = Vec::new();
        let mut object_lines = Vec::new();

        for (o, r, t) in objects.iter() {
            //v1 and v2 hold the endpoints for line of sight
            let v1: Vec2;
            let v2: Vec2;
            //v3 is the third point for the two sides to be used for collision
            let v3: Vec2;

            if x_pos > t.translation.x {
                if y_pos >= t.translation.y {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                    //top right point
                    v3 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                } else {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                    //bottom right point
                    v3 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                }
            //MAYBE code for when y's are equal
            } else {
                if y_pos > t.translation.y {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                    //top left point
                    v3 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                } else {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y + r.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                    //bottom left point
                    v3 = Vec2::new(
                        t.translation.x - r.width / 2.,
                        t.translation.y - r.height / 2.,
                    );
                }
                //MAYBE code for when y's are equal
            }
            //MAYBE code for when x's are equal

            //generate lines of sight
            let s1 = Line::new(Vec2::new(x_pos, y_pos), v1, o.id);
            let s2 = Line::new(Vec2::new(x_pos, y_pos), v2, o.id);
            //MAYBE third line of sight to corner

            //track whether these are in range
            let mut in_range = false;
            if s1.length_squared() < sight_distance * sight_distance {
                sight_lines.push(s1);
                in_range = true;
            }
            if s2.length_squared() < sight_distance * sight_distance {
                sight_lines.push(s2);
                in_range = true;
            }
            if in_range {
                let o1 = Line::new(v1, v3, o.id);
                let o2 = Line::new(v2, v3, o.id);
                object_lines.push(o1);
                object_lines.push(o2);
            }
        }
        /*
        println!("LINES OF SIGHT:");
        for l in sight_lines.iter_mut(){
            l.print_line();
        }
        println!("OBJECT EDGES:");
        for o in object_lines.iter_mut(){
            o.print_line();
        }
        */
        determine_visibility(sight_lines, object_lines);
    }
}

fn determine_visibility(sight: Vec<Line>, obj: Vec<Line>) {
    println!("Determining objects in view...");

    let mut ids: HashSet<i8> = HashSet::new();
    for l in sight.iter() {
        let mut result = true;
        for o in obj.iter() {
            let intersect = lines_intersect(l, o);
            if l.obj_id == 2 && o.obj_id == 1 {
                l.print_line();
                o.print_line();
            }
            if intersect && (o.obj_id != l.obj_id) {
                result = false;
                break;
            }
        }
        if result {
            ids.insert(l.obj_id);
        }
    }
    for id in ids.iter() {
        println!("Object with id {} is visible", id);
    }
}

fn helper(i: Vec2, j: Vec2, k: Vec2) -> bool {
    (k.y - i.y) * (j.x - i.x) > (j.y - i.y) * (k.x - i.x)
}

fn lines_intersect(a: &Line, b: &Line) -> bool {
    (helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end))
        && (helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut pt, mut pv) = player.single_mut();

    // if A pressed & not at full speed, accel; otherwise, deaccel
    if input.pressed(KeyCode::A) {
        if pv.velocity.x > -300. {
            pv.velocity.x = pv.velocity.x - 20.;
        }
    } else if pv.velocity.x < 0. {
        pv.velocity.x = pv.velocity.x + 20.;
    }

    // if D pressed & not at full speed, accel; otherwise, deaccel
    if input.pressed(KeyCode::D) {
        if pv.velocity.x < 300. {
            pv.velocity.x = pv.velocity.x + 20.;
        }
    } else if pv.velocity.x > 0. {
        pv.velocity.x = pv.velocity.x - 20.;
    }

    // if W pressed and we are on the 'ground', increase our vertical velocity to 600s
    if input.pressed(KeyCode::W) {
        if pt.translation.y == -(WIN_H / 2.) + TILE_SIZE * 1.5 {
            pv.velocity.y = 600.;
        }
    }

    // if we are NOT on the ground, apply gravity
    if pt.translation.y > -(WIN_H / 2.) + TILE_SIZE * 1.5 {
        pv.velocity.y += GRAVITY;
    }

    // calculate physical change (d/t * t = d)
    let deltat = time.delta_seconds();
    let change = pv.velocity * deltat;

    // new position is equal to old position plus our change in X/Y since last update
    let new_pos = pt.translation + Vec3::new(change.x, 0., 0.);
    if new_pos.x >= -(WIN_W / 2.) + TILE_SIZE * 1.5 && new_pos.x <= WIN_W / 2. - TILE_SIZE * 1.5 {
        pt.translation = new_pos;
    }

    let new_pos = pt.translation + Vec3::new(0., change.y, 0.);
    // allow translation if in bounds
    if new_pos.y >= -(WIN_H / 2.) + TILE_SIZE * 1.5 && new_pos.y <= WIN_H / 2. - TILE_SIZE * 1.5 {
        pt.translation = new_pos;
    }
    // snap player back to ground level if going below it
    else if new_pos.y < -(WIN_H / 2.) + TILE_SIZE * 1.5 {
        pt.translation.y = -(WIN_H / 2.) + TILE_SIZE * 1.5;
    }
}
