use crate::devicememory::search_physical_device_memory_type;
use crate::renderingcontext::*;
use crate::stagingbuffer::MyStagingBuffer;
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk::Handle;

pub struct MyVertexBuffer {
    pub id: ash::vk::Buffer,
    pub offset: usize,
    pub number_of_vertices: usize,
}

impl MyVertexBuffer {
    pub fn new<T>(context: &MyRenderingContext, content: &Vec<T>) -> MyVertexBuffer {
        unsafe {
            let number_of_vertices = content.len();
            let bytes_count = number_of_vertices * std::mem::size_of::<T>();
            let staging_buffer = MyStagingBuffer::new(context, content);
            let vertex_buffer = Self::allocate_buffer(context, bytes_count);
            Self::copy_buffer(context, &staging_buffer.id, &vertex_buffer, 0, bytes_count);

            // TODO define function to destroy allocated objects
            context
                .logical_device
                .destroy_buffer(staging_buffer.id, None);
            context
                .logical_device
                .free_memory(staging_buffer.memory_id, None);
            // END TODO

            MyVertexBuffer {
                id: vertex_buffer,
                offset: 0 as usize,
                number_of_vertices: number_of_vertices,
            }
        }
    }

    unsafe fn allocate_buffer(context: &MyRenderingContext, bytes_count: usize) -> ash::vk::Buffer {
        // VERTEX BUFFER CREATION
        let vertex_buffer_create_info = ash::vk::BufferCreateInfo {
            s_type: ash::vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: Default::default(),
            size: bytes_count as u64,
            usage: ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
            sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
        };

        let vertex_buffer = context
            .logical_device
            .create_buffer(&vertex_buffer_create_info, None)
            .expect("Cannot create vertex buffer");

        let vertex_buffer_memory_requirements = context
            .logical_device
            .get_buffer_memory_requirements(vertex_buffer);

        let vertex_buffer_memory_type_index = search_physical_device_memory_type(
            &context.instance,
            &context.gpu,
            &vertex_buffer_memory_requirements,
            ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
        .unwrap();

        let memory_allocate_info_for_vertex_buffer = ash::vk::MemoryAllocateInfo {
            s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            allocation_size: vertex_buffer_memory_requirements.size,
            memory_type_index: vertex_buffer_memory_type_index as u32,
        };

        let device_memory_for_vertex_buffer = context
            .logical_device
            .allocate_memory(&memory_allocate_info_for_vertex_buffer, None)
            .expect("Cannot allocate memory for vertex buffer");
        context
            .logical_device
            .bind_buffer_memory(vertex_buffer, device_memory_for_vertex_buffer, 0)
            .expect("Cannot bind memory for vertex buffer");
        vertex_buffer
    }

    unsafe fn copy_buffer(
        context: &MyRenderingContext,
        src: &ash::vk::Buffer,
        dst: &ash::vk::Buffer,
        offset: usize,
        bytes_count: usize,
    ) {
        let copy_command_buffer_allocate_info = ash::vk::CommandBufferAllocateInfo {
            s_type: ash::vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            command_pool: context.command_pool,
            level: ash::vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: 1,
        };

        let command_buffer_copy_buffer = context
            .logical_device
            .allocate_command_buffers(&copy_command_buffer_allocate_info)
            .expect("Cannot allocate command buffer to copy staging buffer")[0];
        let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo {
            s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            flags: ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: std::ptr::null(),
        };
        context
            .logical_device
            .begin_command_buffer(command_buffer_copy_buffer, &command_buffer_begin_info)
            .expect("Cannot begin command buffer to copy staging buffer");
        let buffer_copy = ash::vk::BufferCopy {
            src_offset: offset as u64,
            dst_offset: offset as u64,
            size: bytes_count as u64,
        };
        context.logical_device.cmd_copy_buffer(
            command_buffer_copy_buffer,
            *src,
            *dst,
            &[buffer_copy],
        );
        context
            .logical_device
            .end_command_buffer(command_buffer_copy_buffer)
            .expect("Cannot end command buffer to copy staging buffer");
        let copy_buffer_submit_info = ash::vk::SubmitInfo {
            s_type: ash::vk::StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: std::ptr::null(),
            p_wait_dst_stage_mask: std::ptr::null(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffer_copy_buffer,
            signal_semaphore_count: 0,
            p_signal_semaphores: std::ptr::null(),
        };
        context
            .logical_device
            .queue_submit(
                context.queue,
                &[copy_buffer_submit_info],
                ash::vk::Fence::null(),
            )
            .expect("Cannot submit command buffer to copy staging buffer");
        context
            .logical_device
            .queue_wait_idle(context.queue)
            .expect("Cannot wait for queue to copy staging buffer");

        context
            .logical_device
            .free_command_buffers(context.command_pool, &[command_buffer_copy_buffer]);
    }
}

pub struct MyIndexBuffer {}

impl MyIndexBuffer {
    pub fn new(context: &MyRenderingContext, indices: &Vec<u16>) -> MyIndexBuffer {
        unsafe {
            let number_of_bytes =
                (indices.len() * std::mem::size_of_val(&indices[0])) as ash::vk::DeviceSize;
            let buffer_create_info = ash::vk::BufferCreateInfo {
                s_type: ash::vk::StructureType::BUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                size: number_of_bytes,
                usage: ash::vk::BufferUsageFlags::INDEX_BUFFER,
                sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: context.index_of_queue_family as *const u32,
            };
            let buffer = context
                .logical_device
                .create_buffer(&buffer_create_info, None)
                .unwrap();
            let memory_requirements = context
                .logical_device
                .get_buffer_memory_requirements(buffer);
            let memory_allocate_info = ash::vk::MemoryAllocateInfo {
                s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                allocation_size: number_of_bytes,
                memory_type_index: search_physical_device_memory_type(
                    &context.instance,
                    &context.gpu,
                    &memory_requirements,
                    ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
                )
                .unwrap() as u32,
            };
            let memory = context
                .logical_device
                .allocate_memory(&memory_allocate_info, None)
                .unwrap();
            context
                .logical_device
                .bind_buffer_memory(buffer, memory, 0)
                .unwrap();
            MyIndexBuffer {}
        }
    }
}
