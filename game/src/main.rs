use std::hash::Hash;

//imports from outside crates
use bevy::app::AppExit;
use bevy::asset::{self, LoadState};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::camera::RenderTarget;
use bevy::sprite::collide_aabb::Collision;
use bevy::time::FixedTimestep;
use bevy::ui::update;
use bevy::utils::*;
use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

use iyes_loopless::prelude::*;

//use sdl2::libc::ENOTEMPTY;

//imports from local creates
mod util;
use crate::util::*;

mod active_util;
use crate::active_util::*;

mod ai;
use crate::ai::*;

mod movement_mesh;
use crate::movement_mesh::*;

mod line_of_sight;
use crate::line_of_sight::*;

mod physics;
use crate::physics::*;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);
const START_TIME: f32 = 100.;
const RUNTIME: f64 = 1. / 30.;
const PROJECTILE_SZ: f32 = 6.;

struct Manager {
    room_number: i8,
    wall_id: i8,
    enemy_id: i8,
}

fn create_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    level: Vec<Descriptor>,
    mesh: Graph,
) {
    let mut id = 0;
    for desc in level {
        let mut texture_path = "";
        if !matches!(desc.obj_type, ObjectType::Block) {
            // conditionally render object textures
            if matches!(desc.obj_type, ObjectType::Cobweb) {
                texture_path = "spiderweb.png";
            } else if matches!(desc.obj_type, ObjectType::Spike) {
                texture_path = "spike.png";
            } else if matches!(desc.obj_type, ObjectType::Item) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
            } else if matches!(desc.obj_type, ObjectType::UmbrellaItem) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            //color: Color::PURPLE,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        texture: asset_server.load("umbrella.png"),
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
            } else if matches!(desc.obj_type, ObjectType::JetpackItem) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            //color: Color::GRAY,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        texture: asset_server.load("jetpack.png"),
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
            } else if matches!(desc.obj_type, ObjectType::Barrel) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            //color: Color::GRAY,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        texture: asset_server.load("explosiveBarrel.png"),
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ActiveObject::new(50,0))
                    .insert(MovableObject)
                    .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
                //  .insert(Explosive::new(Timer::from_seconds(2.0, false)));
            } else if matches!(desc.obj_type, ObjectType::Breakable) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        texture: asset_server.load("breakable.png"),
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ActiveObject::new(50,0))
                    .insert(MovableObject)
                    .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
            } else if matches!(desc.obj_type, ObjectType::MeleeEnemy) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 5.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ActiveObject::new(ENEMY_HEALTH, 25))
                    .insert(Object::new(
                        900,
                        desc.width,
                        desc.height,
                        ObjectType::MeleeEnemy,
                    ))
                    .insert(Enemy::new(Type::Melee));
            } else if matches!(desc.obj_type, ObjectType::RangedEnemy) {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::PURPLE,
                            custom_size: Some(Vec2::new(desc.width, desc.height)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(desc.x_pos, desc.y_pos, 5.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ActiveObject::new(ENEMY_HEALTH, 25))
                    .insert(Object::new(
                        900,
                        desc.width,
                        desc.height,
                        ObjectType::RangedEnemy,
                    ))
                    .insert(Enemy::new(Type::Ranged));
            } else if matches!(desc.obj_type, ObjectType::OtherEnemy) {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(desc.width, desc.height)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(desc.x_pos, desc.y_pos, 5.),
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveObject::new(ENEMY_HEALTH, 25))
                .insert(Object::new(
                    900,
                    desc.width,
                    desc.height,
                    ObjectType::OtherEnemy,
                ))
                .insert(Object::new(
                    900,
                    desc.width,
                    desc.height,
                    ObjectType::Barrel,
                ))
                .insert(Enemy::new(Type::Other));
            }
        } else {
            commands
                .spawn_bundle(SpriteBundle {
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
                .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
        }
        id += 1;
    }

    for v in mesh.vertices.clone() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(5., 5.)),
                ..default()
            },
            //   texture: asset_server.load("explosiveBarrel.png"),
            transform: Transform {
                translation: Vec3::new(v.x, v.y, 2.),
                ..default()
            },
            ..default()
        });
    }

    commands.spawn().insert(mesh);
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
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_fixed_timestep(
            Duration::from_millis(17),
            // we need to give it a string name, to refer to it
            "my_fixed_update",
        )
        .add_fixed_timestep(
            Duration::from_millis(250),
            // we need to give it a string name, to refer to it
            "my_fixed_update_2",
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            move_player,
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            barrels_with_barrels.after(move_player),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            apply_collisions.after(barrels_with_barrels),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            enemy_collisions.after(apply_collisions),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            object_collisions.after(enemy_collisions),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            update_positions.after(object_collisions),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            move_enemies,
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            calculate_sight.after(move_enemies),
        )
        .add_fixed_timestep_system(
            "my_fixed_update_2",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            attack_enemies.after(calculate_sight),
        )
        .add_system(player_health)
        .add_system(meleebox_collisions)
        .add_system(item_shop)
        .add_system(my_cursor_system)
        .add_system(show_gui)
        .add_system(attack)
        .add_system(attack_static)
        .add_system(attack_active)
        .add_system(shoot)
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            projectile_active_collision,
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            projectile_static_collisions.after(projectile_active_collision),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            kill_enemies.after(projectile_static_collisions),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            break_objects.after(kill_enemies),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            break_hb_objects.after(break_objects),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            despawn_broken_objects.after(break_hb_objects),
        )
        .add_fixed_timestep_system(
            "my_fixed_update",
            0, // fixed timestep name, sub-stage index
            // it can be a conditional system!
            gravity_on_movables.after(move_player),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let assets = HashMap::new();
    // let sprites = &["barrel.png","boots.png","breakable.png","jetpack.png","spikes.png","umbrella.png"];
    // for sprite in sprites {
    //     assets.insert(sprite, asset_server.load(*sprite));
    // }
    // while asset_server.get_group_load_state(assets.values().into_iter()) != LoadState::Loaded{}

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

    commands.insert_resource(Clock {
        // create the repeating timer
        timer: Timer::from_seconds(START_TIME, true),
    });

    commands
        .spawn_bundle(TextBundle::from_section(
            "",
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                font: asset_server.load("mrsmonster.ttf"),
            },
        ))
        .insert(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .insert(ClockText);

    //spawn creditText
    commands
        .spawn_bundle(TextBundle::from_section(
            "",
            TextStyle {
                font_size: 100.0,
                color: Color::YELLOW,
                font: asset_server.load("mrsmonster.ttf"),
            },
        ))
        .insert(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .insert(CreditText);

    //spawn healthbar
    commands
        .spawn_bundle(TextBundle::from_section(
            "100",
            TextStyle {
                font_size: 100.0,
                color: Color::RED,
                font: asset_server.load("mrsmonster.ttf"),
            },
        ))
        .insert(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        })
        .insert(HealthBar);

    //Player(spawns slightly above origin now, starting tile of map centered on origin.)
    let pt = Transform {
        translation: Vec3::new(0., 320., 900.),
        ..default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: pt,
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Object::new(-1, PLAYER_SZ, PLAYER_SZ, ObjectType::Player))
        .insert(Player::new());
    //this variable can change based on what room the player is in
    let mut level = get_level(1);
    let mesh = get_level_mesh(1);
    create_level(commands, asset_server, texture_atlases, level, mesh);
}

//we can probably add this as an event, to be used when the level id is outside of the possible range
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

//we will also need to implement collisions between 2 active objects, that is where we will do rigidbody collisions
//I'm not sure whether that should run before or after object collisions
fn apply_collisions(
    mut actives: Query<(&Object, &mut ActiveObject, &Transform), With<ActiveObject>>,
    mut objects: Query<(&mut Object, &Transform), (With<Object>, Without<ActiveObject>)>,
    //input: Res<Input<KeyCode>>,
    //will want to use something different later
    mut exit: EventWriter<AppExit>,
) {
    //loop through all objects that move
    for (object, mut active, transform) in actives.iter_mut() {
        for (mut o, t) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                active.projected_position,
                //need to change this to get the size of whatever the object is
                Vec2::new(object.width, object.height),
                t.translation,
                Vec2::new(o.width, o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                if matches!(o.obj_type, ObjectType::Cobweb) {
                    println!("{:?}", coll_type);
                }
                match coll_type {
                    Collision::Left => match o.obj_type {
                        ObjectType::Cobweb => {
                            if active.velocity.x != 0. {
                                active.velocity.x /= 2.;
                            }
                            active.velocity.y = -2.;

                            active.grounded = false;
                        }
                        ObjectType::Block => {
                            active.velocity.x = 0.;
                            active.projected_position.x =
                                t.translation.x - (o.width / 2.) - object.width / 2.;
                        }
                        _ => {}
                    },
                    Collision::Right => match o.obj_type {
                        ObjectType::Cobweb => {
                            if active.velocity.x != 0. {
                                active.velocity.x /= 2.;
                            }
                            active.velocity.y = -2.;
                            active.grounded = false;
                        }
                        ObjectType::Block => {
                            active.velocity.x = 0.;
                            active.projected_position.x =
                                t.translation.x + (o.width / 2.) + object.width / 2.;
                        }
                        _ => {}
                    },
                    Collision::Top => {
                        match o.obj_type {
                            ObjectType::Spike => {
                                exit.send(AppExit);
                            }
                            ObjectType::Cobweb => {
                                if active.velocity.x != 0. {
                                    active.velocity.x /= 2.;
                                }
                                active.velocity.y = -2.;
                                active.grounded = false;
                            }
                            ObjectType::Block => {
                                if matches!(object.obj_type, ObjectType::Barrel)
                                    || matches!(object.obj_type, ObjectType::Breakable)
                                {
                                    if (!active.grounded && active.velocity.y < -15.) {
                                        //object.broken = true;
                                    }
                                }
                                if active.velocity.y < 0. {
                                    //if falling down
                                    active.velocity.y = 0.; //stop vertical velocity
                                }
                                active.projected_position.y =
                                    t.translation.y + (o.height / 2.) + object.height / 2.;

                                active.grounded = true;
                            }
                            _ => {}
                        }
                    }
                    Collision::Bottom => match o.obj_type {
                        ObjectType::Cobweb => {
                            if active.velocity.x != 0. {
                                active.velocity.x /= 2.;
                            }
                            active.velocity.y = -2.;

                            active.grounded = false;
                        }
                        ObjectType::Block => {
                            active.velocity.y = 0.;
                            active.projected_position.y =
                                t.translation.y - (o.height / 2.) - object.height / 2.;
                        }
                        _ => {}
                    },
                    Collision::Inside => match o.obj_type {
                        _ => {
                            println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);
                        }
                    },
                }
            }
        }
    }
}

//this function doesn't seem to work
fn enemy_collisions(
    mut actives: Query<
        (&mut ActiveObject, &Transform),
        (With<Player>, Without<Enemy>, Without<MovableObject>),
    >,
    mut enemies: Query<
        (&mut ActiveObject, &mut Transform),
        (With<Enemy>, Without<Player>, Without<MovableObject>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (mut active, transform) in actives.iter_mut() {
        for (o, t) in enemies.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                active.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                o.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => {
                        active.velocity.x = 0.;
                        active.projected_position.x =
                            t.translation.x - (PLAYER_SZ / 2.) - PLAYER_SZ / 2.;
                    }
                    Collision::Right => {
                        active.velocity.x = 0.;
                        active.projected_position.x =
                            t.translation.x + (PLAYER_SZ / 2.) + PLAYER_SZ / 2.;
                    }
                    Collision::Top => {
                        if active.velocity.y < 0. {
                            active.velocity.y = 0.;
                            active.grounded = true;
                        }
                        active.projected_position.y =
                            t.translation.y + (PLAYER_SZ / 2.) + PLAYER_SZ / 2.;
                    }
                    Collision::Bottom => {
                        active.velocity.y = 0.;
                        active.projected_position.y =
                            t.translation.y - (PLAYER_SZ / 2.) - PLAYER_SZ / 2.;
                    }
                    Collision::Inside => {
                        active.velocity = Vec2::new(0., 0.);
                    }
                }
            }
        }
    }
}

//used for debugging and finding tile coordinates, nothing else. Player start tile is considered (0,0) for sanity.
fn my_cursor_system(
    mouse_input: Res<Input<MouseButton>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        if mouse_input.just_pressed(MouseButton::Left) {
            eprintln!(
                "World coords: {}/{}",
                (world_pos.x / 32.).round(),
                (world_pos.y / 32.).round()
            );
        }
    }
}

fn object_collisions(
    mut movables: Query<
        (&mut Object, &mut ActiveObject, &mut Transform),
        (With<MovableObject>, Without<Player>, Without<Enemy>),
    >,
    mut player: Query<(&mut ActiveObject, &mut Transform), (With<Player>, Without<MovableObject>)>,
    //  mut objects2: Query<(&mut Object,&mut ActiveObject, &mut Transform), (With<MovableObject>, Without<Player>, Without<Enemy>)>,
) {
    let (mut pao, pt) = player.single_mut();
    for (mut o, mut ao, mut t) in movables.iter_mut() {
        let hit_top_half = bevy::sprite::collide_aabb::collide(
            pao.projected_position,
            //ne    ed to change this to get the size of whatever the object is
            Vec2::new(PLAYER_SZ, PLAYER_SZ),
            ao.projected_position,
            Vec2::new(o.width, o.height),
        );
        if hit_top_half.is_some() {
            //if player collides with movable object
            let coll_type: bevy::sprite::collide_aabb::Collision = hit_top_half.unwrap();
            match coll_type {
                Collision::Top => {
                    pao.velocity.y = 0.;
                    pao.grounded = true;
                    ao.velocity.y = 0.;
                }
                Collision::Left => {
                    //t.rotate_z(-0.1);
                    if pao.velocity.x > 0. {
                        ao.velocity.x = pao.velocity.x;
                    }
                    pao.projected_position.x = t.translation.x - (PLAYER_SZ / 2.) - o.width / 2.;
                }
                Collision::Right => {
                    if pao.velocity.x < 0. {
                        ao.velocity.x = pao.velocity.x;
                    }
                    pao.projected_position.x = t.translation.x + (PLAYER_SZ / 2.) + o.width / 2.;
                }
                Collision::Bottom => {
                    pao.velocity.y = 0.;
                    ao.velocity.y = 0.;
                }
                Collision::Inside => {}
            }
        } else {
            ao.velocity.x = 0.;
        }
    }
}

fn update_positions(
    mut actives: Query<(&ActiveObject, &mut Transform), (With<ActiveObject>, Without<Player>)>,
    mut objects: Query<(&Object, &mut Transform), (With<Object>, Without<ActiveObject>)>,
    mut player: Query<(&ActiveObject, &mut Transform), With<Player>>,
    mut cam: Query<&mut Transform, (With<Camera>, Without<Object>, Without<ActiveObject>)>,
) {
    //update position of active objects based on projected position from apply_collisions()

    for (o, mut t) in actives.iter_mut() {
        t.translation = o.projected_position;
    }
    //update player position and camera position
    let (mut pl, mut pt) = player.single_mut();
    let mut camera = cam.single_mut();
    pt.translation = pl.projected_position;
    if pt.translation.x + WIN_W / 2. < MAP_W / 2. && pt.translation.x - WIN_W / 2. > -MAP_W / 2. {
        camera.translation.x = pt.translation.x;
    } else if pt.translation.x > 0. {
        camera.translation.x = MAP_W / 2. - WIN_W / 2.;
    } else {
        camera.translation.x = -MAP_W / 2. + WIN_W / 2.;
    }
    if pt.translation.y + WIN_H / 2. < MAP_H / 2. && pt.translation.y - WIN_H / 2. > -MAP_H / 2. {
        camera.translation.y = pt.translation.y;
    } else if pt.translation.y > 0. {
        camera.translation.y = MAP_H / 2. - WIN_H / 2.;
    } else {
        camera.translation.y = -MAP_H / 2. + WIN_H / 2.;
    }
    camera.translation.y = pt.translation.y;
}
//temporary code, should just apply gravity until they hit the ground, for now, enemies jump with j
//eventually, enemy movement decisions can be implemented in a separate file, their results will determine which action they take
//ex. for enemy in enemies, 1. calc sight 2. make decision on where to go 3. execute one of the select motion commands
fn move_enemies(
    input: Res<Input<KeyCode>>,
    mut enemies: Query<
        (&mut ActiveObject, &Transform, &mut Enemy),
        (With<Enemy>, Without<MovableObject>),
    >,
) {
    for (mut enemy, et, mut e) in enemies.iter_mut() {
        let mut change = Vec2::splat(0.);
        //if input.pressed(KeyCode::G){ //comment out when enemy should move freely
        e.decide_motion(Vec2::new(et.translation.x, et.translation.y), enemy.health);
        if e.recover_health {
            enemy.health += 5;
        }

        match e.motion {
            Motion::Left => {
                enemy.velocity.x = -PLAYER_SPEED;
                enemy.velocity.y += GRAVITY;
            }
            Motion::Right => {
                enemy.velocity.x = PLAYER_SPEED;
                enemy.velocity.y += GRAVITY;
            }
            Motion::Jump => {
                if enemy.grounded {
                    enemy.velocity.y = 10.;
                    change.y = enemy.velocity.y;
                    e.motion = Motion::Fall;
                } else {
                    enemy.velocity.y += GRAVITY;
                }
            }
            Motion::JumpRight => {
                if enemy.grounded {
                    enemy.velocity.y = 10.;
                    change.y = enemy.velocity.y;
                    e.motion = Motion::Right;
                }
            }
            Motion::JumpLeft => {
                if enemy.grounded {
                    enemy.velocity.y = 10.;
                    change.y = enemy.velocity.y;

                    e.motion = Motion::Left;
                }
            }
            Motion::Fall => {
                enemy.velocity.x = 0.;
                enemy.velocity.y += GRAVITY;
            }
            Motion::Stop => {
                enemy.velocity.x = 0.;
                enemy.velocity.y += GRAVITY;
            }
        }
        change.y = enemy.velocity.y;
        change.x = enemy.velocity.x;
        //}  //comment out when enemy should move freely
        //this holds the position the player will end up in if there is no collision
        enemy.projected_position = et.translation + Vec3::new(change.x, change.y, 0.);
        enemy.grounded = false;
    }
}

fn gravity_on_movables(
    mut movables: Query<(&Object, &mut ActiveObject, &Transform), With<MovableObject>>,
    mut objects: Query<(&Object, &mut Transform), (With<Object>, Without<ActiveObject>)>,
) {
    for (mut mo, mut active, mt) in movables.iter_mut() {
        if !active.grounded {
            active.velocity.y += GRAVITY;
        }

        active.projected_position =
            mt.translation + Vec3::new(active.velocity.x, active.velocity.y, 0.);
        active.grounded = false;
    }
}

fn attack_enemies(
    mut enemies: Query<(&mut ActiveObject, &Transform, &mut Enemy), With<Enemy>>,
    mut commands: Commands,
) {
    
    for (mut enemy, et, mut e) in enemies.iter_mut() {
        
        let hitbox: Vec3;
        
        match &e.attack {
            Attack::Up => {
                match &e.t{
                    Type::Melee =>{
                        hitbox = Vec3::new(et.translation.x, et.translation.y + PLAYER_SZ, 0.);
                         commands
                    .   spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: hitbox,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MeleeBox::new(hitbox));
                    }
                    Type::Ranged =>{}
                    Type::Other =>{}
                }               
                
            }
            Attack::Down => {
                match &e.t{
                    Type::Melee =>{
                        hitbox = Vec3::new(et.translation.x, et.translation.y - PLAYER_SZ, 0.);
                         commands
                        .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: hitbox,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MeleeBox::new(hitbox));
                    }
                    Type::Ranged =>{}
                    Type::Other =>{}
                }
                
            }
            Attack::Left => {
                match &e.t{
                    Type::Melee =>{
                        hitbox = Vec3::new(et.translation.x - PLAYER_SZ, et.translation.y, 0.);
                        commands
                         .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: hitbox,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MeleeBox::new(hitbox));
                    }
                    Type::Ranged =>{
                        let vel = Vec2::new(-15., 4.);
                        commands
                        .spawn_bundle(SpriteBundle {
                          sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ)),
                        ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(et.translation.x - PLAYER_SZ, et.translation.y, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Projectile::new(vel, ProjType::EnemyProjectile));
                    }
                    Type::Other =>{}
                }
                
            }
            Attack::Right => {
                match &e.t{
                    
                    Type::Melee =>{
                        hitbox = Vec3::new(et.translation.x + PLAYER_SZ, et.translation.y, 0.);
                        commands
                       .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.)),
                            ..default()
                        },
                        transform: Transform {
                            translation: hitbox,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MeleeBox::new(hitbox));
                    }
                    Type::Ranged =>{

                        let vel = Vec2::new(15., 4.);
                        commands
                        .spawn_bundle(SpriteBundle {
                          sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ)),
                        ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(et.translation.x + PLAYER_SZ, et.translation.y, 2.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Projectile::new(vel, ProjType::EnemyProjectile));
                        
                    }
                    Type::Other =>{}
                }
                
            }
            Attack::None => {}
        }
    }
}

fn meleebox_collisions(
    melee_box: Query<(&MeleeBox, Entity), (With<MeleeBox>, Without<Player>)>,
    mut commands: Commands,
    mut player: Query<(&ActiveObject, &mut Player), With<Player>>,
    mut objects: Query<
        (&mut Object, &Transform, Entity),
        (With<Object>, Without<Player>, Without<Projectile>),
    >,
) {
    for (obj, entity) in melee_box.iter() {
        for (pl, mut p) in player.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                obj.position,
                Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.),
                pl.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res.is_some() {
                commands.entity(entity).despawn();
                p.health -= 5;
            }
        }
        for (mut object, object_t, object_entity) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                obj.position,
                Vec2::new(PLAYER_SZ * 2., PLAYER_SZ * 2.),
                object_t.translation,
                Vec2::new(object.width, object.height),
            );
            if res.is_some() {
                if matches!(object.obj_type, ObjectType::Breakable)
                    || matches!(object.obj_type, ObjectType::Breakable)
                {
                    object.broken = true;
                }
            }
        }
    }
}

fn move_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut ActiveObject, &Transform, &mut Player), (With<Player>)>,
    //mut exit: EventWriter<AppExit>,
) {
    let (mut pl, pt, mut p) = player.single_mut();
    if input.pressed(KeyCode::A) {
        pl.facing_left = true;
        if pl.velocity.x > -PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x - 1.;
        }
    } else if pl.velocity.x < 0. {
        pl.velocity.x = pl.velocity.x + 1.;
    }

    if input.pressed(KeyCode::D) {
        pl.facing_left = false;
        if pl.velocity.x < PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x + 1.;
        }
    } else if pl.velocity.x > 0. {
        pl.velocity.x = pl.velocity.x - 1.;
    }

    if pl.velocity.x == 0. && pl.velocity.y == 0. && input.pressed(KeyCode::H) && p.health < 100 {
        if p.healing_bar == 240 {
            p.health += 20;
            p.healing_bar = 0;
        } else {
            p.healing_bar += 1;
        }
    }

    let mut change = Vec2::splat(0.);
    change.x = pl.velocity.x;

    if input.just_pressed(KeyCode::J) {
        //press to rotate item
        let newI: usize = ((p.active_item + 1) % (p.items.len() as usize)) as usize;
        p.active_item = newI;
        let item = p.items.get(p.active_item);
        match item.unwrap() {
            ItemType::None => {
                println!("No active item!")
            }
            ItemType::Jetpack => {
                println!("Jetpack is on!")
            }
            ItemType::Umbrella => {
                println!("Umbrella activated!")
            }
            ItemType::Boots => {
                println!("Jumping boots are on!")
            }
        }
    }
    if input.pressed(KeyCode::Space) {
        let item = p.items.get(p.active_item);
        match item.unwrap() {
            ItemType::None => {
                if pl.grounded {
                    pl.velocity.y = 10.;
                    change.y = 10.;
                } else {
                    pl.velocity.y += GRAVITY;
                    change.y = pl.velocity.y;
                }
            }
            ItemType::Jetpack => {
                if (pl.velocity.y < 7.5) {
                    pl.velocity.y += 0.5;
                }
                change.y = pl.velocity.y;
            }
            ItemType::Umbrella => {
                if pl.grounded {
                    pl.velocity.y = 10.;
                    change.y = 10.;
                } else {
                    if pl.velocity.y <= UMBRELLA_VELOCITY {
                        //open umbrella when going down
                        pl.velocity.y = UMBRELLA_VELOCITY;
                    } else {
                        pl.velocity.y += GRAVITY;
                    }
                    change.y = pl.velocity.y;
                }
            }
            ItemType::Boots => {
                if pl.grounded {
                    pl.velocity.y = 15.;
                    change.y = 15.;
                } else {
                    pl.velocity.y += GRAVITY;
                    change.y = pl.velocity.y;
                }
            }
        }
    }
    //if the player did not just jump, add gravity to move them downward (colon for gounded found later)
    else if pl.grounded {
        pl.velocity.y += 0.0;
        change.y = pl.velocity.y;
    } else if !(pl.grounded) {
        //print!("Applying Gravity");
        let item = p.items.get(p.active_item).unwrap();

        if matches!(item, ItemType::Umbrella) {
            if input.pressed(KeyCode::S) || (pl.velocity.y > UMBRELLA_VELOCITY) {
                //if they press down, they can close the umbrella
                pl.velocity.y += GRAVITY;
            } else {
                //open umbrella when going down
                pl.velocity.y = UMBRELLA_VELOCITY;
            }
            change.y = pl.velocity.y;
        } else {
            pl.velocity.y += GRAVITY;
            change.y = pl.velocity.y;
        }
    }

    //this holds the position the player will end up in if there is no collision
    pl.projected_position = pt.translation + Vec3::new(change.x, change.y, 0.);
    pl.grounded = false;
}

fn attack(
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut ActiveObject, &mut Transform), With<Player>>,
    mut objects: Query<(&mut Object, &Transform, Entity), (With<Object>, Without<Player>)>,
    mut commands: Commands,
) {
    let (pl, pt) = player.single_mut();
    if input.just_pressed(KeyCode::K) {
        let hitbox_pos: Vec3;
        if input.pressed(KeyCode::S) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y - PLAYER_SZ, 0.);
        } else if input.pressed(KeyCode::W) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y + PLAYER_SZ, 0.);
        } else {
            if pl.facing_left {
                hitbox_pos = Vec3::new(pt.translation.x - PLAYER_SZ, pt.translation.y, 0.);
            } else {
                hitbox_pos = Vec3::new(pt.translation.x + PLAYER_SZ, pt.translation.y, 0.);
            }
        }
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: hitbox_pos,
                    ..default()
                },

                ..default()
            })
            .insert(Hitbox::new());
    }
}

fn attack_static(
    mut objects: Query<(&mut Object, &Transform, Entity), (With<Object>, Without<Player>)>,
    mut commands: Commands,
    mut hitbox: Query<(&mut Hitbox, &Transform, Entity), With<Hitbox>>,
) {
    for (hb, hb_t, hb_e) in hitbox.iter_mut() {
        for (mut _o, t, entity) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                hb_t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(_o.width, _o.height),
            );
            if res.is_some() {
                if (matches!(_o.obj_type, ObjectType::Barrel)
                    || matches!(_o.obj_type, ObjectType::Breakable))
                {
                    _o.broken = true;
                }
            }
        }
    }
}

fn attack_active(
    mut actives: Query<
        (&mut ActiveObject, &Transform, Entity),
        (With<ActiveObject>, Without<Player>, Without<Projectile>),
    >,
    mut commands: Commands,
    mut hitbox: Query<(&mut Hitbox, &Transform, Entity), With<Hitbox>>,
) {
    for (hb, hb_t, hb_e) in hitbox.iter_mut() {
        for (mut a, a_t, a_e) in actives.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                hb_t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                a_t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res.is_some() {
                a.health -= 25;
            }
        }
        commands.entity(hb_e).despawn();
    }
}

fn kill_enemies(
    mut actives: Query<
        (&mut ActiveObject, &Transform, Entity),
        (With<ActiveObject>, Without<Player>, Without<Projectile>),
    >,
    mut commands: Commands,
    mut player: Query<(&mut Player), With<Player>>,
) {
    for (mut a, a_t, a_e) in actives.iter_mut() {
        if a.health <= 0 {
            let mut rng = rand::thread_rng();
            for i in 1..6 {
                let sz = 48. / rng.gen_range(8, 16) as f32;
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(sz, sz)),
                            ..default()
                        },
                        transform: Transform {
                            translation: a.projected_position,
                            ..default()
                        },
                        // texture: asset_server.load("bullet.png"),
                        ..default()
                    })
                    .insert(Projectile::new(
                        Vec2::new(rng.gen_range(-5, 5) as f32, rng.gen_range(2, 7) as f32),
                        ProjType::BrokenObj,
                    ))
                    .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
            }
            commands.entity(a_e).despawn();
            let mut p = player.single_mut();
            p.credits += 50;
        }
    }
}

//Press X to pause the timer, press c to unpause it
fn show_gui(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    //mut commands: Commands,
    //asset_server: Res<AssetServer>,
    mut player: Query<(&Player, &mut Transform), With<Player>>,
    mut clock: ResMut<Clock>,
    mut text: Query<&mut Text, (With<ClockText>, Without<CreditText>, Without<HealthBar>)>,
    mut credit_text: Query<&mut Text, (With<CreditText>, Without<ClockText>, Without<HealthBar>)>,
    mut healthbar: Query<&mut Text, (With<HealthBar>, Without<ClockText>, Without<CreditText>)>,
) {
    let (p, mut pt) = player.single_mut();
    //create_timer(commands, asset_server, time);
    if pt.translation.y < -400. {
        clock.timer.pause();
    } else {
        clock.timer.tick(time.delta());
    }

    let time_remaining = (START_TIME - clock.timer.elapsed_secs()).round();
    //println!("{}", time_remaining);
    for mut text in &mut text {
        if time_remaining > 0.0 {
            text.sections[0].value = time_remaining.to_string();
        }
        if input.pressed(KeyCode::X) {
            clock.timer.pause();
        }
        if input.pressed(KeyCode::C) {
            clock.timer.unpause();
        }
        if clock.timer.finished() {
            println!("Resetting position");
            pt.translation = Vec3::new(0., 64., 0.);
        }
    }

    for mut text in &mut credit_text {
        text.sections[0].value = p.credits.to_string();
    }

    for mut text in &mut healthbar {
        text.sections[0].value = p.health.to_string();
    }
}

fn item_shop(
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut Transform), With<Player>>,
    mut clock: ResMut<Clock>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (mut p, mut pt) = player.single_mut();
    if input.just_pressed(KeyCode::I) && pt.translation.y > -400. {
        println!("\nSHOP INFO: PRESS B WHILE STANDING UNDER ITEM OF CHOICE\nUmbrella: {} Credits\nJumping Boots: {} Credits\nJetpack Price: {} Credits", UMBRELLA_PRICE,BOOTS_PRICE,JETPACK_PRICE);
        clock.timer.pause();
        pt.translation = Vec3::new(0., -575., 0.);

        let mut id = 0;
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(75., 75.)),
                    ..default()
                },
                texture: asset_server.load("jetpack.png"),
                transform: Transform {
                    translation: Vec3::new(200., -400., 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Object::new(id, 50., 50., ObjectType::Active));
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(75., 75.)),
                    ..default()
                },
                texture: asset_server.load("umbrella.png"),
                transform: Transform {
                    translation: Vec3::new(-200., -400., 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Object::new(id, 50., 50., ObjectType::Active));
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(75., 75.)),
                    ..default()
                },
                texture: asset_server.load("boots.png"),
                transform: Transform {
                    translation: Vec3::new(0., -400., 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Object::new(id, 50., 50., ObjectType::Active));
    } else if pt.translation.y <= -400. {
        if input.just_pressed(KeyCode::I) {
            pt.translation = Vec3::new(0., 64., 0.);
            clock.timer.unpause();
        }
        if input.just_pressed(KeyCode::B) {
            if pt.translation.x <= -100. && p.credits >= UMBRELLA_PRICE {
                //IF TRY TO BUY UMBRELLA
                if p.items.contains(&ItemType::Umbrella) {
                    println!("Umbrella already purchased!");
                } else {
                    p.credits -= UMBRELLA_PRICE;
                    p.items.push(ItemType::Umbrella);
                    print!("UMBRELLA PURCHASED!");
                }
            } else if pt.translation.x >= 100. && p.credits >= JETPACK_PRICE {
                //IF TRY TO BUY JETPACK
                if p.items.contains(&ItemType::Umbrella) {
                    println!("Jetpack already purchased!");
                } else {
                    p.credits -= JETPACK_PRICE;
                    p.items.push(ItemType::Jetpack);
                    print!("JETPACK PURCHASED!");
                }
            } else if p.credits >= BOOTS_PRICE {
                //IF TRY TO BUY BOOTS
                if p.items.contains(&ItemType::Umbrella) {
                    println!("Boots already purchased!");
                } else {
                    p.credits -= BOOTS_PRICE;
                    p.items.push(ItemType::Boots);
                    print!("BOOTS PURCHASED!");
                }
            }
            println!("PRESS I TO RETURN!");
        }
    }
}

fn player_health(
    mut player: Query<(&mut Player), With<Player>>,
    mut exit: EventWriter<AppExit>,
    // mut healthbar: Query<(Entity), With<HealthBar>>,
    // mut commands: Commands,
) {
    let (p) = player.single_mut();
    // let e = healthbar.single_mut();
    // commands.entity(e).despawn();
    if p.health <= 0 {
        exit.send(AppExit);
        print!("You lose!");
    }
}

fn barrels_with_barrels(
    mut movables: Query<
        (&mut Object, &mut ActiveObject, &mut Transform),
        (With<MovableObject>, Without<Player>, Without<Enemy>),
    >,
) {
    let mut combinations = movables.iter_combinations_mut();
    while let Some([(mut mo, mut mao, mut mt), (mut mo2, mut mao2, mut mt2)]) =
        combinations.fetch_next()
    {
        // mutably access components data
        let coll = bevy::sprite::collide_aabb::collide(
            mao.projected_position,
            Vec2::new(mo.width, mo.height),
            mao2.projected_position,
            Vec2::new(mo2.width, mo2.height),
        );
        if coll.is_some() {
            let coll_type = coll.unwrap();
            if (matches!(mo.obj_type, ObjectType::Barrel)
                || matches!(mo.obj_type, ObjectType::Breakable))
            {
                match coll_type {
                    Collision::Left => {
                        if mao2.velocity.x != 0. {
                            mao.velocity = mao2.velocity;
                            mt.translation.x = mt2.translation.x + mo.width;
                        } else if mao.velocity.x != 0. {
                            mao2.velocity = mao.velocity;
                            mt2.translation.x = mt.translation.x - mo.width;
                        }
                    }
                    Collision::Right => {
                        if mao2.velocity.x != 0. {
                            mao.velocity = mao2.velocity;
                            mt.translation.x = mt2.translation.x + mo.width;
                        } else if mao.velocity.x != 0. {
                            mao2.velocity = mao.velocity;
                            mt2.translation.x = mt.translation.x - mo.width;
                        }
                    }
                    Collision::Top => {
                        mao2.velocity.y = 0.;
                    }
                    // Collision::Inside => {
                    //     if (mt.translation.x < mt2.translation.x) {
                    //         mt.translation.x = mt2.translation.x - mo.width;
                    //     } else {
                    //         mt.translation.x = mt2.translation.x + mo.width;
                    //     }
                    // }
                    _ => {
                        if mao2.velocity.x != 0. {
                            mao.velocity = mao2.velocity;
                            mt.translation.x = mt2.translation.x + mo.width;
                        } else if mao.velocity.x != 0. {
                            mao2.velocity = mao.velocity;
                            mt2.translation.x = mt.translation.x + mo.width;
                        }
                    }
                }
            }
        }
    }
}
