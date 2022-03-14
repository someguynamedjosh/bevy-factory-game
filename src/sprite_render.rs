use bevy::{
    asset::{Assets, HandleUntyped},
    ecs::Resources,
    prelude::*,
    render::{
        pipeline::{
            BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite,
            CompareFunction, CullMode, DepthStencilStateDescriptor, FrontFace, PipelineDescriptor,
            RasterizationStateDescriptor, StencilStateDescriptor, StencilStateFaceDescriptor,
        },
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
    sprite::SPRITE_PIPELINE_HANDLE,
};

pub const TRANSPARENT_SPRITE_PIPELINE_HANDLE: HandleUntyped = SPRITE_PIPELINE_HANDLE;
// HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID,
// 0x37215d3f806dfa3a);

pub fn build_transparent_sprite_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::default(),
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("vert.glsl"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("frag.glsl"),
            ))),
        })
    }
}

pub mod node {
    pub const COLOR_MATERIAL: &str = "color_material2";
    pub const SPRITE: &str = "sprite2";
}

fn setup_render_graph(graph: &mut RenderGraph, resources: &Resources) {
    graph.add_system_node(
        node::COLOR_MATERIAL,
        AssetRenderResourcesNode::<ColorMaterial>::new(false),
    );
    graph
        .add_node_edge(node::COLOR_MATERIAL, base::node::MAIN_PASS)
        .unwrap();

    graph.add_system_node(node::SPRITE, RenderResourcesNode::<Sprite>::new(true));
    graph
        .add_node_edge(node::SPRITE, base::node::MAIN_PASS)
        .unwrap();

    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    pipelines.set_untracked(
        TRANSPARENT_SPRITE_PIPELINE_HANDLE,
        build_transparent_sprite_pipeline(&mut shaders),
    );
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut AppBuilder) {
        let resources = app.resources_mut();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        setup_render_graph(&mut *render_graph, resources);
    }
}
