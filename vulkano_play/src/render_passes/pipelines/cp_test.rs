use std::sync::Arc;
use vulkano::{
    device::{
        Device,
        Queue
    },
    pipeline::{
        Pipeline,
        PipelineBindPoint,
        compute::ComputePipeline,
    }, 
    image::{
        StorageImage, ImageDimensions, view::ImageView
    }, 
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet
    }, 
    memory::allocator::{
        StandardMemoryAllocator, AllocationCreateInfo, MemoryUsage
    }, 
    buffer::{
        Buffer, 
        BufferCreateInfo, 
        BufferUsage, 
        Subbuffer
    }, 
    format::Format, command_buffer::CopyImageToBufferInfo
};

use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator,
    StandardCommandBufferAllocatorCreateInfo
};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
    SubpassContents, SecondaryAutoCommandBuffer,
};

mod cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path : "shaders/test/cp.comp"
    }
}

pub fn create(
    device: &Arc<Device>
) -> Arc<ComputePipeline> {
    let shader = cs::load(device.clone()).expect("failed to create shader module");

    ComputePipeline::new(
        device.clone(),
        shader.entry_point("main").unwrap(),
        &(),
        None,
        |_| {},
    ).expect("failed to create compute pipeline")
}


pub fn create_pcb(
    device: &Arc<Device>, 
    queue: &Arc<Queue>, 
    buf: &Subbuffer<[u8]>
) -> Arc<PrimaryAutoCommandBuffer> {
    
    let mem_allocator = StandardMemoryAllocator::new_default(device.clone());
    let image = StorageImage::new(
        &mem_allocator,
        ImageDimensions::Dim2d {
            width: 1024,
            height: 1024,
            array_layers: 1,
        },
        Format::R8G8B8A8_UNORM,
        Some(queue.queue_family_index()),
    ).unwrap();
    let view = ImageView::new_default(image.clone()).unwrap();

    let pipeline = create(device);
    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    
    let ds_alloc = StandardDescriptorSetAllocator::new(device.clone());
    let ds = PersistentDescriptorSet::new(
        &ds_alloc,
        layout.clone(),
        [
            WriteDescriptorSet::image_view(0, view) // 0 is the binding
        ], 
    ).unwrap();

    let cb_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
    let mut cb_builder = AutoCommandBufferBuilder::primary(
        &cb_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();
    
    cb_builder
        .bind_pipeline_compute(pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            ds,
        )
        .dispatch([1024 / 8, 1024 / 8, 1]).unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, buf.clone())).unwrap();

    Arc::new(cb_builder.build().unwrap())
}




