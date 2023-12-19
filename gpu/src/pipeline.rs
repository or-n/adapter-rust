use wgpu::*;

pub fn new(
    time_layout: BindGroupLayout,
    vertex_layout: VertexBufferLayout,
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration
) -> wgpu::RenderPipeline {
    let code = wgpu::include_wgsl!("shader.wgsl");
    let shader = device.create_shader_module(code);
    let layout = &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&time_layout],
        push_constant_ranges: &[],
    };
    let render_pipeline_layout = device.create_pipeline_layout(layout);
    let targets = &[Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    })];
    let pipeline = wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_layout],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets,
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    };
    device.create_render_pipeline(&pipeline)
}