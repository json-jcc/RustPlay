

use std::sync::Arc;

use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, Queue};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryUsage};
use vulkano::buffer::BufferContents;
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo};
use vulkano::sync::{self, GpuFuture};
use vulkano::pipeline::{ComputePipeline, GraphicsPipeline};
use vulkano::pipeline::Pipeline;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::pipeline::PipelineBindPoint;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::format::Format;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};

#[derive(BufferContents)]
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u32,
}

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src: "
            #version 450

            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } data;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                data.data[idx] *= 12;
            }
        "
    }
}


fn test_buffer(device: Arc<Device>, queue: Arc<Queue>, queue_family_index: u32) {
    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let source_content : Vec<i32> = (0..64).collect();
    let source = Buffer::from_iter(
        &memory_allocator, 
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        }, 
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        }, 
        source_content
    ).unwrap();

    let destination_content : Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = Buffer::from_iter(
        &memory_allocator, 
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        }, 
        AllocationCreateInfo {
            usage: MemoryUsage::Download,
            ..Default::default()
        }, 
        destination_content
    ).unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut primary_command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();
    
    primary_command_buffer_builder.copy_buffer(
        CopyBufferInfo::buffers(source.clone(), destination.clone())
    ).unwrap();
    
    let command_buffer = primary_command_buffer_builder.build().unwrap();

    sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer).unwrap()
        .then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();
    assert_eq!(&*src_content, &*destination_content);
}

fn test_compute_pipeline(device: Arc<Device>, queue: Arc<Queue>) {
    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let data = MyStruct { a: 5, b: 69 };
    let buffer = Buffer::from_data(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        data,
    ).unwrap();
    
    let shader = cs::load(device.clone()).unwrap();
    let compute_pipeline = ComputePipeline::new(
        device.clone(), 
        shader.entry_point("main").unwrap(), 
        &(), 
        None, 
        |_|{}
    ).unwrap();
    
    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = compute_pipeline
        .layout()
        .set_layouts()
        .get(descriptor_set_layout_index)
        .unwrap();

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [
            WriteDescriptorSet::buffer(0, buffer.clone())
            ], // 0 is the binding
    )
    .unwrap();

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );
    
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();
    
    let work_group_counts = [1024, 1, 1];
    
    command_buffer_builder.bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            descriptor_set_layout_index as u32,
            descriptor_set,
        )
        .dispatch(work_group_counts).unwrap();
    
    // build the command buffer with written commands
    let command_buffer = command_buffer_builder.build().unwrap();

    // flush the command buffer and wait for the GPU to finish
    sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer).unwrap()
    .then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();

}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

fn test_graphics_pipeline(device: Arc<Device>, queue: Arc<Queue>) {
    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    let vertex1 = MyVertex { position: [-0.5, -0.5] };
    let vertex2 = MyVertex { position: [ 0.0,  0.5] };
    let vertex3 = MyVertex { position: [ 0.5, -0.25] };

    let vertex_buffer = Buffer::from_iter(
        &memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vec![vertex1, vertex2, vertex3],
    ).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: Format::R8G8B8A8_UNORM,
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    ).unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();
    let framebuffer = Framebuffer::new(
    render_pass.clone(),
    FramebufferCreateInfo {
        attachments: vec![view],
        ..Default::default()
    },
)
.unwrap();



}

fn main() {

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    
    let instance = Instance::new(
        library, 
        InstanceCreateInfo::default()
    ).expect("failed to create instance");
    
    let physical_device = instance
        .enumerate_physical_devices().expect("could not enumerate devices")
        .next().expect("no devices available");

    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s)", family.queue_count);
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        }).expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).expect("failed to create device");
    
    let queue = queues.next().unwrap();

    test_buffer(device.clone(), queue.clone(), queue_family_index);
    
    test_compute_pipeline(device.clone(), queue.clone());

}
