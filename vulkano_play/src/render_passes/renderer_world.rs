

use image::{ImageBuffer, Rgba};
use vulkano::{
    memory::allocator::{StandardMemoryAllocator, AllocationCreateInfo, MemoryUsage}, 
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}};

use super::{graph::{PassGraph, self}, pipelines::cp_test};

pub trait WorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph);

}

pub struct MobileWorldRenderer {

    fractal_buf: Option<Subbuffer<[u8]>>,

}

impl WorldRenderer for MobileWorldRenderer {
    
    fn render(&mut self, graph: &mut PassGraph) {
        self.update_primitvies(graph);
        self.fractal(graph);
    }

}

impl MobileWorldRenderer {
    
    pub fn new() -> Self {
        Self {
            fractal_buf: None,
        }
    }

}

impl MobileWorldRenderer {
    
    fn update_primitvies(&mut self, graph : &mut PassGraph) {

    }

}

impl MobileWorldRenderer {
    
    fn fractal(&mut self, graph: &mut PassGraph) {
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