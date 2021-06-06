use crate::id::MyId;
use crate::meshloader::Mesh;

pub struct MyGameEntity<'a> {
    pub id: MyId,
    mesh: &'a Mesh,
    pub orientation: glm::Mat4,
}

impl<'a> MyGameEntity<'a> {
    pub fn new(mesh: &'a Mesh) -> MyGameEntity {
        MyGameEntity {
            id: MyId::new(),
            mesh: mesh,
            orientation: glm::identity(),
        }
    }
}
