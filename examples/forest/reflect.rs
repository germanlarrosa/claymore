use std::collections::HashMap;

#[derive(RustcDecodable)]
pub struct Demo {
    pub name: String,
    pub generate: bool,
    pub control: Control,
    pub debug: Debug,
    pub palette: Palette,
}

#[derive(RustcDecodable)]
pub struct Control {
    pub move_speed: f32,
    pub rotate_speed: f32,
}

#[derive(RustcDecodable)]
pub struct Debug {
    pub offset: (i32, i32),
    pub line_jump: i32,
    pub color: (f32, f32, f32, f32),
    pub time_factor: u64,
}

#[derive(RustcDecodable)]
pub struct Palette {
    pub scene: String,
    pub size: f32,
    pub model: Model,
    pub tiles: HashMap<String, String>,
    pub water_plants: Vec<String>,
    pub plants: Vec<String>,
    pub tents: Vec<String>,
    pub camp_fires: Vec<String>,
}

#[derive(RustcDecodable)]
pub struct Model {
    pub grid_size: (i32, i32),
    pub water_plant_chance: f32,
    pub plant_chance: f32,
    pub max_grass_plants: u8,
    pub max_river_plants: u8,
    pub tent_chance: f32,
    pub water_height: f32,
    pub ground_height: f32,
}
