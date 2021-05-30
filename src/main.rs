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

use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk::Handle;
use crate::vertexbuffer::MyPointData;
use crate::vertexbuffer::MyVertexBuffer;

mod renderingcontext;
mod window;
mod lowlevelrenderer;
mod surface;
mod swapchain;
mod vertexbuffer;
mod frame;
mod mvp;
mod uniformbuffer;
mod devicememory;

fn main() {
    let renderer = MyLowLevelRendererBuilder::new().build();
    renderer.run();
}
