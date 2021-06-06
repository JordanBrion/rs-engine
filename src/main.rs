extern crate ash;
extern crate core;
extern crate nalgebra_glm as glm;
extern crate num;
extern crate sdl2;

use lowlevelrenderer::MyLowLevelRendererBuilder;
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
    let entity = MyGameEntity::new(&cube);
    let mut renderer = MyLowLevelRendererBuilder::new()
        .mesh(&cube)
        .uniform_buffer(&entity.orientation)
        .build();

    let mut event_pump = renderer
        .window
        .sdl_context
        .event_pump()
        .expect("Cannot get sdl event pump");
    let mut go = true;

    while go {
        renderer.run();
        go = handle_events(&mut event_pump);
    }
}
