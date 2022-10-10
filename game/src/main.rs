use bevy::{
	prelude::*,
	window::PresentMode,
};
use std::convert::From;

const TITLE: &str = "Team 1 Game";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const PLAYER_SZ: f32 = 32.;

const TILE_SIZE: f32 = 100.;

const SCROLL_SPEED: f32 = 120.;

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component)]
struct Velocity {
	velocity: Vec2,
}

impl Velocity {
	fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

#[derive(Component)]
struct Player;

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
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let images= &["jacob.png","bailey.png","brian.png","ethan.png","jack.png","gio.png", "zach.png"];
	commands.spawn_bundle(Camera2dBundle::default());
	let mut time: f32=0.0;
	for image in images {
		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load(*image),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(time, false)));
		time+=5.0;
	}

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::BLUE,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
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
}

fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
	let mut count = 1.0;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.z = count;
		}
		count+=1.0;
	}
}

fn move_player(
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
){
	let (mut pt, mut pv) = player.single_mut();

	let mut deltav = Vec2::splat(0.);

	if input.pressed(KeyCode::A) {
		deltav.x -= 1.;
	}

	if input.pressed(KeyCode::D) {
		deltav.x += 1.;
	}

	if input.pressed(KeyCode::W) {
		deltav.y += 1.;
	}

	if input.pressed(KeyCode::S) {
		deltav.y -= 1.;
	}

	let deltat = time.delta_seconds(); 
	let acc = ACCEL_RATE * deltat;

	pv.velocity = if deltav.length() > 0. {
		(pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
	}
	else if pv.velocity.length() > acc {
		pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
	}
	else {
		Vec2::splat(0.)
	};
	let change = pv.velocity * deltat;

	let new_pos = pt.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);
	if new_pos.x >= -(WIN_W/2.) + TILE_SIZE/2.
		&& new_pos.x <= WIN_W/2. - TILE_SIZE/2.
	{
		pt.translation = new_pos;
	}

	let new_pos = pt.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
	if new_pos.y >= -(WIN_H/2.) + TILE_SIZE/2.
		&& new_pos.y <= WIN_H/2. - TILE_SIZE/2.
	{
		pt.translation = new_pos;
	}
}
