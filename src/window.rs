use crate::renderingcontext::*;
use sdl2::*;

pub struct MyWindow {
    pub dimensions: ash::vk::Extent2D,
    pub sdl_context: sdl2::Sdl,
    pub inner: sdl2::video::Window,
}

impl MyWindow {
    pub fn new() -> Result<MyWindow, &'static str> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let dimensions = ash::vk::Extent2D {
            width: 1280,
            height: 720,
        };
        let window = sdl_context
            .video()
            .unwrap()
            .window("rust-sdl2 demo", dimensions.width, dimensions.height)
            .vulkan()
            .position_centered()
            .build()
            .unwrap();
        Ok(MyWindow {
            dimensions: dimensions,
            sdl_context: sdl_context,
            inner: window,
        })
    }
}
