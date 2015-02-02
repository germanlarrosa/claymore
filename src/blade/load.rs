use rustc_serialize::json;
use std::collections::HashMap;
use std::old_io as io;

pub type Scalar = f32;

#[derive(RustcDecodable)]
pub struct Scene {
    pub global: Global,
    pub nodes: Vec<Node>,
    pub materials: Vec<Material>,
    pub entities: Vec<Entity>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
}

#[derive(RustcDecodable)]
pub struct Global {
    pub gravity: (f32, f32, f32),
}

#[derive(RustcDecodable)]
pub struct Node {
    pub name: String,
    pub space: Space<Scalar>,
    pub children: Vec<Node>,
    pub actions: Vec<Action>,
}

#[derive(RustcDecodable)]
pub struct Space<S> {
    pub pos: (S, S, S),
    pub rot: (S, S, S, S),
    pub scale: S,
}

#[derive(RustcDecodable)]
pub struct Entity {
    pub mesh: String,
    pub range: (u32, u32),
    pub armature: String,
    pub material: String,
    pub actions: Vec<Action>,
}

#[derive(RustcDecodable)]
pub struct Light {
    pub name: String,
    pub kind: String,
    pub color: (f32, f32, f32),
    pub energy: f32,
    pub distance: f32,
    pub attenuation: (f32, f32),
    pub spherical: bool,
    pub parameters: Vec<f32>,
    pub actions: Vec<Action>,
}

#[derive(RustcDecodable)]
pub struct Camera {
    pub name: String,
    pub angle: (f32, f32),
    pub range: (f32, f32),
    pub actions: Vec<Action>,
}

#[derive(RustcDecodable)]
pub struct Material {
    pub name: String,
    pub shader: String,
    pub data: HashMap<String, Data>,
    pub textures: Vec<Texture>,
}

pub type Data = (String, Vec<f32>);
pub type Texture = ();  //TODO
pub type Action = ();   //TODO

#[derive(Debug)]
pub enum Error {
    Read(io::IoError),
    Decode(json::DecoderError),
}

pub fn json(path: &str) -> Result<Scene, Error> {
    match io::File::open(&Path::new(path)).read_to_string() {
        Ok(data) => json::decode(data.as_slice()).map_err(|e|
            Error::Decode(e)
        ),
        Err(e) => Err(Error::Read(e)),
    }
}
