use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);


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
		.add_system(show_popup)
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