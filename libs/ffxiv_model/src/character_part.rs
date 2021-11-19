use alloc::{string::String, sync::Arc, vec::Vec};
use core::ops::Range;

use hashbrown::{HashMap, HashSet};
use nalgebra::Matrix4;
use zerocopy::AsBytes;

use eng::render::{Buffer, Mesh, Model, RenderContext, Renderable, Renderer, VertexFormat, VertexFormatItem, VertexItemType};
use ffxiv_parser::{BufferItemChunk, BufferItemType, BufferItemUsage, Mdl, MdlMesh};

use crate::context::Context;
use crate::customization::Customization;
use crate::material::create_material;
use crate::model_reader::{EquipmentModelData, ModelData};

pub struct CharacterPart {
    models: Vec<(Model, Vec<Range<u32>>)>,
}

impl CharacterPart {
    pub fn with_model(
        renderer: &Renderer,
        model_data: ModelData,
        bone_transforms: &HashMap<String, Matrix4<f32>>,
        context: &Context,
        customization: &Customization,
    ) -> Self {
        let mdl = model_data.mdl;

        let visibility_mask = 0;
        let hidden_attributes = HashSet::new();
        let lod = 0;

        let mut models = Vec::with_capacity(mdl.mesh_count(lod));
        for ((mesh_data, buffer_item), (mtrl, texs)) in mdl.meshes(lod).zip(mdl.buffer_items(lod)).zip(model_data.mtrls) {
            let mesh = Self::load_mesh(renderer, &mesh_data, buffer_item);
            let mesh_parts = Self::get_mesh_parts(&mdl, &mesh_data, visibility_mask, &hidden_attributes);
            let bone_transform = Self::load_bone_transform(renderer, &mdl, &mesh_data, bone_transforms);

            let material = create_material(renderer, context, &mtrl, &texs, bone_transform, customization, 0);

            models.push((Model::new(renderer, mesh, material), mesh_parts));
        }

        Self { models }
    }

    pub fn with_equipment_model(
        renderer: &Renderer,
        equipment_model_data: EquipmentModelData,
        _bone_transforms: &HashMap<String, Matrix4<f32>>,
        context: &Context,
        customization: &Customization,
    ) -> Self {
        log::debug!(
            "original {:?} deformed {:?}",
            equipment_model_data.original_body_id as u16,
            equipment_model_data.deformed_body_id as u16
        );
        let prebone_deformer = context.get_body_deform_matrices(equipment_model_data.original_body_id, equipment_model_data.deformed_body_id);
        let mdl = equipment_model_data.model_data.mdl;

        let visibility_mask = 0;
        let hidden_attributes = HashSet::new();
        let lod = 0;

        let mut models = Vec::with_capacity(mdl.mesh_count(lod));
        for ((mesh_data, buffer_item), (mtrl, texs)) in mdl.meshes(lod).zip(mdl.buffer_items(lod)).zip(equipment_model_data.model_data.mtrls) {
            let mesh = Self::load_mesh(renderer, &mesh_data, buffer_item);
            let mesh_parts = Self::get_mesh_parts(&mdl, &mesh_data, visibility_mask, &hidden_attributes);
            let bone_transform = Self::load_bone_transform(renderer, &mdl, &mesh_data, &prebone_deformer);

            let material = create_material(
                renderer,
                context,
                &mtrl,
                &texs,
                bone_transform,
                customization,
                equipment_model_data.stain_id,
            );

            models.push((Model::new(renderer, mesh, material), mesh_parts));
        }

        Self { models }
    }

    fn load_mesh(renderer: &Renderer, mesh_data: &MdlMesh<'_>, buffer_item: &BufferItemChunk) -> Mesh {
        let vertex_formats = (0..mesh_data.mesh_info.buffer_count as usize)
            .map(|buffer_index| {
                let buffer_items = buffer_item.items().filter(move |x| x.buffer as usize == buffer_index);
                VertexFormat::new(
                    buffer_items
                        .map(|x| {
                            VertexFormatItem::new(
                                Self::buffer_usage_to_shader_name(&x.usage),
                                Self::convert_buffer_type(x.item_type),
                                x.offset as usize,
                            )
                        })
                        .collect::<Vec<_>>(),
                    mesh_data.mesh_info.strides[buffer_index] as usize,
                )
            })
            .collect::<Vec<_>>();

        Mesh::new(renderer, &mesh_data.buffers, mesh_data.indices, vertex_formats)
    }

    fn buffer_usage_to_shader_name(buffer_usage: &BufferItemUsage) -> &'static str {
        match buffer_usage {
            BufferItemUsage::Position => "position",
            BufferItemUsage::BoneWeight => "bone_weight",
            BufferItemUsage::BoneIndex => "bone_index",
            BufferItemUsage::Normal => "normal",
            BufferItemUsage::TexCoord => "tex_coord",
            BufferItemUsage::Tangent => "tangent",
            BufferItemUsage::BiTangent => "bi_tangent",
            BufferItemUsage::Color => "color",
        }
    }

    fn get_mesh_parts(mdl: &Mdl, mesh_data: &MdlMesh<'_>, visibility_mask: usize, hidden_attributes: &HashSet<&str>) -> Vec<Range<u32>> {
        mdl.parts()[mesh_data.mesh_info.part_offset as usize..mesh_data.mesh_info.part_offset as usize + mesh_data.mesh_info.part_count as usize]
            .iter()
            .filter_map(|mesh_part| {
                if mesh_part.visibility_mask & visibility_mask != mesh_part.visibility_mask
                    || mesh_part.attributes.intersection(hidden_attributes).next().is_some()
                {
                    None
                } else {
                    let begin = mesh_part.index_range.start - mesh_data.mesh_info.index_offset;
                    let end = mesh_part.index_range.end - mesh_data.mesh_info.index_offset;
                    Some(begin..end)
                }
            })
            .collect::<Vec<_>>()
    }

    fn load_bone_transform(renderer: &Renderer, mdl: &Mdl, mesh_data: &MdlMesh<'_>, bone_transforms: &HashMap<String, Matrix4<f32>>) -> Arc<Buffer> {
        let bone_names = mdl.bone_names(mesh_data.mesh_info.bone_index);
        let mut bone_transform_data = Vec::with_capacity(bone_names.size_hint().0 * 4 * 3 * core::mem::size_of::<f32>());
        for bone_name in bone_names {
            if let Some(x) = bone_transforms.get(bone_name) {
                // nalgebra's as_slice uses column_major, so we have to transpose it
                bone_transform_data.extend(x.transpose().as_slice()[..12].as_bytes());
            } else {
                let identity = [1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.];
                bone_transform_data.extend(identity.as_bytes());
            }
        }

        let bone_transform = Arc::new(renderer.buffer_pool.alloc(bone_transform_data.len()));
        bone_transform.write(&bone_transform_data);

        bone_transform
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
    fn render<'a>(&'a self, render_context: &mut RenderContext<'a>) {
        for model in &self.models {
            model.0.render(render_context);
        }
    }
}
