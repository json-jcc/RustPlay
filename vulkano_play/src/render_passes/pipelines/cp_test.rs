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

use crate::render_passes::rg::PassGraph;

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


struct ResourcesTest {

    buffer : Subbuffer<[u8]>,
    image : Arc<StorageImage>,
    image_view : Arc<ImageView<StorageImage>>,
    ds: Arc<PersistentDescriptorSet>,
    pipeline: Arc<ComputePipeline>,
}

impl ResourcesTest{

    pub fn new(
        device: &Arc<Device>, 
        queue: &Arc<Queue>, 
        buf: &Subbuffer<[u8]>
    ) ->Self {
        let mem_allocator = StandardMemoryAllocator::new_default(device.clone());
        let image: Arc<StorageImage> = StorageImage::new(
            &mem_allocator,
            ImageDimensions::Dim2d {
                width: 1024,
                height: 1024,
                array_layers: 1,
            },
            Format::R8G8B8A8_UNORM,
            Some(queue.queue_family_index()),
        ).unwrap();
        let image_view = ImageView::new_default(image.clone()).unwrap();


        let pipeline = create(device);
        let layout = pipeline.layout().set_layouts().get(0).unwrap();
        
        let ds_alloc = StandardDescriptorSetAllocator::new(device.clone());
        let ds = PersistentDescriptorSet::new(
            &ds_alloc,
            layout.clone(),
            [
                WriteDescriptorSet::image_view(0, image_view.clone()) // 0 is the binding
            ], 
        ).unwrap();
        
        Self {
            buffer : buf.clone(),
            image,
            image_view,
            ds,
            pipeline,
        }
    }
}

pub fn record(
    device: &Arc<Device>, 
    queue: &Arc<Queue>, 
    buf: &Subbuffer<[u8]>,
    pcb_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>
) {
    let resources = ResourcesTest::new(device, queue, buf);

    pcb_builder
        .bind_pipeline_compute(resources.pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            resources.pipeline.layout().clone(),
            0,
            resources.ds,
        )
        .dispatch([1024 / 16, 1024 / 16, 1]).unwrap()
        .copy_image_to_buffer(
            CopyImageToBufferInfo::image_buffer(resources.image, resources.buffer)
        ).unwrap();
}

pub fn build(graph: &mut PassGraph, buf: &Subbuffer<[u8]>) {
    
    graph.add_pass(
        |device, queue, pcb_builder| {
        
        let resources = ResourcesTest::new(device, queue, buf);
        
        pcb_builder
            .bind_pipeline_compute(resources.pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                resources.pipeline.layout().clone(),
                0,
                resources.ds,
            )
            .dispatch([1024 / 16, 1024 / 16, 1]).unwrap()
            .copy_image_to_buffer(
                CopyImageToBufferInfo::image_buffer(resources.image, resources.buffer)
            ).unwrap();
        }
    );
}



