use std::ops::Index;

use crate::renderingcontext::*;
use crate::surface::MySurface;
use crate::swapchain::MySwapchain;
use crate::uniformbuffer::*;
use crate::vertexbuffer::MyVertexBuffer;
use crate::window::*;
use ash::version::DeviceV1_0;

pub struct MyFrame {
    pub uniform_buffer: MyUniformBuffer,
    pub command_buffer: ash::vk::CommandBuffer,
}

impl MyFrame {
    pub fn new(
        context: &MyRenderingContext,
        window: &MyWindow,
        surface: &MySurface,
        swapchain: &MySwapchain,
        image: &ash::vk::Image,
        render_pass: &ash::vk::RenderPass,
        pipeline_layout: &ash::vk::PipelineLayout,
        graphics_pipeline: &ash::vk::Pipeline,
        descriptor_pool: &ash::vk::DescriptorPool,
        descriptor_set_layout: &ash::vk::DescriptorSetLayout,
        uniform_buffer: MyUniformBuffer,
        vertex_buffer: &MyVertexBuffer,
    ) -> MyFrame {
        unsafe {
            let uniform_buffer_binding_number = 5; // TODO make dynamic
            let v_descriptor_set_layout = vec![*descriptor_set_layout; 1];
            let descriptor_set_allocate_info = ash::vk::DescriptorSetAllocateInfo {
                s_type: ash::vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                descriptor_pool: *descriptor_pool,
                descriptor_set_count: v_descriptor_set_layout.len() as u32,
                p_set_layouts: v_descriptor_set_layout.as_ptr(),
            };
            let descriptor_set = context
                .logical_device
                .allocate_descriptor_sets(&descriptor_set_allocate_info)
                .expect("Cannot allocate descriptor set")[0];
            let command_buffer_allocate_info = ash::vk::CommandBufferAllocateInfo {
                s_type: ash::vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: std::ptr::null(),
                command_pool: context.command_pool,
                level: ash::vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: swapchain.size() as u32,
            };
            let command_buffer = context
                .logical_device
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Cannot allocate command buffer")[0];
            let descriptor_buffer_info = ash::vk::DescriptorBufferInfo {
                buffer: uniform_buffer.id,
                offset: 0,
                range: ash::vk::WHOLE_SIZE,
            };
            let descriptor_write = ash::vk::WriteDescriptorSet {
                s_type: ash::vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: std::ptr::null(),
                dst_set: descriptor_set,
                dst_binding: uniform_buffer_binding_number,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: std::ptr::null(),
                p_buffer_info: &descriptor_buffer_info,
                p_texel_buffer_view: std::ptr::null(),
            };
            context
                .logical_device
                .update_descriptor_sets(&[descriptor_write], &[]);

            let component_mapping = ash::vk::ComponentMapping {
                r: ash::vk::ComponentSwizzle::IDENTITY,
                g: ash::vk::ComponentSwizzle::IDENTITY,
                b: ash::vk::ComponentSwizzle::IDENTITY,
                a: ash::vk::ComponentSwizzle::IDENTITY,
            };

            let subresource_range = ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            };

            let image_view_create_info = ash::vk::ImageViewCreateInfo {
                s_type: ash::vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                image: *image,
                view_type: ash::vk::ImageViewType::TYPE_2D,
                format: surface.format.format,
                components: component_mapping,
                subresource_range: subresource_range,
            };
            let image_view = context
                .logical_device
                .create_image_view(&image_view_create_info, None)
                .expect("Cannot create image view");

            let framebuffer_create_info = ash::vk::FramebufferCreateInfo {
                s_type: ash::vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                render_pass: *render_pass,
                attachment_count: 1,
                p_attachments: &image_view,
                width: window.dimensions.width,
                height: window.dimensions.height,
                layers: 1,
            };
            let framebuffer = context
                .logical_device
                .create_framebuffer(&framebuffer_create_info, None)
                .expect("Cannot create framebuffer");

            let render_area = ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: window.dimensions,
            };
            let clear_values = ash::vk::ClearValue {
                color: ash::vk::ClearColorValue {
                    float32: [1.0, 0.0, 1.0, 1.0],
                },
            };
            let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo {
                s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                p_inheritance_info: std::ptr::null(),
            };

            context
                .logical_device
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .expect("Cannot begin command buffer");

            let render_pass_begin_info = ash::vk::RenderPassBeginInfo {
                s_type: ash::vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: std::ptr::null(),
                render_pass: *render_pass,
                framebuffer: framebuffer,
                render_area: render_area,
                clear_value_count: 1,
                p_clear_values: &clear_values,
            };

            context.logical_device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                ash::vk::SubpassContents::INLINE,
            );
            context.logical_device.cmd_bind_descriptor_sets(
                command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                *pipeline_layout,
                0,
                &[descriptor_set],
                &[],
            );
            context.logical_device.cmd_bind_pipeline(
                command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                *graphics_pipeline,
            );
            context.logical_device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[vertex_buffer.id],
                &[vertex_buffer.offset as u64],
            );
            context.logical_device.cmd_draw(
                command_buffer,
                vertex_buffer.number_of_vertices as u32,
                1,
                0,
                0,
            );
            context.logical_device.cmd_end_render_pass(command_buffer);
            context
                .logical_device
                .end_command_buffer(command_buffer)
                .expect("Cannot end command buffer");
            MyFrame {
                uniform_buffer: uniform_buffer,
                command_buffer: command_buffer,
            }
        }
    }
}
