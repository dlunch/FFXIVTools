use alloc::vec::Vec;

use crate::{Material, Mesh, MeshPart, RenderContext, Renderable, Renderer};

pub struct Model {
    mesh: Mesh,
    material: Material,
    mesh_parts: Vec<MeshPart>,

    pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(renderer: &Renderer, mesh: Mesh, material: Material, mesh_parts: Vec<MeshPart>) -> Self {
        let attributes = mesh
            .vertex_formats
            .iter()
            .map(|x| x.wgpu_attributes(&material.vertex_shader.inputs))
            .collect::<Vec<_>>();

        let vertex_buffers = attributes
            .iter()
            .zip(mesh.strides.iter())
            .map(|(attributes, stride)| wgpu::VertexBufferDescriptor {
                stride: *stride as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes,
            })
            .collect::<Vec<_>>();

        let pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                format: wgpu::TextureFormat::Bgra8Unorm,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_read_mask: 0,
                stencil_write_mask: 0,
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &vertex_buffers,
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            mesh,
            material,
            pipeline,
            mesh_parts,
        }
    }
}

impl Renderable for Model {
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>) {
        render_context.render_pass.set_pipeline(&self.pipeline);
        render_context.render_pass.set_bind_group(0, &self.material.bind_group, &[]);
        render_context.render_pass.set_index_buffer(
            &self.mesh.index_buffer.buffer,
            self.mesh.index_buffer.offset as u64,
            self.mesh.index_buffer.size as u64,
        );
        for (i, vertex_buffer) in self.mesh.vertex_buffers.iter().enumerate() {
            render_context
                .render_pass
                .set_vertex_buffer(i as u32, &vertex_buffer.buffer, vertex_buffer.offset as u64, vertex_buffer.size as u64);
        }

        let mut last_begin = self.mesh_parts[0].begin;
        let mut last_end = self.mesh_parts[0].begin;
        for mesh_part in &self.mesh_parts {
            if last_end != mesh_part.begin {
                render_context.render_pass.draw_indexed(last_begin..last_end, 0, 0..1);
                last_begin = mesh_part.begin;
            }
            last_end = mesh_part.begin + mesh_part.count;
        }
        render_context.render_pass.draw_indexed(last_begin..last_end, 0, 0..1);
    }
}
