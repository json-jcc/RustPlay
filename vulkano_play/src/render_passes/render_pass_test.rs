
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
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::Swapchain;

use crate::render_passes::pipelines::gp_test;

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

fn get_command_buffers(
    command_buffer_allocator: &StandardCommandBufferAllocator,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &[Arc<Framebuffer>],
    vertex_buffer: &Subbuffer<[gp_test::Vert]>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .map(|framebuffer| {
            let builder = AutoCommandBufferBuilder::primary(
                command_buffer_allocator,
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit,
            )
            .unwrap();
            
            build_pcb(builder, pipeline, framebuffer, vertex_buffer)
        })
        .collect()
}

fn build_pcb(
    mut pcb_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffer: &Arc<Framebuffer>,
    vertex_buffer: &Subbuffer<[gp_test::Vert]>,
) -> Arc<PrimaryAutoCommandBuffer> {
    pcb_builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassContents::Inline,
        ).unwrap()
        //.bind_index_buffer(index_buffer)
        //.bind_descriptor_sets(pipeline_bind_point, pipeline_layout, first_set, descriptor_sets)
        .bind_pipeline_graphics(pipeline.clone())
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .draw(vertex_buffer.len() as u32, 1, 0, 0).unwrap()
        .end_render_pass().unwrap();

    Arc::new(pcb_builder.build().unwrap())
}

fn build_scb(
    mut scb_builder: AutoCommandBufferBuilder<SecondaryAutoCommandBuffer>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffer: &Arc<Framebuffer>,
    vertex_buffer: &Subbuffer<[gp_test::Vert]>,
) -> Arc<SecondaryAutoCommandBuffer> {
    scb_builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassContents::SecondaryCommandBuffers,
    ).unwrap()
    .bind_pipeline_graphics(pipeline.clone())
    .bind_vertex_buffers(0, vertex_buffer.clone())
    .draw(vertex_buffer.len() as u32, 1, 0, 0).unwrap()
    .end_render_pass().unwrap();

    Arc::new(scb_builder.build().unwrap())
}

fn build_pcb_with_scb(
    mut pcb_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    framebuffer: &Arc<Framebuffer>,
) -> Arc<PrimaryAutoCommandBuffer> {

    // let mut scb_building_tasks: Vec<_> = Vec::new();

    // scb_building_tasks.push(std::thread::spawn(|| -> Arc<SecondaryAutoCommandBuffer>{ Arc::new(scb_builder.build().unwrap())))}));

    // let scbs = scb_building_tasks
    //     .into_iter()
    //     .map(|task| task.join().unwrap())
    //     .collect::<Vec<_>>();

    pcb_builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassContents::Inline,
        ).unwrap();

    // for scb in scbs {
    //     pcb_builder
    //         .execute_commands(scb.clone()).unwrap();
    // }
        
    pcb_builder.end_render_pass().unwrap();

    Arc::new(pcb_builder.build().unwrap())
}


pub fn create_command_buffers(
    device: &Arc<Device>, 
    queue: &Arc<Queue>, 
    vertex_buffer: &Subbuffer<[gp_test::Vert]>,
    swapchain: &Arc<Swapchain>,
    viewport: &Viewport,
    images: &Vec<Arc<SwapchainImage>>
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {

    let render_pass = get_render_pass(
        device.clone(), 
        swapchain.clone()
    );

    let framebuffers = get_framebuffers(
        &images, 
        &render_pass
    );

    let pipeline = gp_test::create(&device, render_pass.clone(), viewport.clone());

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    get_command_buffers(
        &command_buffer_allocator,
        &queue,
        &pipeline,
        &framebuffers,
        &vertex_buffer,
    )
}