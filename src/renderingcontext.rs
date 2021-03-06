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
    pub instance: ash::Instance,
    pub entry: ash::Entry,
    pub gpu: ash::vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub index_of_queue_family: usize,
    pub queue: ash::vk::Queue,
    pub command_pool: ash::vk::CommandPool,
}

impl MyRenderingContext {
    pub fn new(window: &MyWindow) -> MyRenderingContext {
        unsafe {
            let entry = ash::Entry::new().expect("Cannot create entry");
            let instance = Self::create_instance(
                &entry,
                window
                    .inner
                    .vulkan_instance_extensions()
                    .expect("Cannot get instance extensions!"),
            );
            let gpu = Self::pick_up_one_gpu(&instance).expect("Cannot find GPU");
            let index_of_queue_family = Self::lookup_queue_family_index(&instance, &gpu)
                .expect("Cannot find graphics queue family");
            let logical_device =
                Self::create_logical_device(&instance, &gpu, index_of_queue_family)
                    .expect("Cannot create logical device");
            let queue = logical_device.get_device_queue(index_of_queue_family as u32, 0);
            let command_pool_create_info = ash::vk::CommandPoolCreateInfo {
                s_type: ash::vk::StructureType::COMMAND_POOL_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                queue_family_index: index_of_queue_family as u32,
            };

            let command_pool = logical_device
                .create_command_pool(&command_pool_create_info, None)
                .expect("Cannot create command pool");

            MyRenderingContext {
                entry: entry,
                instance: instance,
                gpu: gpu,
                logical_device: logical_device,
                index_of_queue_family: index_of_queue_family,
                queue: queue,
                command_pool: command_pool,
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

    unsafe fn lookup_queue_family_index(
        instance: &ash::Instance,
        gpu: &ash::vk::PhysicalDevice,
    ) -> Result<usize, &'static str> {
        let queue_family_properties = instance.get_physical_device_queue_family_properties(*gpu);
        for i in 0..queue_family_properties.len() {
            if queue_family_properties[i]
                .queue_flags
                .contains(ash::vk::QueueFlags::GRAPHICS)
            {
                return Ok(i);
            }
        }
        Err("Queue family not found")
    }

    unsafe fn create_logical_device(
        instance: &ash::Instance,
        gpu: &ash::vk::PhysicalDevice,
        index_of_queue_family: usize,
    ) -> Result<ash::Device, ash::vk::Result> {
        let priority = 1.0_f32;
        let queue_create_info = ash::vk::DeviceQueueCreateInfo {
            s_type: ash::vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: index_of_queue_family as u32,
            queue_count: 1,
            p_queue_priorities: &priority,
        };

        let mut v_extensions = Vec::new();
        v_extensions.push(ash::extensions::khr::Swapchain::name());
        let v_extensions_c: Vec<*const i8> = v_extensions
            .into_iter()
            .map(|e| e.as_ptr() as *const i8)
            .collect();
        let device_create_info = ash::vk::DeviceCreateInfo {
            s_type: ash::vk::StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: v_extensions_c.len() as u32,
            pp_enabled_extension_names: v_extensions_c.as_ptr() as *const *const i8,
            p_enabled_features: std::ptr::null(),
        };
        instance.create_device(*gpu, &device_create_info, None)
    }
}
