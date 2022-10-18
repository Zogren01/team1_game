//imports from outside crates
use bevy::sprite::collide_aabb::Collision;
use bevy::{
	prelude::*,
	window::PresentMode,
};
use std::convert::From;
use std::collections::HashSet;

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
		.add_system(camera_follow)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
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
				translation: Vec3::new(WIN_W/3., 0., 900.),
				..default()
			},
			..default()
		})
		.insert(Velocity::new())
		.insert(Player::new());
		
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::BLACK,
				custom_size: Some(Vec2::new(32., 200.)),
				..default()
			},
			transform: Transform {
				translation: Vec3::new(50., -150., 1.),
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

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::BLACK,
				custom_size: Some(Vec2::new(WIN_W, TILE_SIZE)),
				..default()
			},
			transform: Transform {
				translation: Vec3::new(0., -WIN_H/2. + TILE_SIZE, 1.),
				..default()
			},
			..default()
		})
		.insert(Rect::new(WIN_W, TILE_SIZE))
		.insert(Object::new(3));
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

fn calculate_sight(
	time: Res<Time>,
	player: Query<&Transform, With<Player>>,
	objects: Query<(&Object, &Rect, &Transform), With<Object>>,
	input: Res<Input<KeyCode>>,
){
	//TODO: make a struct for all of the sight lines for a given object
		// hold a Vec containing lines
		// hold a reference to the object
		// loop through each of these when doing checks
	let origin = player.single();
	let x_pos = origin.translation.x;
	let y_pos = origin.translation.y;

	if input.pressed(KeyCode::Q){

		let sight_distance = 300.0;
		let mut sight_lines = Vec::new();
		let mut object_lines = Vec::new();

		for (o, r, t) in objects.iter(){
			//v1 and v2 hold the endpoints for line of sight
			let v1: Vec2;
			let v2: Vec2;
			//v3 is the third point for the two sides to be used for collision
			let v3: Vec2;
			
			if x_pos > t.translation.x {
				if y_pos >= t.translation.y {
					//top left point
					v1 = Vec2::new(t.translation.x - r.width/2., t.translation.y + r.height/2.);
					//bottom right point
					v2 = Vec2::new(t.translation.x + r.width/2., t.translation.y - r.height/2.);
					//top right point
					v3 = Vec2::new(t.translation.x + r.width/2., t.translation.y + r.height/2.);
				}
				else {
					//top right point
					v1 = Vec2::new(t.translation.x + r.width/2., t.translation.y + r.height/2.);
					//bottom left point
					v2 = Vec2::new(t.translation.x - r.width/2., t.translation.y - r.height/2.);
					//bottom right point
					v3 = Vec2::new(t.translation.x + r.width/2., t.translation.y - r.height/2.);
				}
				//MAYBE code for when y's are equal
			}
			else {
				if y_pos > t.translation.y {
					//top right point
					v1 = Vec2::new(t.translation.x + r.width/2., t.translation.y + r.height/2.);
					//bottom left point
					v2 = Vec2::new(t.translation.x - r.width/2., t.translation.y - r.height/2.);
					//top left point
					v3 = Vec2::new(t.translation.x - r.width/2., t.translation.y + r.height/2.);
				}
				else {
					//top left point
					v1 = Vec2::new(t.translation.x - r.width/2., t.translation.y + r.height/2.);
					//bottom right point
					v2 = Vec2::new(t.translation.x + r.width/2., t.translation.y - r.height/2.);
					//bottom left point
					v3 = Vec2::new(t.translation.x - r.width/2., t.translation.y - r.height/2.);
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
			if s1.length_squared() < sight_distance*sight_distance {
				sight_lines.push(s1);
				in_range = true;
			}
			if s2.length_squared() < sight_distance*sight_distance{
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
	for l in sight.iter(){
		let mut result = true;
		for o in obj.iter(){
			let intersect = lines_intersect(l, o);
			if l.obj_id == 2 && o.obj_id == 1{
				l.print_line();
				o.print_line();
			}
			if intersect && (o.obj_id != l.obj_id){
				result = false;
				break;
			}
		}
		if result{
			ids.insert(l.obj_id);
		}
	}
	for id in ids.iter(){
		println!("Object with id {} is visible", id);
	}
	
}

fn helper(i: Vec2, j: Vec2, k: Vec2) -> bool{
	(k.y - i.y) * (j.x - i.x) > (j.y - i.y) * (k.x - i.x)
}

fn lines_intersect(a: &Line, b: &Line) -> bool{
	(helper(a.start, b.start, b.end) != helper(a.end, b.start, b.end)) && 
	(helper(a.start, a.end, b.start) != helper(a.start, a.end, b.end))
}

fn move_player(time: Res<Time>,	input: Res<Input<KeyCode>>, mut player: Query<(&mut Player, &mut Transform, &mut Velocity), (With<Player>, Without<Object>)>, 	objects: Query<(&Object, &Rect, &Transform), (With<Object>,Without<Player>)>,) {

	let (mut pl, mut pt, mut pv) = player.single_mut();
	

	if input.pressed(KeyCode::A) {
		if pv.velocity.x > -PLAYER_SPEED{
			pv.velocity.x = pv.velocity.x - 20.;
		}	
	}
	else if pv.velocity.x < 0.{
		pv.velocity.x = pv.velocity.x + 20.;
	}

	if input.pressed(KeyCode::D) {
		if pv.velocity.x < PLAYER_SPEED{
			pv.velocity.x = pv.velocity.x + 20.;
		}
	}
	else if pv.velocity.x > 0.{
		pv.velocity.x = pv.velocity.x - 20.;
	}
    
    if pv.velocity.y > TERMINAL_VELOCITY{
        pv.velocity.y += GRAVITY;
    }

    if input.pressed(KeyCode::Space) && pl.grounded {
        pv.velocity.y = PLAYER_SPEED * 1.5;
    }

    pl.grounded = false;
	let deltat = time.delta_seconds(); 

	let change = pv.velocity * deltat;

	let mut new_pos = pt.translation + Vec3::new(
		change.x,
		change.y,
		0.,
	);
    //this variable will track where the player will end up if there is no collision with a surface
    let y_goal = new_pos.y;

	for (_o,r,t) in objects.iter() {
		let res = bevy::sprite::collide_aabb::collide(
			new_pos,
			Vec2::new(PLAYER_SZ, PLAYER_SZ),
			t.translation,
			Vec2::new(r.width,r.height)
		);
		if res.is_some()
		{
			let coll_type :bevy::sprite::collide_aabb::Collision= res.unwrap();
			match coll_type {
				Collision::Left => {
					pv.velocity.x=0.;
					new_pos.x=t.translation.x-(r.width/2.)-PLAYER_SZ/2.;
				}
				Collision::Right => {
					pv.velocity.x=0.;
					new_pos.x=t.translation.x+(r.width/2.)+PLAYER_SZ/2.;
				}
				Collision::Top => {
					pv.velocity.y=0.;
                    new_pos.y=t.translation.y+(r.height/2.)+PLAYER_SZ/2.;
				}
				Collision::Bottom => {
					pv.velocity.y=0.;
					new_pos.y=t.translation.y-(r.height/2.)-PLAYER_SZ/2.;
				}
				Collision::Inside => {
                    println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
					pv.velocity = Vec2::new(0.,0.);
				}
			}
		}
    }
    //if the intended y is less than where the player ends up, no ground collision occured
    if y_goal < new_pos.y{
        pl.grounded = true;
    }
    pt.translation = new_pos;
	
}

fn camera_follow(
	player_query: Query<&Transform, With<Player>>,
	mut Camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
)
{
	let player = player_query.single();
	let mut camera = Camera_query.single_mut();

	camera.translation.x = player.translation.x;
	camera.translation.y = player.translation.y;
}


