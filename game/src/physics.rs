use crate::active_util::*;
use crate::ai::*;
use crate::util::*;
use bevy::asset;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use rand::Rng;

const PROJECTILE_SZ: f32 = 6.;
const PROJECTILE_DAMAGE: i32 = 10;

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
    mut player: Query<(&mut ActiveObject, &mut Transform), (With<Player>)>,
    asset_server: Res<AssetServer>,
) {
    let (pl, pt) = player.single_mut();

    let mut vel = Vec2::new(10., 5.);

    if pl.facing_left {
        vel.x *= -1.;
    }
    if input.pressed(KeyCode::W) {
        vel.x = 0.;
        vel.y = 10.;
    } else if input.pressed(KeyCode::S) {
        vel.x = 0.;
        vel.y = -10.;
    }
    vel += pl.velocity;

    if input.just_pressed(KeyCode::L) {
        // for (pla)
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

pub fn projectile_static_collisions(
    mut commands: Commands,
    mut objects: Query<
        (&Object, &Transform, Entity),
        (With<Object>, Without<Player>, Without<Projectile>),
    >,
    mut projectiles: Query<
        (&mut Projectile, &mut Transform, Entity),
        (Without<Object>, Without<Player>, Without<Enemy>),
    >,
) {
    //let (pl, pt) = player.single_mut();
    for (mut pro_o, mut pro_t, entity) in projectiles.iter_mut() {
        pro_o.project_pos = Vec3::new(
            pro_t.translation.x + pro_o.velocity.x,
            pro_t.translation.y + pro_o.velocity.y,
            0.,
        );
        let mut collide = false;
        // pro_t.translation.x += pro_o.velocity.x;
        // pro_t.translation.y += pro_o.velocity.y;
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
                        println!("{:?}", coll_type);
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
                                    p_yvel = rng.gen_range(7, 2) as f32;
                                    p_xvel = (i as f32 - 3.) / 2.;
                                }
                                Collision::Inside => {
                                    p_yvel = rng.gen_range(2, 7) as f32;
                                    p_xvel = rng.gen_range(2, 7) as f32;
                                }
                            }
                            let sz = o_o.height / rng.gen_range(8, 16) as f32;
                            commands
                                .spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::BLACK,
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
        (Without<Object>, Without<Player>, Without<Enemy>),
    >,
    mut enemies: Query<(&mut ActiveObject, &mut Transform), (With<ActiveObject>, With<Enemy>)>,
) {
    for (mut pro_o, mut pro_t, entity) in projectiles.iter_mut() {
        for (mut e_o, e_t) in enemies.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                Vec3::new(pro_t.translation.x, pro_t.translation.y, 5.),
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                e_t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );

            if res.is_some() {
                print!("coll");
                e_o.health -= PROJECTILE_DAMAGE;
            }
        }
    }
}
