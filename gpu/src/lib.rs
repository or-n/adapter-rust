mod pipeline;
mod uniform;
mod vertex;

use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};

use wgpu::*;
use wgpu::util::DeviceExt;

use window::api::*;

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    window: Window, // after surface!
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    time_buffer: Buffer,
    bind_group: BindGroup,
    time: f32,
}

impl State {
    async fn adapter(instance: &Instance, surface: &Surface) -> Adapter {
        let options = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        instance.request_adapter(&options).await.unwrap()
    }

    async fn device_and_queue(adapter: &Adapter) -> (Device, Queue) {
        let descriptor = DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            label: None,
        };
        adapter.request_device(&descriptor, None).await.unwrap()
    }

    fn config(surface_caps: &SurfaceCapabilities, size: PhysicalSize<u32>)
    -> SurfaceConfiguration {
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        }
    }

    pub async fn new(window: Window) -> Self {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = Self::adapter(&instance, &surface).await;
        let (device, queue) = Self::device_and_queue(&adapter).await;
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::config(&surface_caps, window.inner_size());
        surface.configure(&device, &config);
        let time_layout = uniform::layout(&device);

        let vertex_buffer = device.create_buffer_init(
            &util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertex::VERTICES),
                usage: BufferUsages::VERTEX,
            }
        );

        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<vertex::Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &vertex::ATTRIBS,
        };

        let time_buffer = device.create_buffer_init(&uniform::time_buffer());
        let bind_group = uniform::bind_group(&time_layout, &device, &time_buffer);
        let render_pipeline = pipeline::new(time_layout, vertex_layout, &device, &config);
        let time = 0.0;
        Self { window, surface, device, queue, config, render_pipeline,
            vertex_buffer, time_buffer, bind_group, time
        }
    }
}

impl API for State {
    fn new(window: Window) -> Self {
        pollster::block_on(Self::new(window))
    }

    fn window(&self) -> &Window {
        &self.window
    }

    fn size(&self) -> PhysicalSize<u32> {
        PhysicalSize { width: self.config.width, height: self.config.height }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.time += 0.01;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[self.time]));
    }
    
    fn render(&mut self) -> Result<(), ()> {
        let output = self.surface.get_current_texture().unwrap();
        let view = TextureViewDescriptor::default();
        let view = output.texture.create_view(&view);
        let encoder = CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&encoder);
        {
            let color = Color::BLACK;// { r: 0.0, g: 0.1, b: 0.0, a: 1.0 };
            let attachment = RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations { load: LoadOp::Clear(color), store: true },
            };
            let render_pass = RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(attachment)],
                depth_stencil_attachment: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}