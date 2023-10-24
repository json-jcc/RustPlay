
use std::sync::Arc;
use image::{
    ImageBuffer, Rgba
};
use vulkano::{
    memory::allocator::{
        StandardMemoryAllocator, 
        AllocationCreateInfo, 
        MemoryUsage
    }, 
    buffer::{
        Buffer, 
        BufferCreateInfo, 
        BufferUsage, 
        Subbuffer
    }, 
    pipeline::{
        graphics::viewport::Viewport, 
        PipelineBindPoint, 
        Pipeline
    }, 
    command_buffer::CopyImageToBufferInfo, swapchain::Swapchain
};

use vulkano::swapchain::{
    self, AcquireError,
    SwapchainPresentInfo,
};
use vulkano::sync::{
    self, FlushError, GpuFuture
};

use super::{pass_graph::{PassGraph, self}, pipelines::cp_test, rp_test};

pub trait WorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph);

}

pub struct MobileWorldRenderer {

    fractal_buf: Option<Subbuffer<[u8]>>,
    recreate_swapchain: bool,
    //fences: Vec<Option<Arc<FenceSignalFuture<_>>>>,
    pre_fence_idx: i32,
    viewport: Viewport,
    render_fractal_this_frame: bool,
    swapchain: Arc<Swapchain>,

}

impl WorldRenderer for MobileWorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph) {

        self.render_scene(graph);   
        
        if self.render_fractal_this_frame
        {
            self.render_fractal(graph);
        }        
    }
}

impl MobileWorldRenderer {
    
    pub fn new(viewport: Viewport) -> Self {
        Self {
            fractal_buf: None,
            recreate_swapchain: false,
            //fences: Vec::new(),
            pre_fence_idx: 0,
            viewport,
            render_fractal_this_frame: true,
        }
    }

    pub fn switch(&mut self) {
        
    }
}

impl MobileWorldRenderer {

}

impl MobileWorldRenderer {

    fn render_scene(&mut self, graph: &mut PassGraph) {
        
        graph.add_pass(Box::new(move |queue, pcb_builder| {
            
            let (img_idx, suboptimal, acquire_future) = 
                match swapchain::acquire_next_image(self.swapchain.clone(), None) {
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
        }));
    }

    fn render_fractal(&mut self, graph: &mut PassGraph) {
        
        // create gpu resources
        let mem_alloc 
            = StandardMemoryAllocator::new_default(graph.device());
        
        let raw_buf = (0..1024 * 1024 * 4).map(|_| 0u8);

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
        
        let resources = cp_test::ResourcesTest::new(queue.device(), queue, buf.clone());

        // create dispatch lambda
        graph.add_pass(Box::new(
            move |queue, pcb_builder| {

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
        ));

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