use wgpu::*;

pub fn time_buffer() -> util::BufferInitDescriptor<'static> {
    util::BufferInitDescriptor {
        label: Some("Time Buffer"),
        contents: bytemuck::cast_slice(&[0.0]),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    }
}

pub fn layout(device: &Device) -> BindGroupLayout {
    let entries = &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }];
    let descriptor = BindGroupLayoutDescriptor {
        entries,
        label: Some("bind_group_layout"),
    };
    device.create_bind_group_layout(&descriptor)
}

pub fn bind_group(layout: &BindGroupLayout, device: &Device, time_buffer: &Buffer) -> BindGroup {
    let entries = &[
        BindGroupEntry {
            binding: 0,
            resource: time_buffer.as_entire_binding(),
        }
    ];
    device.create_bind_group(&BindGroupDescriptor {
        layout: &layout,
        entries,
        label: Some("bind_group"),
    })
}