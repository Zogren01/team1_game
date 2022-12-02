use crate::active_util::*;
use crate::ai::*;
use crate::util::*;
use bevy::asset;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use rand::Rng;

const PROJECTILE_SZ: f32 = 6.;
const PROJECTILE_DAMAGE: i32 = 25;

#[derive(Component)]
pub struct MovableObject;

pub enum ProjType {
    Particle,
    Projectile,
    BrokenObj,
}

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub proj_type: ProjType,
    pub project_pos: Vec3,
}

impl Projectile {
    pub fn new(vel: Vec2, pt: ProjType) -> Self {
        Self {
            velocity: vel,
            proj_type: pt,
            project_pos: Vec3::splat(0.),
        }
    }
}

#[derive(Component)]
pub struct BrokenObj {
    lifespan: Timer,
}

impl BrokenObj {
    pub fn new(lifespan: Timer) -> Self {
        Self { lifespan: lifespan }
    }
}

pub fn shoot(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player: Query<(&mut Player, &mut ActiveObject, &mut Transform), (With<Player>)>,
    asset_server: Res<AssetServer>,
) {
    let (mut p, pl, pt) = player.single_mut();

    let mut vel = Vec2::new(15., 4.);

    if pl.facing_left {
        vel.x *= -1.;
    }
    if input.pressed(KeyCode::W) {
        vel.x = 0.;
        vel.y = 17.;
    } else if input.pressed(KeyCode::S) {
        vel.x = 0.;
        vel.y = -15.;
    }
    vel += pl.velocity;

    if input.just_pressed(KeyCode::L) {
        // for (pla)
        if p.credits > 0 {
            p.credits -= 5;
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(pt.translation.x, pt.translation.y, 2.),
                        ..default()
                    },
                    // texture: asset_server.load("bullet.png"),
                    ..default()
                })
                .insert(Projectile::new(vel, ProjType::Projectile));
        }
    }
}

pub fn projectile_static_collisions(
    mut commands: Commands,
    mut objects: Query<
        (&mut Object, &Transform, Entity),
        (With<Object>, Without<Player>, Without<Projectile>),
    >,
    mut projectiles: Query<
        (&mut Projectile, &mut Transform, Entity),
        (Without<Object>, Without<Player>, Without<Enemy>),
    >,
) {
    //let (pl, pt) = player.single_mut();
    for (mut pro_o, mut pro_t, entity) in projectiles.iter_mut() {
        let mut collide = false;
        pro_o.velocity.y += GRAVITY;
        for (mut o_o, o_t, o_e) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                pro_o.project_pos,
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                o_t.translation,
                Vec2::new(o_o.width, o_o.height),
            );
            if res.is_some() {
                collide = true;
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                let mut time: f32 = 5.0;
                if matches!(pro_o.proj_type, ProjType::Projectile) {
                    commands.entity(entity).despawn();
                    if matches!(o_o.obj_type, ObjectType::Breakable) {
                        // generate_breakables(&coll_type, o_t, o_o, commands);
                        o_o.broken = true;
                    } else if matches!(o_o.obj_type, ObjectType::Barrel) {
                        // generate_breakables(&coll_type, o_t, o_o, commands);
                        o_o.broken = true;
                    }
                } else if matches!(pro_o.proj_type, ProjType::BrokenObj) {
                    match coll_type {
                        Collision::Left => {
                            pro_o.velocity.x *= -0.8;
                        }
                        Collision::Right => {
                            pro_o.velocity.x *= -0.8;
                        }
                        Collision::Top => {
                            // print!("{}\n", pro_o.velocity.y.abs());
                            if (pro_o.velocity.y.abs() < 1.5) {
                                pro_o.velocity.y = 0.;
                            } else {
                                pro_o.velocity.y *= -0.3;
                            }
                            pro_o.velocity.x /= 2.;
                            // pro_t.translation.y =
                            //     o_t.translation.y + o_o.height / 2. + PROJECTILE_SZ / 2.
                        }
                        Collision::Bottom => {
                            pro_o.velocity.y = 0.;
                        }
                        Collision::Inside => {
                            pro_o.velocity.x = 0.;
                            pro_o.velocity.y = 0.;
                            pro_t.translation.y =
                                o_t.translation.y + o_o.height / 2. + PROJECTILE_SZ / 2.
                        }
                    }
                } else if matches!(pro_o.proj_type, ProjType::Particle) {
                    if matches!(o_o.obj_type, ObjectType::Barrel) {
                        o_o.broken = true;
                        commands.entity(entity).despawn();
                    } else {
                        match coll_type {
                            Collision::Left => {
                                pro_o.velocity.x *= -0.8;
                            }
                            Collision::Right => {
                                pro_o.velocity.x *= -0.8;
                            }
                            Collision::Top => {
                                // print!("{}\n", pro_o.velocity.y.abs());
                                pro_o.velocity.y *= -0.5;
                                pro_o.velocity.x *= 0.8;
                                // pro_t.translation.y =
                                //     o_t.translation.y + o_o.height / 2. + PROJECTILE_SZ / 2.
                            }
                            Collision::Bottom => {
                                pro_o.velocity.y *= -1.;
                                pro_o.velocity.x *= -0.9;
                            }
                            Collision::Inside => {
                                pro_o.velocity.y *= -0.8;
                            }
                        }
                    }
                }
            }
        }
        if !collide {
            pro_t.translation = pro_o.project_pos;
        }
    }
}

pub fn despawn_broken_objects(
    time: Res<Time>,
    mut commands: Commands,
    mut brokenObjects: Query<(&mut BrokenObj, Entity), Without<Object>>,
) {
    for (mut obj, entity) in brokenObjects.iter_mut() {
        obj.lifespan.tick(time.delta());
        if obj.lifespan.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_active_collision(
    mut commands: Commands,
    mut projectiles: Query<
        (&mut Projectile, &mut Transform, Entity),
        (
            With<Projectile>,
            Without<Object>,
            Without<Player>,
            Without<Enemy>,
        ),
    >,
    mut actives: Query<
        (&mut ActiveObject, Entity),
        (With<ActiveObject>, Without<Player>, Without<Projectile>),
    >,
    mut player: Query<(&mut Player, &ActiveObject), With<Player>>,
) {
    for (mut pro_o, mut pro_t, entity_p) in projectiles.iter_mut() {
        pro_o.project_pos = Vec3::new(
            pro_t.translation.x + pro_o.velocity.x,
            pro_t.translation.y + pro_o.velocity.y,
            0.,
        );
        for (mut e_o, entity) in actives.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                pro_o.project_pos,
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                e_o.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                if matches!(pro_o.proj_type, ProjType::Particle) {
                    // let mut p = player.single_mut();
                    e_o.health -= 5;
                    // print!("{}\n", e_o.health);
                    commands.entity(entity_p).despawn();
                    match coll_type {
                        Collision::Top => {
                            pro_o.velocity.y *= -1.;
                            pro_o.velocity.x *= 0.8;
                        }
                        Collision::Bottom => {
                            pro_o.velocity.y *= 1.;
                            pro_o.velocity.x *= 0.8;
                        }
                        Collision::Left => {
                            pro_o.velocity.x *= -1.;
                            pro_o.velocity.y *= 0.8;
                        }
                        Collision::Right => {
                            pro_o.velocity.x *= 1.;
                            pro_o.velocity.y *= 0.8;
                        }
                        Collision::Inside => {
                            pro_o.velocity.x *= 1.;
                        }
                    }
                } else if matches!(pro_o.proj_type, ProjType::Projectile) {
                    e_o.health -= PROJECTILE_DAMAGE;
                } else if matches!(pro_o.proj_type, ProjType::BrokenObj) {
                    if (pro_o.velocity.y <= -5.) {
                        e_o.health -= 20;
                    }
                }
            }
            let (mut p, po) = player.single_mut();
            let res2 = bevy::sprite::collide_aabb::collide(
                pro_o.project_pos,
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                po.projected_position,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );
            if res2.is_some() {
                // let coll_type: bevy::sprite::collide_aabb::Collision = res2.unwrap();
                if matches!(pro_o.proj_type, ProjType::Particle) {
                    if (pro_o.velocity.x * pro_o.velocity.y).round() as i8 > 30 {
                        p.health -= 30;
                    } else if (pro_o.velocity.x * pro_o.velocity.y).round() as i8 > 3 {
                        p.health -= (pro_o.velocity.x * pro_o.velocity.x).round() as i8;
                    }
                    commands.entity(entity_p).despawn();
                } else if matches!(pro_o.proj_type, ProjType::BrokenObj) {
                    // if (pro_o.velocity.y).round() as i8 > 10 {
                    if (pro_o.velocity.y <= -5.) {
                        p.health -= 5;
                    }
                    // } else if (pro_o.velocity.x * pro_o.velocity.y).round() as i8 > 3 {
                    //     p.health -= (pro_o.velocity.x * pro_o.velocity.x).round() as i8;
                    // }
                    commands.entity(entity_p).despawn();
                    print!("Ouch\n");
                }
            }
        }
    }
}

pub fn break_objects(
    mut commands: Commands,
    mut objects: Query<
        (&mut Object, &Transform, Entity),
        (With<Object>, Without<Player>, Without<Projectile>),
    >,
    mut projectiles: Query<
        (&mut Projectile, &mut Transform, Entity),
        (Without<Object>, Without<Player>, Without<Enemy>),
    >,
) {
    for (mut pro_o, mut pro_t, entity) in projectiles.iter_mut() {
        let mut collide = false;
        pro_o.velocity.y += GRAVITY;
        for (mut o_o, o_t, o_e) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                pro_o.project_pos,
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                o_t.translation,
                Vec2::new(o_o.width, o_o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                let mut time: f32 = 5.0;
                if matches!(pro_o.proj_type, ProjType::Projectile) {
                    commands.entity(entity).despawn();
                    if matches!(o_o.obj_type, ObjectType::Breakable) {
                        if o_o.broken {
                            commands.entity(o_e).despawn();

                            let mut rng = rand::thread_rng();
                            for i in 1..5 {
                                let mut rng = rand::thread_rng();
                                let mut p_xvel = 0.;
                                let mut p_yvel = 0.;
                                match coll_type {
                                    Collision::Left => {
                                        p_xvel = rng.gen_range(2, 7) as f32;
                                        p_yvel = (i as f32 - 3.) / 2.;
                                    }
                                    Collision::Right => {
                                        p_xvel = rng.gen_range(-7, -2) as f32;
                                        p_yvel = (i as f32 - 3.) / 2.;
                                    }
                                    Collision::Top => {
                                        p_yvel = rng.gen_range(-7, -2) as f32;
                                        p_xvel = (i as f32 - 3.) / 2.;
                                    }
                                    Collision::Bottom => {
                                        p_yvel = rng.gen_range(4, 10) as f32;
                                        p_xvel = (i as f32 - 3.) / 2.;
                                    }
                                    Collision::Inside => {
                                        let horizontal = if pro_o.velocity.x > pro_o.velocity.y {
                                            true
                                        } else {
                                            false
                                        };
                                        if (horizontal && pro_o.velocity.x > 0.) {
                                            p_xvel = rng.gen_range(2, 7) as f32;
                                            p_yvel = (i as f32 - 3.) / 2.;
                                        } else if horizontal && pro_o.velocity.x < 0. {
                                            p_xvel = rng.gen_range(-7, -2) as f32;
                                            p_yvel = (i as f32 - 3.) / 2.;
                                        } else if !horizontal && pro_o.velocity.y > 0. {
                                            p_yvel = rng.gen_range(2, 7) as f32;
                                            p_xvel = (i as f32 - 3.) / 2.;
                                        } else if !horizontal && pro_o.velocity.y < 0. {
                                            p_yvel = rng.gen_range(-7, -2) as f32;
                                            p_xvel = (i as f32 - 3.) / 2.;
                                        } else {
                                            p_yvel = rng.gen_range(2, 7) as f32;
                                            p_xvel = rng.gen_range(2, 7) as f32;
                                        }
                                    }
                                }
                                let sz = o_o.height / rng.gen_range(8, 16) as f32;
                                commands
                                    .spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::RED,
                                            custom_size: Some(Vec2::new(sz, sz)),
                                            ..default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(
                                                o_t.translation.x,
                                                o_t.translation.y,
                                                2.,
                                            ),
                                            ..default()
                                        },
                                        // texture: asset_server.load("bullet.png"),
                                        ..default()
                                    })
                                    .insert(Projectile::new(
                                        Vec2::new(p_xvel, p_yvel as f32),
                                        ProjType::BrokenObj,
                                    ))
                                    .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
                            }
                        }
                    } else if matches!(o_o.obj_type, ObjectType::Barrel) {
                        if (o_o.broken) {
                            commands.entity(o_e).despawn();

                            let mut rng = rand::thread_rng();
                            for i in 1..10 {
                                let mut rng = rand::thread_rng();
                                let mut p_xvel = 0.;
                                let mut p_yvel = 0.;
                                match coll_type {
                                    Collision::Left => {
                                        p_xvel = rng.gen_range(10, 20) as f32;
                                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Right => {
                                        p_xvel = rng.gen_range(-20, -10) as f32;
                                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Top => {
                                        p_yvel = rng.gen_range(-20, -10) as f32;
                                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Bottom => {
                                        p_yvel = rng.gen_range(10, 20) as f32;
                                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Inside => {
                                        let horizontal = if pro_o.velocity.x > pro_o.velocity.y {
                                            true
                                        } else {
                                            false
                                        };
                                        if (horizontal && pro_o.velocity.x > 0.) {
                                            p_xvel = rng.gen_range(10, 20) as f32;
                                            p_yvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if horizontal && pro_o.velocity.x < 0. {
                                            p_xvel = rng.gen_range(-20, -10) as f32;
                                            p_yvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if !horizontal && pro_o.velocity.y > 0. {
                                            p_yvel = rng.gen_range(10, 20) as f32;
                                            p_xvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if !horizontal && pro_o.velocity.y < 0. {
                                            p_yvel = rng.gen_range(-20, -10) as f32;
                                            p_xvel = (i as f32 - 3.) / 2. + 2.;
                                        } else {
                                            p_yvel = rng.gen_range(4, 10) as f32;
                                            p_xvel = rng.gen_range(-10, 10) as f32;
                                        }
                                    }
                                }
                                let sz = o_o.height / rng.gen_range(8, 16) as f32;
                                commands
                                    .spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::RED,
                                            custom_size: Some(Vec2::new(sz, sz)),
                                            ..default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(
                                                o_t.translation.x,
                                                o_t.translation.y,
                                                2.,
                                            ),
                                            ..default()
                                        },
                                        // texture: asset_server.load("bullet.png"),
                                        ..default()
                                    })
                                    .insert(Projectile::new(
                                        Vec2::new(p_xvel, p_yvel as f32),
                                        ProjType::Particle,
                                    ))
                                    .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
                            }
                        }
                    }
                } else if matches!(pro_o.proj_type, ProjType::Particle) {
                    if matches!(o_o.obj_type, ObjectType::Barrel) {
                        if o_o.broken {
                            commands.entity(o_e).despawn();

                            let mut rng = rand::thread_rng();
                            for i in 1..10 {
                                let mut rng = rand::thread_rng();
                                let mut p_xvel = 0.;
                                let mut p_yvel = 0.;
                                match coll_type {
                                    Collision::Left => {
                                        p_xvel = rng.gen_range(10, 20) as f32;
                                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Right => {
                                        p_xvel = rng.gen_range(-20, -10) as f32;
                                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Top => {
                                        p_yvel = rng.gen_range(-20, -10) as f32;
                                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Bottom => {
                                        p_yvel = rng.gen_range(10, 20) as f32;
                                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                                    }
                                    Collision::Inside => {
                                        let horizontal = if pro_o.velocity.x > pro_o.velocity.y {
                                            true
                                        } else {
                                            false
                                        };
                                        if (horizontal && pro_o.velocity.x > 0.) {
                                            p_xvel = rng.gen_range(10, 20) as f32;
                                            p_yvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if horizontal && pro_o.velocity.x < 0. {
                                            p_xvel = rng.gen_range(-20, -10) as f32;
                                            p_yvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if !horizontal && pro_o.velocity.y > 0. {
                                            p_yvel = rng.gen_range(10, 20) as f32;
                                            p_xvel = (i as f32 - 3.) / 2. + 2.;
                                        } else if !horizontal && pro_o.velocity.y < 0. {
                                            p_yvel = rng.gen_range(-20, -10) as f32;
                                            p_xvel = (i as f32 - 3.) / 2. + 2.;
                                        } else {
                                            p_yvel = rng.gen_range(4, 10) as f32;
                                            p_xvel = rng.gen_range(-10, 10) as f32;
                                        }
                                    }
                                }
                                let sz = o_o.height / rng.gen_range(8, 16) as f32;
                                commands
                                    .spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::RED,
                                            custom_size: Some(Vec2::new(sz, sz)),
                                            ..default()
                                        },
                                        transform: Transform {
                                            translation: Vec3::new(
                                                o_t.translation.x,
                                                o_t.translation.y,
                                                2.,
                                            ),
                                            ..default()
                                        },
                                        // texture: asset_server.load("bullet.png"),
                                        ..default()
                                    })
                                    .insert(Projectile::new(
                                        Vec2::new(p_xvel, p_yvel as f32),
                                        ProjType::Particle,
                                    ))
                                    .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn break_hb_objects(
    mut player: Query<(&mut Player, &Transform), With<Player>>,
    mut commands: Commands,
    mut objects: Query<
        (&mut Object, &Transform, Entity),
        (With<Object>, Without<Player>, Without<Projectile>),
    >,
) {
    for (mut o_o, o_t, o_e) in objects.iter_mut() {
        if o_o.broken {
            let (p, pt) = player.single_mut();
            let horizontal = if (o_t.translation.x - pt.translation.x).abs()
                > (o_t.translation.y - pt.translation.y).abs()
            {
                true
            } else {
                false
            };
            let mut rng = rand::thread_rng();
            let mut p_xvel = 0.;
            let mut p_yvel = 0.;
            commands.entity(o_e).despawn();

            if matches!(o_o.obj_type, ObjectType::Barrel) {
                for i in 1..10 {
                    if (horizontal && pt.translation.x < o_t.translation.x) {
                        p_xvel = rng.gen_range(10, 20) as f32;
                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                    } else if (horizontal && pt.translation.x > o_t.translation.x) {
                        p_xvel = rng.gen_range(-20, -10) as f32;
                        p_yvel = (i as f32 - 3.) / 2. + 2.;
                    } else if (!horizontal && pt.translation.y > o_t.translation.y) {
                        p_yvel = rng.gen_range(-20, -10) as f32;
                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                    } else if (!horizontal && pt.translation.y < o_t.translation.y) {
                        p_yvel = rng.gen_range(10, 20) as f32;
                        p_xvel = (i as f32 - 3.) / 2. + 2.;
                    }
                    let sz = o_o.height / rng.gen_range(8, 16) as f32;
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
                                custom_size: Some(Vec2::new(sz, sz)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(o_t.translation.x, o_t.translation.y, 2.),
                                ..default()
                            },
                            // texture: asset_server.load("bullet.png"),
                            ..default()
                        })
                        .insert(Projectile::new(
                            Vec2::new(p_xvel, p_yvel as f32),
                            ProjType::Particle,
                        ))
                        .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
                }
            } else if matches!(o_o.obj_type, ObjectType::Breakable) {
                for i in 1..5 {
                    let mut rng = rand::thread_rng();
                    let mut p_xvel = 0.;
                    let mut p_yvel = 0.;
                    if (horizontal && pt.translation.x < o_t.translation.x) {
                        p_xvel = rng.gen_range(2, 7) as f32;
                        p_yvel = (i as f32 - 3.) / 2.;
                    } else if (horizontal && pt.translation.x > o_t.translation.x) {
                        p_xvel = rng.gen_range(-7, -2) as f32;
                        p_yvel = (i as f32 - 3.) / 2.;
                    } else if (!horizontal && pt.translation.y > o_t.translation.y) {
                        p_yvel = rng.gen_range(-7, -2) as f32;
                        p_xvel = (i as f32 - 3.) / 2.;
                    } else if (!horizontal && pt.translation.y < o_t.translation.y) {
                        p_yvel = rng.gen_range(2, 7) as f32;
                        p_xvel = (i as f32 - 3.) / 2.;
                    }

                    let sz = o_o.height / rng.gen_range(8, 16) as f32;
                    commands
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
                                custom_size: Some(Vec2::new(sz, sz)),
                                ..default()
                            },
                            transform: Transform {
                                translation: Vec3::new(o_t.translation.x, o_t.translation.y, 2.),
                                ..default()
                            },
                            // texture: asset_server.load("bullet.png"),
                            ..default()
                        })
                        .insert(Projectile::new(
                            Vec2::new(p_xvel, p_yvel as f32),
                            ProjType::BrokenObj,
                        ))
                        .insert(BrokenObj::new(Timer::from_seconds(4.0, false)));
                }
            }
        }
    }
}
