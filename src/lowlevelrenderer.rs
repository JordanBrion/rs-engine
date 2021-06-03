use crate::frame::*;
use crate::meshloader::*;
use crate::mvp::*;
use crate::renderingcontext::*;
use crate::surface::*;
use crate::swapchain::*;
use crate::uniformbuffer::*;
use crate::vertexbuffer::*;
use crate::window::*;
use ash::version::DeviceV1_0;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ffi::CString;
use std::ops::Index;
use std::time::Duration;

const FRAME_COUNT: usize = 2;

pub struct MyLowLevelRenderer {
    window: MyWindow,
    context: MyRenderingContext,
    surface: MySurface,
    swapchain: MySwapchain,
    vertex_buffer: MyVertexBuffer,
    index_buffer: MyIndexBuffer,
    v_frames: Vec<MyFrame>,
}

impl MyLowLevelRenderer {
    pub fn run(&self) {
        unsafe {
            let fence_create_info = ash::vk::FenceCreateInfo {
                s_type: ash::vk::StructureType::FENCE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: ash::vk::FenceCreateFlags::SIGNALED,
            };

            let semaphore_acquired_image_create_info = ash::vk::SemaphoreCreateInfo {
                s_type: ash::vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
            };

            let semaphore_pipeline_done_create_info = ash::vk::SemaphoreCreateInfo {
                s_type: ash::vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
            };

            let v_fences_wait_gpu = [
                self.context
                    .logical_device
                    .create_fence(&fence_create_info, None)
                    .expect("Cannot create fence"),
                self.context
                    .logical_device
                    .create_fence(&fence_create_info, None)
                    .expect("Cannot create fence"),
            ];
            let mut v_fences_ref_wait_gpu = vec![ash::vk::Fence::null(); self.swapchain.size()];
            let mut v_semaphores_acquired_image = Vec::with_capacity(FRAME_COUNT);
            let mut v_semaphores_pipeline_done = Vec::with_capacity(FRAME_COUNT);

            for _ in 0..FRAME_COUNT {
                v_semaphores_acquired_image.push(
                    self.context
                        .logical_device
                        .create_semaphore(&semaphore_acquired_image_create_info, None)
                        .expect("Cannot create sempahore"),
                );
                v_semaphores_pipeline_done.push(
                    self.context
                        .logical_device
                        .create_semaphore(&semaphore_pipeline_done_create_info, None)
                        .expect("Cannot create sempahore"),
                );
            }

            let mut event_pump = self
                .window
                .sdl_context
                .event_pump()
                .expect("Cannot get sdl event pump");
            let mut go = true;
            let mut current_frame_index = 0;
            let mut matrices = MyMvp {
                m_model: glm::identity(),
                m_view: glm::look_at(
                    &glm::vec3(0.0, 0.0, 4.0),
                    &glm::vec3(0.0, 0.0, 0.0),
                    &glm::vec3(0.0, 1.0, 0.0),
                ),
                m_projection: glm::perspective(16.0f32 / 9.0f32, 45.0f32, 1.0f32, 100.0f32),
            };

            while go {
                go = Self::handle_events(&mut event_pump);

                self.context
                    .logical_device
                    .wait_for_fences(&[v_fences_wait_gpu[current_frame_index]], true, !(0 as u64))
                    .expect("Cannot wait for fences");

                let infos_of_acquired_image = self
                    .swapchain
                    .loader
                    .acquire_next_image(
                        self.swapchain.inner,
                        !(0 as u64),
                        v_semaphores_acquired_image[current_frame_index],
                        ash::vk::Fence::null(),
                    )
                    .expect("Cannot acquire next image");

                let index_of_acquired_image = infos_of_acquired_image.0 as usize;

                if v_fences_ref_wait_gpu[index_of_acquired_image] != ash::vk::Fence::null() {
                    self.context
                        .logical_device
                        .wait_for_fences(
                            &[v_fences_ref_wait_gpu[index_of_acquired_image]],
                            true,
                            !(0 as u64),
                        )
                        .expect("Cannot wait for fences");
                }

                v_fences_ref_wait_gpu[index_of_acquired_image] =
                    v_fences_wait_gpu[current_frame_index];

                self.context
                    .logical_device
                    .reset_fences(&[v_fences_ref_wait_gpu[index_of_acquired_image]])
                    .expect("Cannot reset fences");

                let current_frame = self.v_frames.index(index_of_acquired_image);
                current_frame
                    .uniform_buffer
                    .update(&self.context.logical_device, &mut matrices);

                let wait_stage_submit_info = ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
                let submit_info = ash::vk::SubmitInfo {
                    s_type: ash::vk::StructureType::SUBMIT_INFO,
                    p_next: std::ptr::null(),
                    wait_semaphore_count: 1,
                    p_wait_semaphores: &v_semaphores_acquired_image[current_frame_index],
                    p_wait_dst_stage_mask: &wait_stage_submit_info
                        as *const ash::vk::PipelineStageFlags,
                    command_buffer_count: 1,
                    p_command_buffers: &current_frame.command_buffer,
                    signal_semaphore_count: 1,
                    p_signal_semaphores: &v_semaphores_pipeline_done[current_frame_index],
                };
                self.context
                    .logical_device
                    .queue_submit(
                        self.context.queue,
                        &[submit_info],
                        v_fences_ref_wait_gpu[index_of_acquired_image],
                    )
                    .expect("Cannot submit queue");

                let present_info = ash::vk::PresentInfoKHR {
                    s_type: ash::vk::StructureType::PRESENT_INFO_KHR,
                    p_next: std::ptr::null(),
                    wait_semaphore_count: 1,
                    p_wait_semaphores: &v_semaphores_pipeline_done[current_frame_index],
                    swapchain_count: 1,
                    p_swapchains: &self.swapchain.inner,
                    p_image_indices: &infos_of_acquired_image.0,
                    p_results: std::ptr::null_mut(),
                };
                self.swapchain
                    .loader
                    .queue_present(self.context.queue, &present_info)
                    .expect("Cannot present image");

                current_frame_index = (current_frame_index + 1) % FRAME_COUNT;
            }
        }
    }

    fn handle_events(event_pump: &mut sdl2::EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => return true,
            }
        }
        true
    }
}
pub struct MyLowLevelRendererBuilder {
    window: MyWindow,
    context: MyRenderingContext,
    surface: MySurface,
    swapchain: MySwapchain,
    pipeline_layout: ash::vk::PipelineLayout,
    render_pass: ash::vk::RenderPass,
    graphics_pipeline: ash::vk::Pipeline,
    descriptor_pool: ash::vk::DescriptorPool,
    descriptor_set_layout: ash::vk::DescriptorSetLayout,
    vertex_buffer: Option<MyVertexBuffer>,
    index_buffer: Option<MyIndexBuffer>,
}

impl MyLowLevelRendererBuilder {
    pub fn new() -> MyLowLevelRendererBuilder {
        unsafe {
            let mut window = MyWindow::new().unwrap();
            let context = MyRenderingContext::new(&window);
            let surface = MySurface::new(&context, &mut window).unwrap();
            let swapchain = MySwapchain::new(&context, &window, &surface);

            let shader_entry_name =
                CString::new("main").expect("Cannot create vertex shader entry name");
            let v_pipeline_shader_stage_create_infos = [
                ash::vk::PipelineShaderStageCreateInfo {
                    s_type: ash::vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next: std::ptr::null(),
                    flags: Default::default(),
                    stage: ash::vk::ShaderStageFlags::VERTEX,
                    module: Self::create_shader_module(
                        &context.logical_device,
                        "shaders/006_spinning_triangle.vert.spv",
                    ),
                    p_name: shader_entry_name.as_ptr(),
                    p_specialization_info: std::ptr::null(),
                },
                ash::vk::PipelineShaderStageCreateInfo {
                    s_type: ash::vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                    p_next: std::ptr::null(),
                    flags: Default::default(),
                    stage: ash::vk::ShaderStageFlags::FRAGMENT,
                    module: Self::create_shader_module(
                        &context.logical_device,
                        "shaders/006_spinning_triangle.frag.spv",
                    ),
                    p_name: shader_entry_name.as_ptr(),
                    p_specialization_info: std::ptr::null(),
                },
            ];

            let vertex_input_binding_description = ash::vk::VertexInputBindingDescription {
                binding: 0,
                stride: std::mem::size_of::<MyVec3>() as u32,
                input_rate: ash::vk::VertexInputRate::VERTEX,
            };

            let v_vertex_input_attribute_description =
                &[ash::vk::VertexInputAttributeDescription {
                    location: 1,
                    binding: 0,
                    format: ash::vk::Format::R32G32B32_SFLOAT,
                    offset: 0,
                }];

            let vertex_input_state_create_info = ash::vk::PipelineVertexInputStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                vertex_binding_description_count: 1,
                p_vertex_binding_descriptions: &vertex_input_binding_description,
                vertex_attribute_description_count: v_vertex_input_attribute_description.len()
                    as u32,
                p_vertex_attribute_descriptions: v_vertex_input_attribute_description.as_ptr(),
            };

            let input_assembly_state_create_info = ash::vk::PipelineInputAssemblyStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                topology: ash::vk::PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: ash::vk::FALSE,
            };

            let viewport = ash::vk::Viewport {
                x: 0f32,
                y: 0f32,
                width: window.dimensions.width as f32,
                height: window.dimensions.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            };

            let scissor = ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: ash::vk::Extent2D {
                    width: window.dimensions.width,
                    height: window.dimensions.height,
                },
            };

            let viewport_state_create_info = ash::vk::PipelineViewportStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                viewport_count: 1,
                p_viewports: &viewport,
                scissor_count: 1,
                p_scissors: &scissor,
            };

            let rasterization_state_create_info = ash::vk::PipelineRasterizationStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                depth_clamp_enable: ash::vk::FALSE,
                rasterizer_discard_enable: ash::vk::FALSE,
                polygon_mode: ash::vk::PolygonMode::FILL,
                cull_mode: ash::vk::CullModeFlags::NONE,
                front_face: ash::vk::FrontFace::CLOCKWISE,
                depth_bias_enable: ash::vk::FALSE,
                depth_bias_constant_factor: 0f32,
                depth_bias_clamp: 0f32,
                depth_bias_slope_factor: 0f32,
                line_width: 1f32,
            };

            let multisample_state_create_info = ash::vk::PipelineMultisampleStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                rasterization_samples: ash::vk::SampleCountFlags::TYPE_1,
                sample_shading_enable: ash::vk::FALSE,
                min_sample_shading: 0f32,
                p_sample_mask: std::ptr::null(),
                alpha_to_coverage_enable: ash::vk::FALSE,
                alpha_to_one_enable: ash::vk::FALSE,
            };

            let depth_stencil_state_create_info = ash::vk::PipelineDepthStencilStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                depth_test_enable: ash::vk::TRUE,
                depth_write_enable: ash::vk::TRUE,
                depth_compare_op: ash::vk::CompareOp::LESS,
                depth_bounds_test_enable: ash::vk::FALSE,
                stencil_test_enable: ash::vk::FALSE,
                front: Default::default(),
                back: Default::default(),
                min_depth_bounds: 0f32,
                max_depth_bounds: 1f32,
            };

            let color_blend_attachment = ash::vk::PipelineColorBlendAttachmentState {
                blend_enable: ash::vk::FALSE,
                src_color_blend_factor: ash::vk::BlendFactor::ONE,
                dst_color_blend_factor: ash::vk::BlendFactor::ZERO,
                color_blend_op: ash::vk::BlendOp::ADD,
                src_alpha_blend_factor: ash::vk::BlendFactor::ONE,
                dst_alpha_blend_factor: ash::vk::BlendFactor::ZERO,
                alpha_blend_op: ash::vk::BlendOp::ADD,
                color_write_mask: ash::vk::ColorComponentFlags::R
                    | ash::vk::ColorComponentFlags::G
                    | ash::vk::ColorComponentFlags::B
                    | ash::vk::ColorComponentFlags::A,
            };

            let color_blend_state_create_info = ash::vk::PipelineColorBlendStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                logic_op_enable: ash::vk::FALSE,
                logic_op: ash::vk::LogicOp::COPY,
                attachment_count: 1,
                p_attachments: &color_blend_attachment,
                blend_constants: [0f32; 4],
            };

            let dynamic_state_create_info = ash::vk::PipelineDynamicStateCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                dynamic_state_count: 0 as u32,
                p_dynamic_states: std::ptr::null(),
            };

            let uniform_buffer_binding_number = 5;
            let descriptor_set_layout_binding = ash::vk::DescriptorSetLayoutBinding {
                binding: uniform_buffer_binding_number,
                descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: ash::vk::ShaderStageFlags::VERTEX,
                p_immutable_samplers: std::ptr::null(),
            };

            let descriptor_set_layout_create_info = ash::vk::DescriptorSetLayoutCreateInfo {
                s_type: ash::vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                binding_count: 1,
                p_bindings: &descriptor_set_layout_binding,
            };

            let descriptor_set_layout = context
                .logical_device
                .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
                .expect("Cannot create descriptor set layout");

            let pipeline_layout_create_info = ash::vk::PipelineLayoutCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                set_layout_count: 1,
                p_set_layouts: &descriptor_set_layout,
                push_constant_range_count: 0,
                p_push_constant_ranges: std::ptr::null(),
            };

            let pipeline_layout = context
                .logical_device
                .create_pipeline_layout(&pipeline_layout_create_info, None)
                .expect("Cannot create pipeline layout");

            let attachment_description = ash::vk::AttachmentDescription {
                flags: Default::default(),
                format: surface.format.format,
                samples: ash::vk::SampleCountFlags::TYPE_1,
                load_op: ash::vk::AttachmentLoadOp::CLEAR,
                store_op: ash::vk::AttachmentStoreOp::STORE,
                stencil_load_op: ash::vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: ash::vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: ash::vk::ImageLayout::UNDEFINED,
                final_layout: ash::vk::ImageLayout::PRESENT_SRC_KHR,
            };

            let color_attachment_reference = ash::vk::AttachmentReference {
                attachment: 0,
                layout: ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            };

            let subpass_description = ash::vk::SubpassDescription {
                flags: Default::default(),
                pipeline_bind_point: ash::vk::PipelineBindPoint::GRAPHICS,
                input_attachment_count: 0,
                p_input_attachments: std::ptr::null(),
                color_attachment_count: 1,
                p_color_attachments: &color_attachment_reference,
                p_resolve_attachments: std::ptr::null(),
                p_depth_stencil_attachment: std::ptr::null(),
                preserve_attachment_count: 0,
                p_preserve_attachments: std::ptr::null(),
            };

            let render_pass_create_info = ash::vk::RenderPassCreateInfo {
                s_type: ash::vk::StructureType::RENDER_PASS_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: Default::default(),
                attachment_count: 1,
                p_attachments: &attachment_description,
                subpass_count: 1,
                p_subpasses: &subpass_description,
                dependency_count: 0,
                p_dependencies: std::ptr::null(),
            };
            let render_pass = context
                .logical_device
                .create_render_pass(&render_pass_create_info, None)
                .expect("Cannot create render pass");

            let graphics_pipeline_create_info = ash::vk::GraphicsPipelineCreateInfo {
                s_type: ash::vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: ash::vk::PipelineCreateFlags::DISABLE_OPTIMIZATION,
                stage_count: v_pipeline_shader_stage_create_infos.len() as u32,
                p_stages: v_pipeline_shader_stage_create_infos.as_ptr(),
                p_vertex_input_state: &vertex_input_state_create_info,
                p_input_assembly_state: &input_assembly_state_create_info,
                p_tessellation_state: std::ptr::null(),
                p_viewport_state: &viewport_state_create_info,
                p_rasterization_state: &rasterization_state_create_info,
                p_multisample_state: &multisample_state_create_info,
                p_depth_stencil_state: &depth_stencil_state_create_info,
                p_color_blend_state: &color_blend_state_create_info,
                p_dynamic_state: &dynamic_state_create_info,
                layout: pipeline_layout,
                render_pass: render_pass,
                subpass: 0,
                base_pipeline_handle: ash::vk::Pipeline::null(),
                base_pipeline_index: -1,
            };
            let v_graphics_pipelines = context
                .logical_device
                .create_graphics_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[graphics_pipeline_create_info],
                    None,
                )
                .expect("Cannot create graphics pipeline");
            let graphics_pipeline = v_graphics_pipelines[0];
            let descriptor_pool_size = ash::vk::DescriptorPoolSize {
                ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
                descriptor_count: swapchain.size() as u32,
            };
            let descriptor_pool_create_info = ash::vk::DescriptorPoolCreateInfo {
                s_type: ash::vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: ash::vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
                max_sets: swapchain.size() as u32,
                pool_size_count: 1,
                p_pool_sizes: &descriptor_pool_size,
            };
            let descriptor_pool = context
                .logical_device
                .create_descriptor_pool(&descriptor_pool_create_info, None)
                .expect("Cannot create descriptor pool");

            MyLowLevelRendererBuilder {
                window: window,
                context: context,
                surface: surface,
                swapchain: swapchain,
                render_pass: render_pass,
                pipeline_layout: pipeline_layout,
                graphics_pipeline: graphics_pipeline,
                descriptor_pool: descriptor_pool,
                descriptor_set_layout: descriptor_set_layout,
                vertex_buffer: None,
                index_buffer: None,
            }
        }
    }

    pub fn mesh(mut self, mesh: &Mesh) -> MyLowLevelRendererBuilder {
        self.vertex_buffer = Some(MyVertexBuffer::new(&self.context, &mesh.vertices));
        self.index_buffer = Some(MyIndexBuffer::new(&self.context, &mesh.indices));
        self
    }

    pub fn build(self) -> MyLowLevelRenderer {
        let v_frames = self.allocate_frames();
        if let (Some(vertex_buffer), Some(index_buffer)) = (self.vertex_buffer, self.index_buffer) {
            return MyLowLevelRenderer {
                window: self.window,
                context: self.context,
                surface: self.surface,
                swapchain: self.swapchain,
                vertex_buffer: vertex_buffer,
                index_buffer: index_buffer,
                v_frames: v_frames,
            };
        } else {
            panic!("You need a vertex buffer and an index buffer to build the renderer.");
        }
    }

    fn allocate_frames(&self) -> Vec<MyFrame> {
        let count = self.swapchain.size();
        let mut v_frames = Vec::with_capacity(count);

        for i in 0..count {
            v_frames.push(MyFrame::new(
                &self.context,
                &self.window,
                &self.surface,
                &self.swapchain,
                &self.swapchain.v_images[i],
                &self.render_pass,
                &self.pipeline_layout,
                &self.graphics_pipeline,
                &self.descriptor_pool,
                &self.descriptor_set_layout,
                MyUniformBuffer::new(&self.context, std::mem::size_of::<MyMvp>()),
                self.vertex_buffer.as_ref().unwrap(),
                self.index_buffer.as_ref().unwrap(),
            ));
        }
        v_frames
    }

    unsafe fn create_shader_module(
        logical_device: &ash::Device,
        shader_path: &str,
    ) -> ash::vk::ShaderModule {
        let mut shader_files =
            std::fs::File::open(shader_path).expect("Something went wrong when opening shader");
        let shader_instructions =
            ash::util::read_spv(&mut shader_files).expect("Failed to read shader spv file");
        let shader_module_create_infos =
            ash::vk::ShaderModuleCreateInfo::builder().code(shader_instructions.as_slice());
        logical_device
            .create_shader_module(&shader_module_create_infos, None)
            .expect("Cannot create shader module")
    }
}
