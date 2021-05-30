extern crate ash;
extern crate core;
extern crate nalgebra_glm as glm;
extern crate num;
extern crate sdl2;

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

use crate::window::*;

pub struct MyRenderingContext {
    pub sdl_context: sdl2::Sdl,
    pub instance: ash::Instance,
    pub entry: ash::Entry,
    pub gpu: ash::vk::PhysicalDevice,
}

impl MyRenderingContext {
    pub fn new(window: &MyWindow) -> MyRenderingContext {
        unsafe {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
            let entry = ash::Entry::new().expect("Cannot create entry");
            let instance = Self::create_instance(
                &entry,
                window
                    .inner
                    .vulkan_instance_extensions()
                    .expect("Cannot get instance extensions!"),
            );
            let gpu = Self::pick_up_one_gpu(&instance).expect("Cannot find GPU");
            MyRenderingContext {
                sdl_context: sdl_context,
                entry: entry,
                instance: instance,
                gpu: gpu,
            }
        }
    }

    unsafe fn create_instance(entry: &ash::Entry, v_extensions: Vec<&str>) -> ash::Instance {
        let v_layers = vec![
            CString::new("VK_LAYER_KHRONOS_validation").expect("Cannot validation layer name")
        ];
        let application_name =
            CString::new("003_swapchain").expect("Cannot create application name");
        let engine_name = CString::new("Not Unreal Engine 4").expect("Cannot create engine name");
        let application_info = ash::vk::ApplicationInfo {
            s_type: ash::vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: application_name.as_ptr(),
            application_version: ash::vk_make_version!(1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: ash::vk_make_version!(0, 0, 1),
            api_version: ash::vk_make_version!(1, 0, 0),
        };
        let v_extensions_c: Vec<*const u8> = v_extensions.iter().map(|ss| ss.as_ptr()).collect();
        let instance_create_info = ash::vk::InstanceCreateInfo {
            s_type: ash::vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_application_info: &application_info,
            enabled_layer_count: v_layers.len() as u32,
            pp_enabled_layer_names: v_layers.as_ptr() as *const *const i8,
            enabled_extension_count: v_extensions_c.len() as u32,
            pp_enabled_extension_names: v_extensions_c.as_ptr() as *const *const i8,
        };
        entry
            .create_instance(&instance_create_info, None)
            .expect("Cannot create instance")
    }

    unsafe fn pick_up_one_gpu(instance: &ash::Instance) -> Option<ash::vk::PhysicalDevice> {
        match instance.enumerate_physical_devices() {
            Ok(ref gpus) if gpus.len() > 0 => Some(gpus[0]),
            Ok(_) => None,
            Err(_e) => None,
        }
    }
}
