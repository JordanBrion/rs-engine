use crate::devicememory::search_physical_device_memory_type;
use crate::renderingcontext::*;
use ash::version::DeviceV1_0;

pub struct MyUniformBuffer {
    pub id: ash::vk::Buffer,
    device_memory: ash::vk::DeviceMemory,
}

impl MyUniformBuffer {
    pub fn new(context: &MyRenderingContext, bytes_count: usize) -> MyUniformBuffer {
        unsafe {
            let buffer_create_info = ash::vk::BufferCreateInfo {
                s_type: ash::vk::StructureType::BUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                size: bytes_count as u64,
                usage: ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: std::ptr::null(),
            };
            let uniform_buffer = context
                .logical_device
                .create_buffer(&buffer_create_info, None)
                .expect("Cannot create uniform buffer");
            let buffer_requirements = context
                .logical_device
                .get_buffer_memory_requirements(uniform_buffer);
            let memory_allocate_info = ash::vk::MemoryAllocateInfo {
                s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                allocation_size: buffer_requirements.size,
                memory_type_index: search_physical_device_memory_type(
                    &context.instance,
                    &context.gpu,
                    &buffer_requirements,
                    ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                        | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
                )
                .expect("Cannot find memory type for uniform buffer memory")
                    as u32,
            };
            let device_memory = context
                .logical_device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Cannot allocate memory for uniform buffer");
            context
                .logical_device
                .bind_buffer_memory(uniform_buffer, device_memory, 0)
                .expect("Cannot bind uniform buffer to its memory");
            MyUniformBuffer {
                id: uniform_buffer,
                device_memory: device_memory,
            }
        }
    }

    pub unsafe fn update<T>(&self, logical_device: &ash::Device, content: &T) {
        let bytes_count = std::mem::size_of::<T>();
        let p_data = logical_device
            .map_memory(
                self.device_memory,
                0,
                bytes_count as ash::vk::DeviceSize,
                Default::default(),
            )
            .expect("Cannot map device memory");
        std::ptr::copy_nonoverlapping(
            content as *const T as *const std::ffi::c_void,
            p_data,
            bytes_count,
        );
        logical_device.unmap_memory(self.device_memory);
    }
}
