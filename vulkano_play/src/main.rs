
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryUsage};
use vulkano::buffer::BufferContents;

#[derive(BufferContents)]
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u32,
}

fn main() {

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(
        library, 
        InstanceCreateInfo::default())
        .expect("failed to create instance");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    for family in physical_device.queue_family_properties() {
        println!("Found a queue family with {:?} queue(s)", family.queue_count);
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

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
    )
    .expect("failed to create device");
    
    let queue = queues.next().unwrap();

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

    // let data: i32 = 12;
    // let buffer = Buffer::from_data(
    //     &memory_allocator,
    //     BufferCreateInfo {
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         usage: MemoryUsage::Upload,
    //         ..Default::default()
    //     },
    //     data,
    // )
    // .expect("failed to create buffer");

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
    )
    .unwrap();


    let mut content = buffer.write().unwrap();
    content.a *= 2;
    content.b = 9;


}
