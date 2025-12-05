use std::fs;
use bevy::{color, prelude::*, sprite_render::Material2d, window::PrimaryWindow};

#[derive(Resource)]
pub struct Map {
    // 1D representation of the map
    pub diagram: Vec<char>,
    pub size: u8,
}

#[derive(Component)]
struct Cell {
    x: u8,
    y: u8,
}

fn setup(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let contents = fs::read_to_string("./src/day_4/input.txt").expect("Could not read file");
    let lines: Vec<&str> = contents.lines().collect();
    let map = Map::from(lines.clone());
    let map2 = Map::from(lines);
    commands.insert_resource(map2);
    let cell_size = 10.0;
    for y in 0..map.size {
        for x in 0..map.size {
            let color = match map.get(x, y).unwrap() {
                '@' => color::palettes::basic::BLACK,
                'x' => color::palettes::basic::GREEN,
                '.' => color::palettes::basic::GRAY,
                _ => color::palettes::basic::GRAY,
            };
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())), 
                MeshMaterial2d(materials.add(Color::from(color))), // Attach as a component
                Transform::from_xyz(x as f32 * cell_size, y as f32 * cell_size, 0.0).with_scale(Vec3::splat(cell_size)),
                GlobalTransform::default(),
                Cell { x, y },
            ));
        }
    }
    let desired_view_size = map.size as f32 * cell_size;
    let window = windows.single().unwrap();
    let width = window.width();
    let height = window.height();
    let window_size = width.min(height); // or get from Bevy's window resource
    let scale = desired_view_size / window_size; // >1.0 zooms out
    let center_x = (map.size as f32 * cell_size) / 2.0;
    let center_y = (map.size as f32 * cell_size) / 2.0;

    commands.spawn((
        Camera2d::default(), 
         Transform {
            translation: Vec3::new(center_x, center_y, 0.0),
            scale: Vec3::splat(scale),
            ..Default::default()
    },
       ));


}

fn display(
    mut map: ResMut<Map>,
    mut cell_query: Query<(&Cell, &MeshMaterial2d<ColorMaterial>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut positions_to_mark: Vec<(u8, u8)> = Vec::new();
        for y in 0..map.size {
            for x in 0..map.size {
                if map.is_valid_position(x, y, 4) {
                    map.set(x, y, 'x');
                    positions_to_mark.push((x, y));
                }
            }
        }
    for (cell, material) in cell_query.iter_mut() {
         let color = match map.get(cell.x, cell.y).unwrap() {
             '@' => Srgba::rgb(0.933, 0.733, 0.764),
                'x' => Srgba::rgb(0.965, 0.788, 0.055),
                _ => Srgba::rgb (0.137, 0.161, 0.274)
        };
        let mut mat = materials.get_mut(&material.0).unwrap();
        mat.color = Color::from(color)
    }

        

        for pos in positions_to_mark {
            map.set(pos.0, pos.1, '.');
        }
    
}

pub fn visualize() {
     App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, display)
        .run();
}