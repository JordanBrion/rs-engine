use crate::devicememory::search_physical_device_memory_type;
use crate::renderingcontext::*;
use ash::version::DeviceV1_0;

pub struct MyStagingBuffer {
    pub id: ash::vk::Buffer,
    pub memory_id: ash::vk::DeviceMemory,
}

impl MyStagingBuffer {
    pub fn new<T>(context: &MyRenderingContext, content: &Vec<T>) -> MyStagingBuffer {
        unsafe {
            let bytes_count = content.len() * std::mem::size_of_val(&content[0]);
            let staging_buffer_create_info = ash::vk::BufferCreateInfo {
                s_type: ash::vk::StructureType::BUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                size: bytes_count as u64,
                usage: ash::vk::BufferUsageFlags::TRANSFER_SRC,
                sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 0,
                p_queue_family_indices: std::ptr::null(),
            };

            let staging_buffer = context
                .logical_device
                .create_buffer(&staging_buffer_create_info, None)
                .expect("Cannot staging buffer");

            let staging_buffer_memory_requirements = context
                .logical_device
                .get_buffer_memory_requirements(staging_buffer);

            let staging_buffer_memory_type_index = search_physical_device_memory_type(
                &context.instance,
                &context.gpu,
                &staging_buffer_memory_requirements,
                ash::vk::MemoryPropertyFlags::HOST_COHERENT
                    | ash::vk::MemoryPropertyFlags::HOST_VISIBLE,
            )
            .unwrap();

            let memory_allocate_info_for_staging_buffer = ash::vk::MemoryAllocateInfo {
                s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                allocation_size: staging_buffer_memory_requirements.size,
                memory_type_index: staging_buffer_memory_type_index as u32,
            };

            let device_memory = context
                .logical_device
                .allocate_memory(&memory_allocate_info_for_staging_buffer, None)
                .expect("Cannot allocate memory for staging buffer");

            let memory_offset = 0 as ash::vk::DeviceSize;
            context
                .logical_device
                .bind_buffer_memory(staging_buffer, device_memory, memory_offset)
                .expect("Cannot bind memory for staging buffer");

            let p_data = context
                .logical_device
                .map_memory(
                    device_memory,
                    memory_offset,
                    staging_buffer_create_info.size,
                    Default::default(),
                )
                .expect("Cannot map memory");
            std::ptr::copy_nonoverlapping(
                content.as_ptr() as *const std::ffi::c_void,
                p_data,
                bytes_count,
            );
            context.logical_device.unmap_memory(device_memory);

            MyStagingBuffer {
                id: staging_buffer,
                memory_id: device_memory,
            }
        }
    }
}
