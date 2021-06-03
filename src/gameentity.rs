use crate::gameentityid::MyGameEntityId;
use crate::meshloader::Mesh;

pub struct MyGameEntity<'a> {
    id: MyGameEntityId,
    mesh: &'a Mesh,
    orientation: glm::Mat4,
}

impl<'a> MyGameEntity<'a> {
    pub fn new(mesh: &'a Mesh) -> MyGameEntity {
        MyGameEntity {
            id: MyGameEntityId::new(),
            mesh: mesh,
            orientation: glm::identity(),
        }
    }
}
