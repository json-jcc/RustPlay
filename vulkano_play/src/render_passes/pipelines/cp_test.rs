use std::sync::Arc;
use vulkano::{
    device::{
        Device,
        Queue
    },
    render_pass::{
        RenderPass,
        Subpass,
    },
    pipeline::{
        Pipeline,
        PipelineBindPoint,
        compute::ComputePipeline,
        graphics::{
            viewport::{
                Viewport,
                ViewportState
            },
            discard_rectangle::DiscardRectangleState,
            input_assembly::{
                InputAssemblyState
            }, 
            multisample::MultisampleState,
            rasterization::{ 
                RasterizationState,
            },
            depth_stencil::DepthStencilState, color_blend::ColorBlendState,
        }, 
        GraphicsPipeline
    }, image::{SampleCount, StorageImage, ImageDimensions, view::ImageView}, descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet}, 
        memory::allocator::{
            StandardMemoryAllocator, AllocationCreateInfo, MemoryUsage
        }, 
        buffer::{
            Buffer, 
            BufferCreateInfo, 
            BufferUsage, Subbuffer
        }, format::Format, command_buffer::CopyImageToBufferInfo
};

use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator,
    StandardCommandBufferAllocatorCreateInfo
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, RenderPassBeginInfo,
    SubpassContents, SecondaryAutoCommandBuffer,
};

mod cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path : "shaders/test/cp.comp"
    }
}

pub fn create(device: &Arc<Device>) -> Arc<ComputePipeline> {
     let shader = cs::load(device.clone()).expect("failed to create shader module");

    ComputePipeline::new(
        device.clone(),
        shader.entry_point("main").unwrap(),
        &(),
        None,
        |_| {},
    ).expect("failed to create compute pipeline")
}


pub fn create_pcb(device: &Arc<Device>, queue: &Arc<Queue>, buf: &Subbuffer<[u8]>) -> Arc<PrimaryAutoCommandBuffer> {
    let cb_alloc = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let cb_builder = AutoCommandBufferBuilder::primary(
        &cb_alloc,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    let pipeline = create(device);

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let image = StorageImage::new(
        &memory_allocator,
        ImageDimensions::Dim2d {
            width: 1024,
            height: 1024,
            array_layers: 1,
        },
        Format::R8G8B8A8_UNORM,
        Some(queue.queue_family_index()),
    ).unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();
    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());

    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [WriteDescriptorSet::image_view(0, view)], // 0 is the binding
    ).unwrap();

    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .bind_pipeline_compute(pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            set,
        )
        .dispatch([1024 / 8, 1024 / 8, 1])
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, buf.clone()))
        .unwrap();

    Arc::new(cb_builder.build().unwrap())
}




