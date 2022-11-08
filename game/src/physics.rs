use crate::active_util::*;
use crate::ai::*;
use crate::util::*;
use bevy::asset;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use rand::Rng;

const PROJECTILE_SZ: f32 = 5.;
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
}

impl Projectile {
    pub fn new(vel: Vec2, pt: ProjType) -> Self {
        Self {
            velocity: vel,
            proj_type: pt,
        }
    }

    // pub fn destroy(mut commands: Commands) {
    //     commands.entity(S).despawn();
    // }
}

#[derive(Component)]
pub struct BrokenObj {
    lifespan: Timer,
}

impl BrokenObj {
    pub fn new(lifespan: Timer) -> Self {
        Self {
            lifespan: lifespan,
        }
    }
}

pub fn shoot(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut player: Query<(&mut ActiveObject, &mut Transform), (With<Player>)>,
    asset_server: Res<AssetServer>,
) {
    let (pl, pt) = player.single_mut();

    let mut xvel = 20.;
    let mut yvel = 5.;
    if pl.facing_left {
        xvel *= -1.;
    }
    if input.pressed(KeyCode::W) {
        xvel = 0.;
        yvel = 15.;
    }
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
            .insert(Projectile::new(Vec2::new(xvel, yvel), ProjType::Projectile));
        // let id = x.id();
    }
}

pub fn projectile_collisions(
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
        pro_t.translation.x += pro_o.velocity.x;
        pro_t.translation.y += pro_o.velocity.y;
        pro_o.velocity.y += GRAVITY;
        for (mut o_o, o_t, o_e) in objects.iter_mut() {
            let res = bevy::sprite::collide_aabb::collide(
                Vec3::new(pro_t.translation.x, pro_t.translation.y, 0.),
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                o_t.translation,
                Vec2::new(o_o.width, o_o.height),
            );

            if res.is_some() {
                let mut time: f32 = 10.0;
                // let entity_o = o_e.id(); // id of object
                if matches!(o_o.obj_type, ObjectType::Breakable) {
                    commands.entity(o_e).despawn();
                    for i in 1..10 {
                        let mut rng = rand::thread_rng();
                        let p_xvel = (i as f32 - 5.) / 2.;
                        let p_yvel = rng.gen_range(3, 6);
                        commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    custom_size: Some(Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ)),
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
                            .insert(BrokenObj::new(Timer::from_seconds(time, false)));
                    }
                }
                if matches!(pro_o.proj_type, ProjType::Projectile) {
                    commands.entity(entity).despawn();
                } else if matches!(pro_o.proj_type, ProjType::BrokenObj) {
                    let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();

                    match coll_type {
                        Collision::Left => {
                            pro_o.velocity.x = 0.;
                        }
                        Collision::Right => {
                            pro_o.velocity.x = 0.;
                        }
                        Collision::Top => {
                            pro_o.velocity.y = 0.;
                        }
                        Collision::Bottom => {
                            pro_o.velocity.y = 0.;
                        }
                        Collision::Inside => {
                            pro_o.velocity.x = 0.;
                            pro_o.velocity.y = 0.;
                        }
                    }
                }
            }
        }
    }
}

pub fn despawn_broken_objects(
    time: Res<Time>,
    mut commands: Commands,
    mut brokenObjects: Query<(&mut BrokenObj, Entity),Without<Object>>,
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
                Vec3::new(pro_t.translation.x, pro_t.translation.y, 0.),
                Vec2::new(PROJECTILE_SZ, PROJECTILE_SZ),
                e_t.translation,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
            );

            if res.is_some() {
                e_o.health -= PROJECTILE_DAMAGE;
                println!("{}", e_o.health);
            }
        }
    }
}
