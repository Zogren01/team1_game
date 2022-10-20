//imports from outside crates
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::Collision;
use bevy::{prelude::*, window::PresentMode};

//imports from local creates
mod util;
use crate::util::*;

mod active_util;
use crate::active_util::*;

mod ai;
use crate::ai::*;
#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

struct Manager{
    room_number: i8,
    wall_id: i8,
    enemy_id: i8,
}


fn create_level( 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    level: Vec<Descriptor>){
        for desc in level{
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(desc.width, desc.height)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Object::new(0, desc.width, desc.height));
        }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
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
        //.add_system(attack)
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

    //This is for the overlay
    //Putting comments for every object so we know which is which. This is a bad idea for future levels but for now but it gets a basis going.
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        texture: asset_server.load("Room_1.png"),
        transform: Transform::from_xyz(912., 500., 0.),
        ..default()
    });

    //Player(spawns slightly above origin now, starting tile of map centered on origin.)
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 64., 900.),
                ..default()
            },
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Player);

    //main floor
    let mut level = get_level(1);
    create_level(commands, asset_server, texture_atlases, level);

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
    objects: Query<(&Object, &Transform), With<Object>>,
    input: Res<Input<KeyCode>>,
) {
    //TODO: make a struct for all of the sight lines for a given object
    // hold a Vec containing lines
    // hold a reference to the object
    // loop through each of these when doing checks
    let origin = player.single();
    let x_pos = origin.translation.x;
    let y_pos = origin.translation.y;

    if input.pressed(KeyCode::Q) {
        let sight_distance = 300.0;
        let mut sight_lines = Vec::new();
        let mut object_lines = Vec::new();

        for (o, t) in objects.iter() {
            //v1 and v2 hold the endpoints for line of sight
            let v1: Vec2;
            let v2: Vec2;
            //v3 is the third point for the two sides to be used for collision
            let v3: Vec2;

            if x_pos > t.translation.x {
                if y_pos >= t.translation.y {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //top right point
                    v3 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                } else {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //bottom right point
                    v3 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                }
            //MAYBE code for when y's are equal
            } else {
                if y_pos > t.translation.y {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //top left point
                    v3 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                } else {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //bottom left point
                    v3 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
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
        determine_visibility(sight_lines, object_lines);
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut ActiveObject, &mut Transform),
        (With<Player>, Without<Object>),
    >,
    objects: Query<(&Object, &Transform), (With<Object>, Without<Player>)>,
    mut exit: EventWriter<AppExit>,
    mut cam: Query<&mut Transform, (With<Camera>, Without<Object>, Without<Player>)>,
) {
    let (mut pl, mut pt) = player.single_mut();

    let mut camera = cam.single_mut();
    if input.pressed(KeyCode::A) {
        pl.facing_left = true;
        if pl.velocity.x > -PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x - 20.;
        }
    } else if pl.velocity.x < 0. {
        pl.velocity.x = pl.velocity.x + 20.;
    }

    if input.pressed(KeyCode::D) {
        pl.facing_left = false;
        if pl.velocity.x < PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x + 20.;
        }
    } else if pl.velocity.x > 0. {
        pl.velocity.x = pl.velocity.x - 20.;
    }

    if pl.velocity.y > TERMINAL_VELOCITY {
        pl.velocity.y += GRAVITY;
    }

    if input.pressed(KeyCode::Space) && pl.grounded {
        pl.velocity.y = PLAYER_SPEED * 2.;
    }

    pl.grounded = false;
    let deltat = time.delta_seconds();

    let change = pl.velocity * deltat;

    let mut new_pos = pt.translation + Vec3::new(change.x, change.y, 0.);
    //this variable will track where the player will end up if there is no collision with a surface
    for (_o, t) in objects.iter() {
        let res = bevy::sprite::collide_aabb::collide(
            new_pos,
            Vec2::new(PLAYER_SZ, PLAYER_SZ),
            t.translation,
            Vec2::new(_o.width, _o.height),
        );
        if res.is_some() {
            let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
            match coll_type {
                Collision::Left => {
                    pl.velocity.x = 0.;
                    new_pos.x = t.translation.x - (_o.width / 2.) - PLAYER_SZ / 2.;
                }
                Collision::Right => {
                    pl.velocity.x = 0.;
                    new_pos.x = t.translation.x + (_o.width / 2.) + PLAYER_SZ / 2.;
                }
                Collision::Top => {
                    if pl.velocity.y < 0. {
                        //if falling down
                        pl.velocity.y = 0.; //stop vertical velocity
                        pl.grounded = true;
                    }
                    new_pos.y = t.translation.y + (_o.height / 2.) + PLAYER_SZ / 2.;
                    if _o.id == 1 {
                        exit.send(AppExit);
                    }
                }
                Collision::Bottom => {
                    pl.velocity.y = 0.;
                    new_pos.y = t.translation.y - (_o.height / 2.) - PLAYER_SZ / 2.;
                }
                Collision::Inside => {
                    println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                    pl.velocity = Vec2::new(0., 0.);
                }
            }
        }
    }

    pt.translation = new_pos;
    camera.translation.x = pt.translation.x;
    camera.translation.y = pt.translation.y;
}
/*
fn attack(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Player, &mut Transform, &mut Velocity),
        (With<Player>, Without<Object>),
    >,
    objects: Query<(&Object, &Transform), (With<Object>, Without<Player>)>,
    mut commands: Commands,
) {
    let (pl, pt, pv) = player.single_mut();
    if input.just_pressed(KeyCode::P) {
        let mut hitbox_pos;
        if input.pressed(KeyCode::S) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y - PLAYER_SZ, 0.);
        } else if pv.velocity.y != 0. {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y + PLAYER_SZ, 0.);
        } else if !pl.facing_left {
            hitbox_pos = Vec3::new(pt.translation.x + PLAYER_SZ, pt.translation.y, 0.);
        } else {
            hitbox_pos = Vec3::new(pt.translation.x - PLAYER_SZ, pt.translation.y, 0.);
        }
        for (_o, t) in objects.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                hitbox_pos,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(_o.width, _o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => {
                        println!("Attacked object right of player");
                    }
                    Collision::Right => {
                        println!("Attacked object left of player");
                    }
                    Collision::Top => {
                        println!("Attacked object bottom of player");
                    }
                    Collision::Bottom => {
                        println!("Attacked object top of player");
                    }
                    Collision::Inside => {
                        if pt.translation.y - PLAYER_SZ / 2. >= t.translation.y + PLAYER_SZ / 2. {
                            println!("Attacked object below player");
                        } else if pt.translation.x > t.translation.x {
                            println!("Attacked object left of player");
                        } else {
                            println!("Attacked object right of player");
                        }
                    }
                }
            }
        }
    }
}
*/
