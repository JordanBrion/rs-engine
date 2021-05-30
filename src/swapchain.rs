use crate::renderingcontext::*;
use crate::surface::*;
use crate::window::*;

pub struct MySwapchain {
    pub loader: ash::extensions::khr::Swapchain,
    pub inner: ash::vk::SwapchainKHR,
    pub v_images: Vec<ash::vk::Image>,
}

impl MySwapchain {
    pub fn new(context: &MyRenderingContext, window: &MyWindow, surface: &MySurface) -> MySwapchain {
        unsafe {
            let swapchain_loader =
                ash::extensions::khr::Swapchain::new(&context.instance, &context.logical_device);
            let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR {
                s_type: ash::vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
                p_next: std::ptr::null(),
                flags: Default::default(), //ash::vk::SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS,
                surface: surface.inner,
                min_image_count: surface.image_count as u32,
                image_format: surface.format.format,
                image_color_space: surface.format.color_space,
                image_extent: window.dimensions,
                image_array_layers: 1,
                image_usage: ash::vk::ImageUsageFlags::COLOR_ATTACHMENT,
                image_sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: std::ptr::null(),
                pre_transform: surface.capabilities.current_transform,
                composite_alpha: ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
                present_mode: Self::choose_swapchain_present_mode(&surface.v_present_modes),
                clipped: ash::vk::TRUE,
                old_swapchain: ash::vk::SwapchainKHR::null(),
            };

            let swapchain = swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Cannot create swapchain");
            let v_swapchain_images = swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Cannot get swapchain images");
            let swapchain_size = v_swapchain_images.len();
            MySwapchain {
                loader: swapchain_loader,
                inner: swapchain,
                v_images: v_swapchain_images,
            }
        }
    }

    pub fn size(&self) -> usize {
        self.v_images.len()
    }

    fn choose_swapchain_present_mode(
        v_present_modes: &Vec<ash::vk::PresentModeKHR>,
    ) -> ash::vk::PresentModeKHR {
        return match v_present_modes.iter().find(|mode| {
            return **mode == ash::vk::PresentModeKHR::MAILBOX;
        }) {
            Some(mode) => *mode,
            None => ash::vk::PresentModeKHR::FIFO,
        };
    }
}
