struct MyGameEntityId {
    id: usize,
}
pub struct MyGameEntity {
    id: MyGameEntityId,
    orientation: glm::Mat4,
}