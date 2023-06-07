

use std::sync::Arc;
use vulkano::sync::{self, GpuFuture};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::device::{Device, Queue};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, CommandBufferUsage,
};


pub struct Pass {

    f: fn(&Arc<Device>, &Arc<Queue>, &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) -> (),
}

pub struct PassGraph {

    device: Arc<Device>,
    queue: Arc<Queue>,

    passes: Vec<Pass>,
    pcb: Option<Arc<PrimaryAutoCommandBuffer>>
    
}

impl PassGraph {
    pub fn new(device: &Arc<Device>, queue: &Arc<Queue>) -> Self {
        Self {
            device: device.clone(),
            queue: queue.clone(),
            passes : Vec::new(),
            pcb: None,
        }
    }

    pub fn setup(&mut self) {
        let cb_allocator = StandardCommandBufferAllocator::new(self.device.clone(), Default::default());
        let mut cb_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> = AutoCommandBufferBuilder::primary(
            &cb_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();
        
        for pass in self.passes.iter_mut() {
            pass.f.clone()(&self.device, &self.queue, &mut cb_builder);
        }

        self.pcb = Some(Arc::new(cb_builder.build().unwrap()));
    }

    pub fn compile(&mut self) {

    }

    pub fn execute(&mut self) {
        
        let cp_test_future = sync::now(self.device.clone())
        .then_execute(self.queue.clone(), self.pcb.unwrap().clone()).unwrap()
        .then_signal_fence_and_flush().unwrap();

        cp_test_future.wait(None).unwrap()
        
    }

    pub fn add_pass<F>(&mut self, f: F)
    where F:  FnMut(&Arc<Device>, &Arc<Queue>, &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) {
        self.passes.push(Pass {
            f,
        });
    }
}