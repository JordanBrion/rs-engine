extern crate ash;
extern crate core;
extern crate nalgebra_glm as glm;
extern crate num;
extern crate sdl2;

use lowlevelrenderer::MyLowLevelRendererBuilder;
use mvp::MyMvp;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use std::ffi::CString;

use core::convert::Into;

use crate::vertexbuffer::MyVertexBuffer;
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::Handle;

mod camera;
mod devicememory;
mod frame;
mod gameentity;
mod id;
mod lowlevelrenderer;
mod meshloader;
mod mvp;
mod renderingcontext;
mod stagingbuffer;
mod surface;
mod swapchain;
mod uniformbuffer;
mod vertexbuffer;
mod window;

use camera::MyCamera;
use gameentity::MyGameEntity;
use meshloader::*;

fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => return true,
        }
    }
    true
}

fn main() {
    let cube = read_mesh("resources/mesh/cube.obj");
    let camera = MyCamera::new();
    let mut entity_1 = MyGameEntity::new(&cube);
    let mut entity_2 = MyGameEntity::new(&cube);
    let mut renderer = MyLowLevelRendererBuilder::new()
        .mesh(&cube)
        .uniform_buffer(&entity_1.id, std::mem::size_of_val(&entity_1.orientation))
        .uniform_buffer(&entity_2.id, std::mem::size_of_val(&entity_2.orientation))
        .build();

    let mut event_pump = renderer
        .window
        .sdl_context
        .event_pump()
        .expect("Cannot get sdl event pump");
    let mut go = true;

    let mut matrices = MyMvp {
        m_model: glm::identity(),
        m_view: glm::look_at(
            &glm::vec3(0.0, 0.0, 4.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        ),
        m_projection: glm::perspective(16.0f32 / 9.0f32, 45.0f32, 1.0f32, 100.0f32),
    };

    let mut model_1: glm::Mat4 = glm::identity();
    model_1 = glm::scale(&model_1, &glm::vec3(1.0, 0.5, 1.0));
    let mut model_2: glm::Mat4 = glm::identity();
    model_2 = glm::scale(&model_2, &glm::vec3(1.0, 1.0, 1.0));
    let translation_1: glm::Mat4 = glm::translate(&glm::identity(), &glm::vec3(2.0, 0.0, 0.0));
    let translation_2: glm::Mat4 = glm::translate(&glm::identity(), &glm::vec3(-2.0, 0.0, 0.0));

    while go {
        model_1 = glm::rotate(&model_1, 0.01, &glm::vec3(0.0, 1.0, 0.0));
        model_2 = glm::rotate(&model_2, -0.01, &glm::vec3(0.0, 1.0, 0.0));
        entity_1.orientation = matrices.m_projection * matrices.m_view * translation_1 * model_1;
        entity_2.orientation = matrices.m_projection * matrices.m_view * translation_2 * model_2;
        renderer.acquire_image();
        renderer.update(&entity_1);
        renderer.update(&entity_2);
        renderer.run();
        go = handle_events(&mut event_pump);
    }
}
