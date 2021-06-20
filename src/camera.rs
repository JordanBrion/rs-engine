use glm::*;

pub struct MyCamera {
    view: glm::Mat4,
    projection: glm::Mat4,
}

impl MyCamera {
    pub fn new() -> MyCamera {
        MyCamera {
            view: glm::look_at(
                &glm::vec3(0.0, 0.0, 4.0),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            ),
            projection: glm::perspective(16.0f32 / 9.0f32, 45.0f32, 1.0f32, 100.0f32),
        }
    }
}
