
use std::sync::Arc;
use std::any::Any;
use vulkano::VulkanLibrary;
use vulkano::buffer::{
    Buffer, BufferCreateInfo, BufferUsage
};
use vulkano::device::physical::{
    PhysicalDevice, PhysicalDeviceType
};
use vulkano::device::{
    Device, DeviceCreateInfo, DeviceExtensions, 
    QueueCreateInfo, QueueFlags, 
    Features,
};
use vulkano::image::{ImageUsage};
use vulkano::instance::{
    Instance, InstanceCreateInfo
};
use vulkano::memory::allocator::{
    AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator
};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::swapchain::{
    self, AcquireError, Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
    SwapchainPresentInfo,
};
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::{
    self, FlushError, GpuFuture
};
use vulkano_win::VkSurfaceBuild;

use winit::event::{
    Event, WindowEvent
};
use winit::event_loop::{
    ControlFlow, EventLoop
};
use winit::window::{
    Window, WindowBuilder
};

use crate::render_passes::graph::PassGraph;
use crate::render_passes::renderer_world::{
    WorldRenderer,
    MobileWorldRenderer
};
use crate::render_passes::{
    rp_test,
    pipelines::{
        gp_test,
    } 
};

use image::{ImageBuffer, Rgba};

pub fn select_physical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> (Arc<PhysicalDevice>, u32) {
    instance
        .enumerate_physical_devices().expect("failed to enumerate physical devices")
        .filter(|p| p.supported_extensions().contains(device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            _ => 4,
        }).expect("no device available")
}

pub fn test_vulkano() {

    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let required_instance_extensions = vulkano_win::required_extensions(&library);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_instance_extensions,
            ..Default::default()
        },
    ).expect("failed to create instance");

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let window = surface
        .object().unwrap()
        .clone()
        .downcast::<Window>().unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);

    let features = Features {
        tessellation_shader : true,
        geometry_shader : true,
        ray_tracing_pipeline : true,
        ..Default::default()
    };

    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![
                QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }
            ],
            enabled_extensions: device_extensions,
            enabled_features : features,
            ..Default::default()
        },
    ).expect("failed to create device");

    let (mut swapchain, images) = {
        let caps = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("failed to get surface capabilities");

        let dimensions = window.inner_size();
        let composite_alpha = caps.supported_composite_alpha
            .into_iter()
            .next()
            .unwrap();
        let image_format = Some(
            physical_device
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );

        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format,
                image_extent: dimensions.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap()
    };

    let queue = queues.next().unwrap();

    let mut graph = PassGraph::new(&queue);
    // let mut world_renderer = MobileWorldRenderer::new();
    let mut wrdr: Box<dyn WorldRenderer> = Box::new(MobileWorldRenderer::new());
    wrdr.render(&mut graph);
    graph.execute();

    //world_renderer.handle_fractal();
    
    let mem_alloc = StandardMemoryAllocator::new_default(device.clone());
    
    let vertices = vec![
        gp_test::Vert {position: [-0.5, -0.5]}, 
        gp_test::Vert {position: [0.0, 0.5]}, 
        gp_test::Vert {position: [0.5, -0.25]}
    ];

    let vbo = Buffer::from_iter(
        &mem_alloc,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vertices.into_iter(),
    ).unwrap();

    let ebo = Buffer::from_iter(
        &mem_alloc,
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vec![0u32, 1, 2].into_iter(),
    ).unwrap();

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: window.inner_size().into(),
        depth_range: 0.0..1.0,
    };

    let rp_test = Arc::new(rp_test::RenderPassTest::new(
        &device, 
        &swapchain, 
        &viewport
    ));

    let mut cbs = rp_test.create_command_buffers(
        &queue, 
        &vbo, &ebo, 
        &images
    );

    let mut window_resized = false;
    let mut recreate_swapchain = false;

    let frames_in_flight = images.len();
    let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
    let mut pre_fence_idx = 0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            window_resized = true;
        }
        Event::MainEventsCleared => {
            if window_resized || recreate_swapchain {
                recreate_swapchain = false;

                let new_dimensions = window.inner_size();

                let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                    image_extent: new_dimensions.into(),
                    ..swapchain.create_info()
                }) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                    Err(e) => panic!("failed to recreate swapchain: {e}"),
                };
                swapchain = new_swapchain;

                if window_resized {
                    window_resized = false;

                    viewport = Viewport {
                        origin: [0.0, 0.0],
                        dimensions: window.inner_size().into(),
                        depth_range: 0.0..1.0,
                    };

                    cbs = rp_test.create_command_buffers(
                        &queue, 
                        &vbo, 
                        &ebo, 
                        &new_images);
                }
            }

            

            let (img_idx, suboptimal, acquire_future) =
                match swapchain::acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            // wait for the fence related to this image to finish (normally this would be the oldest fence)
            if let Some(image_fence) = &fences[img_idx as usize] {
                image_fence.wait(None).unwrap();
            }

            let previous_future = match fences[pre_fence_idx as usize].clone() {
                // Create a NowFuture
                None => {
                    let mut now = sync::now(device.clone());
                    now.cleanup_finished();

                    now.boxed()
                }
                // Use the existing FenceSignalFuture
                Some(fence) => fence.boxed(),
            };

            let future = previous_future
                .join(acquire_future)
                .then_execute(
                    queue.clone(), 
                    cbs[img_idx as usize].clone()
                    //cbs[0].clone()
                ).unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), img_idx),
                )
                .then_signal_fence_and_flush();

            fences[img_idx as usize] = match future {
                Ok(value) => Some(Arc::new(value)),
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    None
                }
                Err(e) => {
                    println!("failed to flush future: {e}");
                    None
                }
            };

            pre_fence_idx = img_idx;
        }
        _ => (),
    });
}