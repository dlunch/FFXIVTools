use zerocopy::AsBytes;

use crate::{Material, Mesh};

pub struct Model {
    pub(crate) mesh: Mesh,
    pub(crate) bind_group: wgpu::BindGroup,
    pub(crate) pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(device: &wgpu::Device, mesh: Mesh, material: Material) -> Self {
        let mx_total = Self::generate_matrix(1024.0 / 768.0);
        let mx_ref = mx_total.as_slice();
        let uniform_buf = device.create_buffer_with_data(mx_ref.as_bytes(), wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material.bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..64,
                    },
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&material.texture.texture.create_default_view()),
                },
                wgpu::Binding {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&material.texture.sampler),
                },
            ],
            label: None,
        });

        let state_descriptor = wgpu::VertexStateDescriptor {
            index_format: mesh.index_format(),
            vertex_buffers: &[mesh.buffer_descriptor()],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &material.pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &material.vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &material.fs_module,
                entry_point: "main",
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
            vertex_state: state_descriptor,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self { mesh, bind_group, pipeline }
    }

    fn generate_matrix(aspect_ratio: f32) -> nalgebra::Matrix4<f32> {
        use std::f32::consts::PI;

        // nalgebra's perspective uses [-1, 1] NDC z range, so convert it to [0, 1].
        #[rustfmt::skip]
        let correction: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        let projection = nalgebra::Matrix4::new_perspective(aspect_ratio, 45.0 * PI / 180.0, 1.0, 10.0);
        let view = nalgebra::Matrix4::look_at_rh(
            &nalgebra::Point3::new(1.5f32, -5.0, 3.0),
            &nalgebra::Point3::new(0.0, 0.0, 0.0),
            &nalgebra::Vector3::z_axis(),
        );

        correction * projection * view
    }
}
