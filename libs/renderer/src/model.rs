use nalgebra::Matrix4;

use crate::{Material, Mesh};

pub struct Model {
    mesh: Mesh,
    material: Material,

    pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(device: &wgpu::Device, mesh: Mesh, material: Material) -> Self {
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &material.pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &material.vertex_shader.module,
                entry_point: material.vertex_shader.entry,
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &material.fragment_shader.module,
                entry_point: material.fragment_shader.entry,
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: mesh.index_format(),
                vertex_buffers: &mesh.vertex_descriptors(),
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self { mesh, material, pipeline }
    }

    pub(crate) fn set_mvp(&mut self, device: &wgpu::Device, mvp: Matrix4<f32>) {
        self.material.set_mvp(device, mvp)
    }

    pub(crate) fn render(&self, command_encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainOutput) {
        let mut rpass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.material.bind_group, &[]);
        rpass.set_index_buffer(&self.mesh.index, 0, 0);
        for (i, vertex_buffer) in self.mesh.vertex_buffers.iter().enumerate() {
            rpass.set_vertex_buffer(i as u32, &vertex_buffer, 0, 0);
        }
        rpass.draw_indexed(0..self.mesh.index_count as u32, 0, 0..1);
    }
}
