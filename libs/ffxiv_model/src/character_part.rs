use alloc::{string::String, sync::Arc, vec::Vec};

use hashbrown::{HashMap, HashSet};
use log::debug;
use nalgebra::Matrix4;
use zerocopy::AsBytes;

use ffxiv_parser::BufferItemType;
use renderer::{Mesh, Model, RenderContext, Renderable, Renderer, VertexFormat, VertexFormatItem, VertexItemType};

use crate::context::Context;
use crate::material::create_material;
use crate::model_reader::ModelData;

pub struct CharacterPart {
    models: Vec<Model>,
}

impl CharacterPart {
    pub async fn new(renderer: &Renderer, model_data: ModelData, bone_transforms: &HashMap<String, Matrix4<f32>>, context: &Context) -> Self {
        let mdl = model_data.mdl;

        let visibility_mask = 0;
        let hidden_attributes = HashSet::new();
        let lod = 0;
        let meshes = mdl.meshes(lod);
        let buffer_items = mdl.buffer_items(lod);

        let mut models = Vec::new();
        for (mesh_index, (mesh_data, buffer_item)) in meshes.zip(buffer_items).enumerate() {
            let vertex_formats = (0..mesh_data.mesh_info.buffer_count as usize)
                .map(|buffer_index| {
                    let buffer_items = buffer_item.items().filter(move |x| x.buffer as usize == buffer_index);
                    VertexFormat::new(
                        buffer_items
                            .map(|x| VertexFormatItem::new(x.usage.as_str(), Self::convert_buffer_type(x.item_type), x.offset as usize))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            let strides = (0..mesh_data.mesh_info.buffer_count as usize)
                .map(|i| mesh_data.mesh_info.strides[i] as usize)
                .collect::<Vec<_>>();

            let mesh_parts = mdl.parts()
                [mesh_data.mesh_info.part_offset as usize..mesh_data.mesh_info.part_offset as usize + mesh_data.mesh_info.part_count as usize]
                .iter()
                .filter_map(|mesh_part| {
                    debug!("part attributes {:?} mask {}", mesh_part.attributes, mesh_part.visibility_mask);

                    if mesh_part.visibility_mask & visibility_mask != mesh_part.visibility_mask
                        || mesh_part.attributes.intersection(&hidden_attributes).next().is_some()
                    {
                        None
                    } else {
                        let begin = mesh_part.index_range.start - mesh_data.mesh_info.index_offset;
                        let end = mesh_part.index_range.end - mesh_data.mesh_info.index_offset;
                        Some(begin..end)
                    }
                })
                .collect::<Vec<_>>();

            let mesh = Mesh::new(&renderer, mesh_data.buffers.as_ref(), &strides, mesh_data.indices, vertex_formats).await;

            let (mtrl, texs) = &model_data.mtrls[mesh_index];

            let bone_names = mdl.bone_names(mesh_data.mesh_info.bone_index).collect::<Vec<_>>();
            let mut bone_transform_data = Vec::<u8>::with_capacity(bone_names.len() * 4 * 3 * core::mem::size_of::<f32>());
            for bone_name in bone_names {
                if let Some(x) = bone_transforms.get(bone_name) {
                    bone_transform_data.extend(x.as_slice()[..12].as_bytes());
                } else {
                    let identity = [1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.];
                    bone_transform_data.extend(identity.as_bytes());
                }
            }

            let bone_transform = Arc::new(renderer.buffer_pool.alloc(bone_transform_data.len()));
            bone_transform.write(&bone_transform_data).await.unwrap();

            let material = create_material(renderer, context, mtrl, texs, bone_transform).await;

            models.push(Model::new(&renderer, mesh, material, mesh_parts));
        }

        Self { models }
    }

    fn convert_buffer_type(item_type: BufferItemType) -> VertexItemType {
        match item_type {
            BufferItemType::UByte4 => VertexItemType::UByte4,
            BufferItemType::UByte4n => VertexItemType::UByte4,
            BufferItemType::Float2 => VertexItemType::Float2,
            BufferItemType::Float3 => VertexItemType::Float3,
            BufferItemType::Float4 => VertexItemType::Float4,
            BufferItemType::Half2 => VertexItemType::Half2,
            BufferItemType::Half4 => VertexItemType::Half4,
            _ => panic!(),
        }
    }
}

impl Renderable for CharacterPart {
    fn render<'a>(&'a self, mut render_context: &mut RenderContext<'a>) {
        for model in &self.models {
            model.render(&mut render_context);
        }
    }
}
