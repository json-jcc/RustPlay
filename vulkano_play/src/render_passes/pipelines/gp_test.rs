

use std::sync::Arc;
use vulkano::{
    device::{
        Device
    },
    render_pass::{
        RenderPass,
        Subpass,
    },
    pipeline::{
        graphics::{
            viewport::{
                Viewport,
                ViewportState
            },
            discard_rectangle::DiscardRectangleState,
            input_assembly::{
                InputAssemblyState
            }, 
            multisample::MultisampleState,
            rasterization::{ 
                RasterizationState,
            },
            depth_stencil::DepthStencilState, color_blend::ColorBlendState,
        }, 
        GraphicsPipeline
    }, image::SampleCount, buffer::Subbuffer
};

use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::buffer::{BufferContents};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path : "shaders/test/static_triangle.vs"
    }
}

// mod tesc {
//     vulkano_shaders::shader! {
//         ty: "tess_ctrl",
//         path: "shaders/test/static_triangle.tesc"
//     }
// }

// mod tese {
//     vulkano_shaders::shader! {
//         ty: "tess_eval",
//         path: "shaders/test/static_triangle.tese"
//     }
// }

// mod gs {
//     vulkano_shaders::shader! {
//         ty: "geometry",
//         path: "shaders/test/static_triangle.gs"
//     }
// }

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/test/static_triangle.fs"
    }
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vert {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

pub struct Resources {

    vbo: Subbuffer<[Vert]>,
    ebo: Subbuffer<[u32]>,

    
}

impl Resources {
    
}

pub fn create(
    device: &Arc<Device>,
    render_pass: Arc<RenderPass>,
    subpass_idx: u32,
    viewport: Viewport,
) -> Arc<GraphicsPipeline> {

    let vs = vs::load(device.clone()).expect("failed to create shader module");
    //let tesc = tesc::load(device.clone()).expect("failed to create shader module");
    //let tese = tese::load(device.clone()).expect("failed to create shader module");
    //let gs = gs::load(device.clone()).expect("failed to create shader module");
    let fs = fs::load(device.clone()).expect("failed to create shader module");

    GraphicsPipeline::start()
        .render_pass(Subpass::from(render_pass, subpass_idx).unwrap())
        
        .vertex_input_state(Vert::per_vertex())
        .input_assembly_state(
            InputAssemblyState::default()
        )
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        
        // .tessellation_state(
        //     TessellationState::default()
        // )
        //.tessellation_shaders(tesc.entry_point("main").unwrap(), (), tese.entry_point("main").unwrap(), ())
        //.geometry_shader(gs.entry_point("main").unwrap(), ())
        
        .viewport_state(
            ViewportState::viewport_fixed_scissor_irrelevant(
                [viewport]
            )
        )
        .discard_rectangle_state(
            DiscardRectangleState::default()
        )
        .rasterization_state(
            RasterizationState::default()
        )        
        
        .multisample_state(
            MultisampleState { 
                rasterization_samples: SampleCount::Sample1,
                ..Default::default()
            }
        )
        .depth_stencil_state(
            DepthStencilState::simple_depth_test()
        )
        .color_blend_state(
            ColorBlendState::default()
        )
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        
        .build(device.clone()).unwrap()
}
