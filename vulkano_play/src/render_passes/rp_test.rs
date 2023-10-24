
use std::sync::Arc;

use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassContents, SecondaryAutoCommandBuffer,
};
use vulkano::device::{
    Device, Queue
};
use vulkano::image::view::ImageView;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::{
    GraphicsPipeline, 
    PipelineBindPoint
};
use vulkano::pipeline::Pipeline;
use vulkano::render_pass::{
    Framebuffer, 
    FramebufferCreateInfo, 
    RenderPass
};
use vulkano::swapchain::Swapchain;
use vulkano::descriptor_set::{
    PersistentDescriptorSet, 
    WriteDescriptorSet
};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::memory::allocator::{
    AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator
};
use vulkano::buffer::{
    Buffer, BufferCreateInfo, BufferUsage
};

use crate::render_passes::pipelines::gp_test;



pub struct RenderPassTest {

    device: Arc<Device>,
    rp: Arc<RenderPass>,
    
    pipeline: Arc<GraphicsPipeline>,
    descriptor_sets: Vec<Arc<PersistentDescriptorSet>>
}

impl RenderPassTest {

    fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(), // set the format the same as the swapchain
                    samples: 1,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap()
    }
    
    fn get_framebuffers(
        images: &[Arc<SwapchainImage>],
        render_pass: &Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
   
    fn create_descriptor_sets(device: &Arc<Device>, pipeline: &Arc<GraphicsPipeline>) -> Arc<PersistentDescriptorSet>{
        let descriptor_set_layouts = pipeline.layout().set_layouts();

        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

        let data_iter = 0..65536u32;
        let data_buffer = Buffer::from_iter(
            &memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            data_iter,
        ).expect("failed to create buffer");

        
        //let sampler = Sampler::simple_repeat_linear(device.clone());
        PersistentDescriptorSet::new(
            &descriptor_set_allocator,
            descriptor_set_layouts.get(0).unwrap().clone(),
            [
                WriteDescriptorSet::buffer(0, data_buffer.clone()), // 0 is the binding
                //WriteDescriptorSet::sampler(0, sampler) // 1 is the binding
                ], 
        ).unwrap()
    }

    pub fn new(
        device: &Arc<Device>, 
        swapchain: &Arc<Swapchain>,
        viewport: &Viewport
    ) -> Self {
        let rp = RenderPassTest::get_render_pass(
            device.clone(), 
            swapchain.clone()
        );

        let pipeline_subpass_0 = gp_test::create(
            &device, 
            rp.clone(), 0, 
            viewport.clone()
        );

        Self {
            device: device.clone(),
            rp: rp,
            pipeline: pipeline_subpass_0.clone(),
            descriptor_sets: vec![
                RenderPassTest::create_descriptor_sets(device, &pipeline_subpass_0)
            ]
        }
    }

    pub fn record(
        &self,
        cb_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        fbo: &Arc<Framebuffer>,
        vbo: &Subbuffer<[gp_test::Vert]>,
        ebo: &Subbuffer<[u32]>
    ){
        cb_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(fbo.clone())
                },
                SubpassContents::Inline,
            ).unwrap()
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, vbo.clone())
            .bind_index_buffer(ebo.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics, 
                self.pipeline.layout().clone(), 
                0, 
                self.descriptor_sets.clone()
            )
            .draw(vbo.len() as u32, 1, 0, 0).unwrap()
            .end_render_pass().unwrap();
    }

    pub fn create_command_buffers(
        &self,
        queue: &Arc<Queue>, 
        vbo: &Subbuffer<[gp_test::Vert]>,
        ebo: &Subbuffer<[u32]>,
        images: &Vec<Arc<SwapchainImage>>
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        let cb_alloc = StandardCommandBufferAllocator::new(self.device.clone(), Default::default());

        RenderPassTest::get_framebuffers(
            &images, 
            &self.rp
        )
        .iter()
        .map(|fbo| {
            let mut cb_builder = AutoCommandBufferBuilder::primary(
                &cb_alloc,
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit,
            ).unwrap();

            self.record(
                &mut cb_builder,
                &fbo,
                &vbo,
                &ebo
            );

            Arc::new(cb_builder.build().unwrap())
        }).collect::<Vec<_>>()
    }
}

