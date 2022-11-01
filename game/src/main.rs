//imports from outside crates
use bevy::app::AppExit;
use bevy::asset;
use bevy::render::camera::RenderTarget;
use bevy::sprite::collide_aabb::Collision;
use bevy::{prelude::*, window::PresentMode};
use bevy::time::FixedTimestep;

//imports from local creates
mod util;
use crate::util::*;

mod active_util;
use crate::active_util::*;

mod ai;
use crate::ai::*;

mod movement_mesh;
use crate::movement_mesh::*;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);
const START_TIME: f32 = 15.;

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
            }
            else if matches!(desc.obj_type, ObjectType::Item){
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
            }
            else if matches!(desc.obj_type, ObjectType::UmbrellaItem){
                commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
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
            else if matches!(desc.obj_type, ObjectType::JetpackItem){
                commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
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
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(desc.width, desc.height)),
                        ..default()
                    },
                    texture: asset_server.load(texture_path),
                    transform: Transform {
                        translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Object::new(id, desc.width, desc.height, desc.obj_type));
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
    for v in mesh.vertices {
        commands.spawn()
            .insert(v);
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
        
        
     //   .add_system(enemy_collisions)
        //.add_system(show_popup)

        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(RUNTIME))
        //         .with_system(move_player)
        //         .with_system(update_positions)
        //         .with_system(enemy_collisions)
        //         .with_system(apply_collisions)
        //         .with_system(move_enemies)
                

        //  )
        //         .add_system(calculate_sight)
        //         .add_system(my_cursor_system)
        //         .add_system(show_gui)
        //         .add_system(item_shop)

        .add_system(move_player.after(show_gui).before(enemy_collisions).before(apply_collisions))
        .add_system(enemy_collisions)
        .add_system(apply_collisions.after(enemy_collisions))
        
        .add_system(update_positions.after(apply_collisions))
        .add_system(move_enemies.after(move_player).before(enemy_collisions).before(apply_collisions))
        .add_system(my_cursor_system)
        .add_system(show_gui)
        .add_system(calculate_sight.after(update_positions))
        .add_system(item_shop.before(show_gui))
        .add_system(attack)
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

    commands.insert_resource(Clock {
        // create the repeating timer
        timer: Timer::from_seconds(START_TIME, true),
    });

    //This is for the overlay
    //Putting comments for every object so we know which is which. This is a bad idea for future levels but for now but it gets a basis going.
    /*
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        texture: asset_server.load("Room_1.png"),
        transform: Transform::from_xyz(912., 500., 0.),
        ..default()
    });
    */

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
        translation: Vec3::new(0., -1000., 900.),
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
        .insert(Object::new(-1, PLAYER_SZ, PLAYER_SZ, ObjectType::Active))
        .insert(Player::new());

    //improved code to spawn in all walls of a level
    let mut level = get_level(0);
    let mesh = get_level_mesh(0);
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
//needs some serious refactoring
fn calculate_sight(
    player: Query<(&Object, &Transform), (With<ActiveObject>, With<Player>)>,
    mut enemies: Query<(&Object, &Transform, &mut Enemy), (With<ActiveObject>, With<Enemy>)>,
    objects: Query<(&Object, &Transform), (With<Object>, Without<ActiveObject>)>,
) {
    //store data for player and other enemies for later use
    let mut others = Vec::new();
    for (obj, tr, _en) in enemies.iter() {
        let data = (*obj, *tr);
        others.push(data);
    }
    let (obj, tr) = player.single();
    others.push((*obj, *tr));

    let sight_distance = 300.0;

    for (_obj, tr, mut en) in enemies.iter_mut() {
        let pos = tr.translation;
        let mut sight_lines = Vec::new();
        let mut object_lines = Vec::new();

        for (o, t) in objects.iter() {
            //v1 and v2 hold the endpoints for line of sight, v3 holds the corner
            let (v1, v2, v3) = find_vertices(
                pos.x,
                pos.y,
                t.translation.x,
                t.translation.y,
                o.width,
                o.height,
            );
            let d = Descriptor::new2(o.width, o.height, t.translation.x, t.translation.y, o.obj_type, o.id);
            //generate lines of sight
            let s1 = Line::new(Vec2::new(pos.x, pos.y), v1, d);
            let s2 = Line::new(Vec2::new(pos.x, pos.y), v2, d);

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
            //maybe add code to check the corner of objects
            if in_range {
                let o1 = Line::new(v1, v3, d);
                let o2 = Line::new(v2, v3, d);
                object_lines.push(o1);
                object_lines.push(o2);
            }
        }
        for (o, t) in others.iter() {
            //v1 and v2 hold the endpoints for line of sight, v3 holds the corner
            let (v1, v2, v3) = find_vertices(
                pos.x,
                pos.y,
                t.translation.x,
                t.translation.y,
                o.width,
                o.height,
            );
            let d = Descriptor::new2(o.width, o.height, t.translation.x, t.translation.y, o.obj_type, o.id);
            //generate lines of sight
            let s1 = Line::new(Vec2::new(pos.x, pos.y), v1, d);
            let s2 = Line::new(Vec2::new(pos.x, pos.y), v2, d);

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
            //maybe add code to check the corner of objects
            if in_range {
                let o1 = Line::new(v1, v3, d);
                let o2 = Line::new(v2, v3, d);
                object_lines.push(o1);
                object_lines.push(o2);
            }
        }
        en.determine_visibility(sight_lines, object_lines, obj.height);
    }
}

//we will also need to implement collisions between 2 active objects, that is where we will do rigidbody collisions
//I'm not sure whether that should run before or after object collisions
fn apply_collisions(
    mut actives: Query<(&mut ActiveObject, &Transform), With<ActiveObject>>,
    objects: Query<(&Object, &Transform), (With<Object>, Without<ActiveObject>)>,
    input: Res<Input<KeyCode>>,
    //will want to use something different later
    mut exit: EventWriter<AppExit>,
) {
    //loop through all objects that move
    for (mut active, transform) in actives.iter_mut() {
        for (o, t) in objects.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                active.projected_position,
                //need to change this to get the size of whatever the object is
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(o.width, o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => match o.obj_type {
                        ObjectType::JetpackItem => {}
                        ObjectType::UmbrellaItem => {}
                        ObjectType::Spike => {}
                        ObjectType::Item => {}
                        ObjectType::Cobweb => {
                            if active.velocity.x != 0. {
                                active.velocity.x /= 2.;
                            }
                            if active.velocity.y != 0. && active.velocity.y < 5. {
                                active.velocity.y /= 2.;
                            }
                            active.grounded = false;
                        }
                        ObjectType::Block => {
                            active.velocity.x = 0.;
                            active.projected_position.x =
                                t.translation.x - (o.width / 2.) - PLAYER_SZ / 2.;
                        }
                        ObjectType::Active => {}
                    },
                    Collision::Right => match o.obj_type {
                        ObjectType::JetpackItem => {}
                        ObjectType::UmbrellaItem => {}
                        ObjectType::Spike => {}
                        ObjectType::Item => {}
                        ObjectType::Cobweb => {
                            if active.velocity.x != 0. {
                                active.velocity.x /= 2.;
                            }
                            if active.velocity.y != 0. && active.velocity.y < 5. {
                                active.velocity.y /= 2.;
                            }
                            active.grounded = false;
                        }
                        ObjectType::Block => {
                            active.velocity.x = 0.;
                            active.projected_position.x =
                                t.translation.x + (o.width / 2.) + PLAYER_SZ / 2.;
                        }
                        ObjectType::Active => {}
                    },
                    Collision::Top => {
                        match o.obj_type {
                            ObjectType::JetpackItem => {}
                            ObjectType::UmbrellaItem => {}
                            ObjectType::Spike => {
                                exit.send(AppExit);
                            }
                            ObjectType::Item => {}
                            ObjectType::Cobweb => {
                                if active.velocity.y < 0. {
                                    //if falling down
                                    active.velocity.y /= 2.; //stop vertical velocity
                                }
                                if active.velocity.y != 0. && active.velocity.y < 5. {
                                    active.velocity.y /= 2.;
                                }
                                active.grounded = false;
                            }
                            ObjectType::Block => {
                                if active.velocity.y < 0. {
                                    //if falling down
                                    active.velocity.y = 0.; //stop vertical velocity
                                    active.grounded = true;
                                }
                                active.projected_position.y =
                                    t.translation.y + (o.height / 2.) + PLAYER_SZ / 2.;
                            }
                            ObjectType::Active => {}
                        }
                    }
                    Collision::Bottom => {
                        match o.obj_type {
                            ObjectType::JetpackItem => {}
                            ObjectType::UmbrellaItem => {}
                            ObjectType::Spike => {}
                            ObjectType::Item => {}
                            ObjectType::Cobweb => {
                                if active.velocity.y < 0. {
                                    //if falling down
                                    active.velocity.y /= 2.; //stop vertical velocity
                                }
                                if active.velocity.y != 0. && active.velocity.y < 5. {
                                    active.velocity.y /= 2.;
                                }
                                active.grounded = false;
                            }
                            ObjectType::Block => {
                                active.velocity.y = 0.;
                                active.projected_position.y =
                                    t.translation.y - (o.height / 2.) - PLAYER_SZ / 2.;
                            }
                            ObjectType::Active => {}
                        }
                    }
                    Collision::Inside => {
                        match o.obj_type {
                            ObjectType::JetpackItem => {}
                            ObjectType::UmbrellaItem => {}


                            ObjectType::Spike => {println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);}
                            ObjectType::Item => {println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);}
                            ObjectType::Cobweb => {println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);}
                            ObjectType::Block => {println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);}
                            ObjectType::Active => {println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                            active.velocity = Vec2::new(0., 0.);}

                        }
                    }
                }
            }
        }
    }
}

fn enemy_collisions(
    mut actives: Query<
        (&mut ActiveObject,&Transform),
        (With<Player>, Without<Enemy>),
        >,
    mut enemies: Query<
        (&mut ActiveObject, &mut Transform),
        (With<Enemy>, Without<Player>),
        >,
    mut exit: EventWriter<AppExit>,
){
    for (mut active, transform) in actives.iter_mut(){

        for (o, t) in enemies.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                active.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => {
                        active.velocity.x = 0.;
                        active.projected_position.x = t.translation.x - (PLAYER_SZ / 2.) - PLAYER_SZ / 2.;
                    }
                    Collision::Right => {
                        active.velocity.x = 0.;
                        active.projected_position.x = t.translation.x + (PLAYER_SZ / 2.) + PLAYER_SZ / 2.;
                    }
                    Collision::Top => {
                        if active.velocity.y < 0. {
                            active.velocity.y = 0.;
                            active.grounded = false;
                        }
                        active.projected_position.y = t.translation.y + (PLAYER_SZ / 2.) + PLAYER_SZ / 2.;
                    }
                    Collision::Bottom => {
                        active.velocity.y = 0.;
                        active.projected_position.y = t.translation.y - (PLAYER_SZ / 2.) - PLAYER_SZ / 2.;
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
                ((world_pos.y / 32.)).round()
            );
        }
    }
}

fn update_positions(
    mut actives: Query<(&ActiveObject, &mut Transform), (With<ActiveObject>, Without<Player>)>,
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

    camera.translation.x = pt.translation.x;
    camera.translation.y = pt.translation.y;
}
//temporary code, should just apply gravity until they hit the ground, for now, enemies jump with j
//eventually, enemy movement decisions can be implemented in a separate file, their results will determine which action they take
//ex. for enemy in enemies, 1. calc sight 2. make decision on where to go 3. execute one of the select motion commands
fn move_enemies(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut enemies: Query<
        (&mut ActiveObject, &Transform, &mut Enemy),
        (With<Enemy>),
    >,
){
    let deltat = time.delta_seconds();
    for (mut enemy, et, mut e) in enemies.iter_mut(){
        if input.just_pressed(KeyCode::K) {
            println!(
                "For enemy at position {}, {}",
                et.translation.x, et.translation.y
            );
            e.check_visible_objects();
        }
        let mut change = Vec2::splat(0.);
        //need argument to let enemy know about collisions that have occured
        e.decide_motion(&et.translation);
        //if the player did not just jump, add gravity to move them downward (collision for grounded found later)
        
        enemy.velocity.y += GRAVITY * deltat;
        change.y = enemy.velocity.y;
        change.x = enemy.velocity.x * deltat;
        //this holds the position the player will end up in if there is no collision
        enemy.projected_position = et.translation + Vec3::new(change.x, change.y, 0.);
        enemy.grounded = false;

        
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut ActiveObject, &mut Transform), (With<Player>)>,
) {
    let (mut pl, mut pt) = player.single_mut();

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

    let deltat = time.delta_seconds();
    let mut change = Vec2::splat(0.);
    change.x = pl.velocity.x * deltat;
    //the reason that jump height was inconsistent was because this could only happen when on the ground,
    //and it was multiplied by deltat, so faster framerate meant shorter jump
    //this code does fix the issue, but might create a new one (yay...)
    if input.just_pressed(KeyCode::Space) && pl.grounded {
        pl.velocity.y = 8.;
        change.y = 8.;
    }
    //if the player did not just jump, add gravity to move them downward (collision for gounded found later)
    else {
        pl.velocity.y += GRAVITY * deltat;
        change.y = pl.velocity.y;
    }
    //this holds the position the player will end up in if there is no collision
    pl.projected_position = pt.translation + Vec3::new(change.x, change.y, 0.);
    pl.grounded = false;
}


fn attack(
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut ActiveObject, &mut Transform), (With<Player>)>,

    objects: Query<(&Object, &Transform), (With<Object>, Without<Player>)>,
    mut commands: Commands,
) {
    let (pl, pt) = player.single_mut();
    if input.just_pressed(KeyCode::P) {
        let mut hitbox_pos;
        if input.pressed(KeyCode::S) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y - PLAYER_SZ, 0.);// DOWN
        } else if input.pressed(KeyCode::W) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y + PLAYER_SZ, 0.);// UP
        } else if input.pressed(KeyCode::D){
            hitbox_pos = Vec3::new(pt.translation.x + PLAYER_SZ, pt.translation.y, 0.);// RIGHT
        } else {
            hitbox_pos = Vec3::new(pt.translation.x - PLAYER_SZ, pt.translation.y, 0.);//LEFT
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

//Press X to pause the timer, press c to unpause it
fn show_gui(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: Query<(&mut Player, &mut Transform), (With<Player>)>,
    mut clock: ResMut<Clock>,
    mut text: Query<&mut Text, (With<ClockText>, Without<CreditText>, Without<HealthBar>)>,
    mut credit_text: Query<&mut Text, (With<CreditText>, Without<ClockText>, Without<HealthBar>)>,
    mut healthbar: Query<&mut Text, (With<HealthBar>, Without<ClockText>, Without<CreditText>)>,
) {
    let (mut p, mut pt)= player.single_mut();
    //create_timer(commands, asset_server, time);
    if pt.translation.y < -400.{
        clock.timer.pause();
    }
    else{
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
        text.sections[0].value= p.credits.to_string();
    }

    for mut text in &mut healthbar {

        text.sections[0].value=p.health.to_string();
    }
}

fn item_shop(
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut Transform), (With<Player>)>,
    mut clock: ResMut<Clock>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let (mut p, mut pt)= player.single_mut();
    if input.just_pressed(KeyCode::I) && pt.translation.y > -400. {
        print!("\nSHOP INFO: PRESS B ON BLOCK TO BUY\nLEFT: UMBRELLA\nRIGHT: JETPACK\n");
        clock.timer.pause();
        pt.translation = Vec3::new(0., -475., 0.);

        let mut id = 0;
        commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(75., 75.)),
                        ..default()
                    },
                    texture: asset_server.load("jetpack.png"),
                    transform: Transform {
                        translation: Vec3::new(150., -400., 2.),
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
                         translation: Vec3::new(-150., -400., 2.),
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
        // if input.just_pressed(KeyCode::B) {
            
        //     if p.credits >= UMBRELLA_PRICE  { //IF TRY TO BUY UMBRELLA
        //         p.credits-=UMBRELLA_PRICE;
        //         p.item = ItemType::Umbrella;
        //         print!("UMBRELLA PURCHASED!");
        //     } else if p.credits >= JETPACK_PRICE { //IF TRY TO BUY JETPACK
        //         p.credits-=JETPACK_PRICE;
        //         p.item = ItemType::Jetpack;
        //         print!("JETPACK PURCHASED!");
        //     }
        //     print!("\n PRESS I TO RETURN!");
        // }
    }
}
