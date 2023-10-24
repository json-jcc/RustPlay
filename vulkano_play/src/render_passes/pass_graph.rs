use std::sync::Arc;
use vulkano::sync::{self, GpuFuture};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::device::{Queue, DeviceOwned, Device};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, CommandBufferUsage,
};


type FnMutPass = dyn FnMut(&Arc<Queue>, &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>);

pub struct Pass
{
    f: Box<FnMutPass>,
}

pub struct PassPrerequisites {

}

pub struct PassGraph {

    queue: Arc<Queue>,
    cb_allocator: StandardCommandBufferAllocator,
    passes: Vec<Pass>,
}

impl PassGraph {
    pub fn new(queue: &Arc<Queue>) -> Self {
        
        let cb_allocator = StandardCommandBufferAllocator::new(queue.device().clone(), Default::default());

        Self {
            queue: queue.clone(),
            cb_allocator,
            passes : Vec::new(),
        }
    }

    pub fn device(&self) -> Arc<Device> {
        self.queue.device().clone()
    }

    pub fn add_pass(&mut self, f: Box<FnMutPass>) {
        self.passes.push(Pass {
            f
        });
    }

    pub fn compile(&mut self) {
        
    }

    pub fn execute(&mut self) {

        let mut pcb_builder = AutoCommandBufferBuilder::primary(
            &self.cb_allocator, 
            self.queue.queue_family_index(), 
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();

        for pass in self.passes.iter_mut() {
            (pass.f)(&self.queue, &mut pcb_builder);
        }

        let pcb = Arc::new(pcb_builder.build().unwrap());

        let future = sync::now(self.queue.device().clone())
            .then_execute(self.queue.clone(), pcb.clone()).unwrap()
            .then_signal_fence_and_flush().unwrap();

        future.wait(None).unwrap();
    }
}