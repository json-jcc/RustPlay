
use std::sync::Arc;
use image::{ImageBuffer, Rgba};
use vulkano::{
    memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryUsage}, 
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}, pipeline::graphics::viewport::Viewport};

use vulkano::swapchain::{
    self, AcquireError, Surface, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
    SwapchainPresentInfo,
};
use vulkano::sync::future::FenceSignalFuture;
use vulkano::sync::{
    self, FlushError, GpuFuture
};
use vulkano_win::VkSurfaceBuild;

use super::{graph::{PassGraph, self}, pipelines::cp_test};

pub trait WorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph);

}

pub struct MobileWorldRenderer {

    fractal_buf: Option<Subbuffer<[u8]>>,
    recreate_swapchain: bool,
    fences: Vec<Option<Arc<FenceSignalFuture<_>>>>,
    pre_fence_idx: i32,
    viewport: Viewport

}

impl WorldRenderer for MobileWorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph) {
        self.render_fractal(graph);
        self.render_scene(graph);   
    }

}

impl MobileWorldRenderer {
    
    pub fn new(viewport: Viewport) -> Self {
        Self {
            fractal_buf: None,
            recreate_swapchain: false,
            fences: Vec::new(),
            pre_fence_idx: 0,
            viewport
        }
    }

}

impl MobileWorldRenderer {
    

}

impl MobileWorldRenderer {
    
    fn render_fractal(&mut self, graph: &mut PassGraph) {
        let mem_alloc 
            = StandardMemoryAllocator::new_default(graph.device());
        
        self.fractal_buf = Some(Buffer::from_iter(
            &mem_alloc,
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_DST,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Download,
                ..Default::default()
            },
            (0..1024 * 1024 * 4).map(|_| 0u8),
        ).expect("failed to create buffer"));

        let buf = self.fractal_buf.clone().unwrap();
        cp_test::build(graph, buf.clone());
    }

    fn render_scene(&mut self, graph: &mut PassGraph) {
        let (img_idx, suboptimal, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        // wait for the fence related to this image to finish (normally this would be the oldest fence)
        if let Some(image_fence) = &self.fences[img_idx as usize] {
            image_fence.wait(None).unwrap();
        }

        let previous_future = match self.fences[self.pre_fence_idx as usize].clone() {
            // Create a NowFuture
            None => {
                let mut now = sync::now(graph.device().clone());
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

        self.fences[img_idx as usize] = match future {
            Ok(value) => Some(Arc::new(value)),
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                None
            }
            Err(e) => {
                println!("failed to flush future: {e}");
                None
            }
    };

    self.pre_fence_idx = img_idx;
    }

    pub fn handle_fractal(&self) {
        let buffer_content = self.fractal_buf.as_ref().unwrap().read().unwrap();
        let image = 
        ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
        image.save("image.png").unwrap();
    }

}




pub struct DeferredShadingWorldRenderer {

}

impl DeferredShadingWorldRenderer {
    
    pub fn new() -> Self {
        Self {
        }
    }

}

impl WorldRenderer for DeferredShadingWorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph) {

    }

}