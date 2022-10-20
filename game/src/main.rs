//imports from outside crates
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::Collision;
use bevy::{prelude::*, window::PresentMode};
use std::collections::HashSet;
use std::convert::From;

//imports from local creates
mod util;
use crate::util::*;

mod active_util;
use crate::active_util::*;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

/*
#[derive(Component)]
struct Ground;
*/

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
        .add_system(attack)
        //.add_system(camera_follow)
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
        .insert(Velocity::new())
        .insert(Player::new());

    //main floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(1856., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(912., 0., 2.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(1856., 32.))
        .insert(Object::new(0));

    //ceiling
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(1920., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(912., 1024., 2.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(1920., 32.))
        .insert(Object::new(0));

    //left wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 1024.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-32., 496., 2.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 1024.))
        .insert(Object::new(0));

    //right wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32., 1024.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1856., 496., 2.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 1024.))
        .insert(Object::new(0));

    //first 1x2 wall on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(192., 48., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 64.))
        .insert(Object::new(0));

    //second 1x2 wall on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(576., 48., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 64.))
        .insert(Object::new(0));

    //2x3 block on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(64., 96.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1040., 64., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(64., 96.))
        .insert(Object::new(0));

    //last 1x2 block on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1312., 48., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 64.))
        .insert(Object::new(0));

    //first floating 1x1 on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1472., 64., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //second floating 1x1 on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1632., 64., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //grounded 1x1 on first floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1728., 32., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //second floor from left side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(992., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(480., 192., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(992., 32.))
        .insert(Object::new(0));

    //second floor from right side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(768., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1456., 192., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(768., 32.))
        .insert(Object::new(0));

    //first 1x2 wall on second floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(224., 240., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 64.))
        .insert(Object::new(0));

    //2x3 block on second floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(64., 96.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(848., 256., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(64., 96.))
        .insert(Object::new(0));

    //platform above second floor jutting out from left wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(192., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(80., 352., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(192., 32.))
        .insert(Object::new(0));

    //platform floating above second floor in left-middle
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(512., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(528., 384., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(512., 32.))
        .insert(Object::new(0));

    //floor of box on upper-mid right side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(768., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1456., 352., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(768., 32.))
        .insert(Object::new(0));

    //left wall of box on upper-mid right side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 384.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1088., 464., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 384.))
        .insert(Object::new(0));

    //ceiling of box on upper-mid right side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(512., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1360., 640., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(512., 32.))
        .insert(Object::new(0));

    //floating platform in box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(512., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1424., 480., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(512., 32.))
        .insert(Object::new(0));

    //1x1 on floating platform in box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1376., 512., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 on right wall in box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1824., 576., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 on right corner in box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1824., 384., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 on right corner in box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1440., 384., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));
    //1x1 in left corner of box on upper-mid side
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1120., 384., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x2 floating in air
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(960., 432., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 64.))
        .insert(Object::new(0));

    //1x3 on platform above second floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 96.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(608., 448., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 96.))
        .insert(Object::new(0));

    //wall beside 1x3
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 192.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(512., 496., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 192.))
        .insert(Object::new(0));
    //floor connected to ^ wall
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(512., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(240., 576., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(512., 32.))
        .insert(Object::new(0));

    //1x1 sitting on ^ floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 608., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 floating in air below enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(896., 608., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 floating in air above enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(928., 800., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(736., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1472., 768., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(736., 32.))
        .insert(Object::new(0));

    //1x1 on wall below floating 1x1
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1056., 512., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //floor to room left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(768., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(400., 704., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(768., 32.))
        .insert(Object::new(0));

    //1x1 in room to left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(448., 736., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //wall in room to left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 192.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(288., 816., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 192.))
        .insert(Object::new(0));

    //1x1 on left side of wall in room to left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(256., 800., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //1x1 on right side of wall in room to left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(320., 832., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(0));

    //right wall of room to left of enemy floor
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::BLACK,
                custom_size: Some(Vec2::new(32., 288.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(512., 864., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 288.))
        .insert(Object::new(0));

    /*
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                //color: Color::GRAY,
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            texture: asset_server.load("spikes.png"),
            transform: Transform {
                translation: Vec3::new(318., -136., 1.),
                ..default()
            },
            ..default()
        })
        .insert(Rect::new(32., 32.))
        .insert(Object::new(1));
        */
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

    if input.pressed(KeyCode::Q) {
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
    mut player: Query<
        (&mut Player, &mut Transform, &mut Velocity),
        (With<Player>, Without<Object>),
    >,
    objects: Query<(&Object, &Rect, &Transform), (With<Object>, Without<Player>)>,
    mut exit: EventWriter<AppExit>,
    mut cam: Query<&mut Transform, (With<Camera>, Without<Object>, Without<Player>)>,
) {
    let (mut pl, mut pt, mut pv) = player.single_mut();

    let mut camera = cam.single_mut();
    if input.pressed(KeyCode::A) {
        pl.facing_left = true;
        if pv.velocity.x > -PLAYER_SPEED {
            pv.velocity.x = pv.velocity.x - 20.;
        }
    } else if pv.velocity.x < 0. {
        pv.velocity.x = pv.velocity.x + 20.;
    }

    if input.pressed(KeyCode::D) {
        pl.facing_left = false;
        if pv.velocity.x < PLAYER_SPEED {
            pv.velocity.x = pv.velocity.x + 20.;
        }
    } else if pv.velocity.x > 0. {
        pv.velocity.x = pv.velocity.x - 20.;
    }

    if pv.velocity.y > TERMINAL_VELOCITY {
        pv.velocity.y += GRAVITY;
    }

    if input.pressed(KeyCode::Space) && pl.grounded {
        pv.velocity.y = PLAYER_SPEED * 2.;
    }

    pl.grounded = false;
    let deltat = time.delta_seconds();

    let change = pv.velocity * deltat;

    let mut new_pos = pt.translation + Vec3::new(change.x, change.y, 0.);
    //this variable will track where the player will end up if there is no collision with a surface
    for (_o, r, t) in objects.iter() {
        let res = bevy::sprite::collide_aabb::collide(
            new_pos,
            Vec2::new(PLAYER_SZ, PLAYER_SZ),
            t.translation,
            Vec2::new(r.width, r.height),
        );
        if res.is_some() {
            let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
            match coll_type {
                Collision::Left => {
                    pv.velocity.x = 0.;
                    new_pos.x = t.translation.x - (r.width / 2.) - PLAYER_SZ / 2.;
                }
                Collision::Right => {
                    pv.velocity.x = 0.;
                    new_pos.x = t.translation.x + (r.width / 2.) + PLAYER_SZ / 2.;
                }
                Collision::Top => {
                    if pv.velocity.y < 0. {
                        //if falling down
                        pv.velocity.y = 0.; //stop vertical velocity
                        pl.grounded = true;
                    }
                    new_pos.y = t.translation.y + (r.height / 2.) + PLAYER_SZ / 2.;
                    if _o.id == 1 {
                        exit.send(AppExit);
                    }
                }
                Collision::Bottom => {
                    pv.velocity.y = 0.;
                    new_pos.y = t.translation.y - (r.height / 2.) - PLAYER_SZ / 2.;
                }
                Collision::Inside => {
                    println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                    pv.velocity = Vec2::new(0., 0.);
                }
            }
        }
    }

    pt.translation = new_pos;
    camera.translation.x = pt.translation.x;
    camera.translation.y = pt.translation.y;
}

fn attack(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Player, &mut Transform, &mut Velocity),
        (With<Player>, Without<Object>),
    >,
    objects: Query<(&Object, &Rect, &Transform), (With<Object>, Without<Player>)>,
    mut commands: Commands,
) {
    let (pl, pt, pv) = player.single_mut();
    if input.just_pressed(KeyCode::P) {
        let mut hitbox_pos;
        if (input.pressed(KeyCode::S)) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y - PLAYER_SZ, 0.);
        } else if (pv.velocity.y != 0.) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y + PLAYER_SZ, 0.);
        } else if (!pl.facing_left) {
            hitbox_pos = Vec3::new(pt.translation.x + PLAYER_SZ, pt.translation.y, 0.);
        } else {
            hitbox_pos = Vec3::new(pt.translation.x - PLAYER_SZ, pt.translation.y, 0.);
        }
        for (_o, r, t) in objects.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                hitbox_pos,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(r.width, r.height),
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
                        if (pt.translation.y - PLAYER_SZ / 2. >= t.translation.y + PLAYER_SZ / 2.) {
                            println!("Attacked object below player");
                        } else if (pt.translation.x > t.translation.x) {
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
/*
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut Camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player = player_query.single();
    let mut camera = Camera_query.single_mut();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
*/
