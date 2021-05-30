use crate::renderingcontext::*;
use sdl2::*;

pub struct MyWindow {
    pub dimensions: ash::vk::Extent2D,
    pub inner: sdl2::video::Window,
}

impl MyWindow {
    pub fn new(context: &MyRenderingContext) -> Result<MyWindow, &'static str> {
        let dimensions = ash::vk::Extent2D { 
            width: 1280,
            height: 720,
        };
        let window = context
            .sdl_context
            .video()
            .unwrap()
            .window("rust-sdl2 demo", dimensions.width, dimensions.height)
            .vulkan()
            .position_centered()
            .build()
            .unwrap();
        Ok(MyWindow {
            dimensions: dimensions,
            inner: window,
        })
    }
}
