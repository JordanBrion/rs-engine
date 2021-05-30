use crate::{renderingcontext::*, window::MyWindow};
use ash::version::InstanceV1_0;
use ash::vk::Handle;

pub struct MySurface {
    pub inner: ash::vk::SurfaceKHR,
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub format: ash::vk::SurfaceFormatKHR,
    pub image_count: usize,
    pub v_present_modes: Vec<ash::vk::PresentModeKHR>,
}

impl MySurface {
    pub fn new(context: &MyRenderingContext, window: &mut MyWindow) -> Result<MySurface, &'static str> {
        unsafe {
            let surface_loader =
                ash::extensions::khr::Surface::new(&context.entry, &context.instance);
            let surface_handle = window
                .inner
                .vulkan_create_surface(context.instance.handle().as_raw() as usize)
                .expect("Cannot create surface");
            let surface = ash::vk::SurfaceKHR::from_raw(surface_handle);
            let presentation_supported = surface_loader.get_physical_device_surface_support(
                context.gpu,
                context.index_of_queue_family as u32,
                surface,
            );
            if !presentation_supported {
                return Err("Presentation not supported !");
            }
            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(context.gpu, surface)
                .expect("Cannot get surface capabilities");
            let v_surface_formats = surface_loader
                .get_physical_device_surface_formats(context.gpu, surface)
                .expect("Cannot get physical device surface formats");
            let v_surface_present_modes = surface_loader
                .get_physical_device_surface_present_modes(context.gpu, surface)
                .expect("Cannot get surface present mode");
            let available_format =
                Self::search_format(&v_surface_formats).expect("Cannot find surface format");
            let image_count = if surface_capabilities.max_image_count > 0
                && surface_capabilities.min_image_count + 1 > surface_capabilities.max_image_count
            {
                surface_capabilities.max_image_count
            } else {
                surface_capabilities.min_image_count + 1
            };

            window.dimensions = if surface_capabilities.current_extent.width != !(0 as u32) {
                surface_capabilities.current_extent
            } else {
                ash::vk::Extent2D {
                    width: num::clamp(
                        window.dimensions.width as u32,
                        surface_capabilities.min_image_extent.width,
                        surface_capabilities.max_image_extent.width,
                    ),
                    height: num::clamp(
                        window.dimensions.height as u32,
                        surface_capabilities.min_image_extent.height,
                        surface_capabilities.max_image_extent.height,
                    ),
                }
            };
            Ok(MySurface {
                inner: surface,
                capabilities: surface_capabilities,
                format: available_format,
                image_count: image_count as usize,
                v_present_modes: v_surface_present_modes
            })
        }
    }

    fn search_format(
        v_surface_formats: &Vec<ash::vk::SurfaceFormatKHR>,
    ) -> Result<ash::vk::SurfaceFormatKHR, &'static str> {
        for format in v_surface_formats {
            if format.format == ash::vk::Format::B8G8R8A8_UNORM
                && format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return Ok(*format);
            }
        }
        Err("Cannot find surface format")
    }
}
