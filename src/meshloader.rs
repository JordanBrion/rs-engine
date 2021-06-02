extern crate obj;

use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

pub type MyVec3 = [f32; 3];

pub struct Mesh {
    pub vertices: Vec<MyVec3>,
}

pub fn read_mesh(fullpath: &str) -> Mesh {
    let input = BufReader::new(File::open(fullpath).unwrap());
    let model: Obj = load_obj(input).unwrap();
    Mesh {
        vertices: model.vertices.into_iter().map(|f| f.position).collect(),
    }
}
