use crate::renderingcontext::*;
use sdl2::*;

pub struct MyWindow {
    width: usize,
    height: usize,
    pub inner: sdl2::video::Window,
}

impl MyWindow {
    pub fn new(context: &MyRenderingContext) -> Result<MyWindow, &'static str> {
        let width = 1280;
        let height = 720;
        let window = context
            .sdl_context
            .video()
            .unwrap()
            .window("rust-sdl2 demo", width, height)
            .vulkan()
            .position_centered()
            .build()
            .unwrap();
        Ok(MyWindow {
            width: width as usize,
            height: height as usize,
            inner: window,
        })
    }
}
