use std::cell::RefCell;
use std::cmp;
use std::sync::Arc;

use log::debug;

use crate::byte_reader::ByteReader;
use crate::{animation::HavokAnimation, object::HavokObject, transform::HavokTransform};

#[repr(u8)]
enum RotationQuantization {
    POLAR32 = 0,
    THREECOMP40 = 1,
    THREECOMP48 = 2,
    THREECOMP24 = 3,
    STRAIGHT16 = 4,
    UNCOMPRESSED = 5,
}

impl RotationQuantization {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::POLAR32,
            1 => Self::THREECOMP40,
            2 => Self::THREECOMP48,
            3 => Self::THREECOMP24,
            4 => Self::STRAIGHT16,
            5 => Self::UNCOMPRESSED,
            _ => panic!(),
        }
    }
}

#[repr(u8)]
enum ScalarQuantization {
    BITS8 = 0,
    BITS16 = 1,
}

impl ScalarQuantization {
    pub fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::BITS8,
            1 => Self::BITS16,
            _ => panic!(),
        }
    }
}

pub struct HavokSplineCompressedAnimation {
    duration: f32,
    number_of_transform_tracks: usize,
    num_frames: usize,
    num_blocks: usize,
    max_frames_per_block: usize,
    mask_and_quantization_size: u32,
    block_inverse_duration: f32,
    frame_duration: f32,
    block_offsets: Vec<u32>,
    data: Vec<u8>,
}

impl HavokSplineCompressedAnimation {
    pub fn new(object: Arc<RefCell<HavokObject>>) -> Self {
        let root = object.borrow();

        let duration = root.get("duration").as_real();
        let number_of_transform_tracks = root.get("numberOfTransformTracks").as_int() as usize;
        let num_frames = root.get("numFrames").as_int() as usize;
        let num_blocks = root.get("numBlocks").as_int() as usize;
        let max_frames_per_block = root.get("maxFramesPerBlock").as_int() as usize;
        let mask_and_quantization_size = root.get("maskAndQuantizationSize").as_int() as u32;
        let block_inverse_duration = root.get("blockInverseDuration").as_real();
        let frame_duration = root.get("frameDuration").as_real();

        let raw_block_offsets = root.get("blockOffsets").as_array();
        let block_offsets = raw_block_offsets.iter().map(|x| x.as_int() as u32).collect::<Vec<_>>();

        let raw_data = root.get("data").as_array();
        let data = raw_data.iter().map(|x| x.as_int() as u8).collect::<Vec<_>>();

        Self {
            duration,
            number_of_transform_tracks,
            num_frames,
            num_blocks,
            max_frames_per_block,
            mask_and_quantization_size,
            block_inverse_duration,
            frame_duration,
            block_offsets,
            data,
        }
    }

    fn get_block_and_time(&self, frame: usize, delta: f32) -> (usize, f32, u8) {
        let mut block_out = frame / (self.max_frames_per_block - 1);

        block_out = cmp::max(block_out, 0);
        block_out = cmp::min(block_out, self.num_blocks - 1);

        let first_frame_of_block = block_out * (self.max_frames_per_block - 1);
        let real_frame = (frame - first_frame_of_block) as f32 + delta;
        let block_time_out = real_frame * self.frame_duration;

        let quantized_time_out = ((block_time_out * self.block_inverse_duration) * (self.max_frames_per_block as f32 - 1.)) as u8;

        (block_out, block_time_out, quantized_time_out)
    }

    fn compute_packed_nurbs_offsets<'a>(base: &'a [u8], p: &[u32], o2: usize, o3: u32) -> &'a [u8] {
        let offset = (p[o2] + (o3 & 0x7fff_ffff)) as usize;

        &base[offset..]
    }

    fn unpack_quantization_types(packed_quantization_types: u8) -> (ScalarQuantization, RotationQuantization, ScalarQuantization) {
        let translation = ScalarQuantization::from_raw(packed_quantization_types & 0x03);
        let rotation = RotationQuantization::from_raw((packed_quantization_types >> 2) & 0x0F);
        let scale = ScalarQuantization::from_raw((packed_quantization_types >> 6) & 0x03);

        (translation, rotation, scale)
    }

    fn sample_translation(&self, quantization: ScalarQuantization, time: f32, quantized_time: u8, mask: u8, data: &mut ByteReader) -> [f32; 4] {
        let result = if mask != 0 {
            Self::read_nurbs_curve(quantization, data, quantized_time, self.frame_duration, time, mask, [0., 0., 0., 0.])
        } else {
            [0., 0., 0., 0.]
        };

        data.align(4);

        result
    }

    fn sample_rotation(&self, quantization: RotationQuantization, time: f32, quantized_time: u8, mask: u8, data: &mut ByteReader) -> [f32; 4] {
        let result = Self::read_nurbs_quaternion(quantization, data, quantized_time, self.frame_duration, time, mask);

        data.align(4);

        result
    }

    fn sample_scale(&self, quantization: ScalarQuantization, time: f32, quantized_time: u8, mask: u8, data: &mut ByteReader) -> [f32; 4] {
        let result = if mask != 0 {
            Self::read_nurbs_curve(quantization, data, quantized_time, self.frame_duration, time, mask, [1., 1., 1., 1.])
        } else {
            [1., 1., 1., 1.]
        };

        data.align(4);

        result
    }

    #[allow(unused_variables)]
    fn read_nurbs_curve(
        quantization: ScalarQuantization,
        data: &mut ByteReader,
        quantized_time: u8,
        frame_duration: f32,
        u: f32,
        mask: u8,
        initial_value: [f32; 4],
    ) -> [f32; 4] {
        initial_value
    }

    #[allow(unused_variables)]
    fn read_nurbs_quaternion(
        quantization: RotationQuantization,
        data: &mut ByteReader,
        quantized_time: u8,
        frame_duration: f32,
        u: f32,
        mask: u8,
    ) -> [f32; 4] {
        [0., 0., 0., 1.]
    }
}

impl HavokAnimation for HavokSplineCompressedAnimation {
    fn sample(&self, time: f32) -> Vec<HavokTransform> {
        let frame_float = ((time / 1000.) / self.duration) * (self.num_frames as f32 - 1.);
        let frame = frame_float as usize;
        let delta = frame_float - frame as f32;

        let (block, block_time, quantized_time) = self.get_block_and_time(frame, delta);
        debug!("{} {} {}", block, block_time, quantized_time);

        let mut data = ByteReader::new(Self::compute_packed_nurbs_offsets(
            &self.data,
            &self.block_offsets,
            block,
            self.mask_and_quantization_size,
        ));
        let mut mask = ByteReader::new(Self::compute_packed_nurbs_offsets(&self.data, &self.block_offsets, block, 0x8000_0000));

        let mut result = Vec::with_capacity(self.number_of_transform_tracks);
        for _ in 0..self.number_of_transform_tracks {
            let packed_quantization_types = mask.read();

            let (translation_type, rotation_type, scale_type) = Self::unpack_quantization_types(packed_quantization_types);

            let translation = self.sample_translation(translation_type, block_time, quantized_time, mask.read(), &mut data);
            let rotation = self.sample_rotation(rotation_type, block_time, quantized_time, mask.read(), &mut data);
            let scale = self.sample_scale(scale_type, block_time, quantized_time, mask.read(), &mut data);

            result.push(HavokTransform::from_trs(translation, rotation, scale));
        }

        result
    }

    fn duration(&self) -> f32 {
        self.duration
    }
}
