use alloc::{sync::Arc, vec, vec::Vec};
use core::{cell::RefCell, cmp, convert::TryInto};

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

    pub fn align(&self) -> usize {
        match self {
            Self::POLAR32 => 4,
            Self::THREECOMP40 => 1,
            Self::THREECOMP48 => 2,
            Self::THREECOMP24 => 1,
            Self::STRAIGHT16 => 2,
            Self::UNCOMPRESSED => 4,
        }
    }

    pub fn bytes_per_quaternion(&self) -> usize {
        match self {
            Self::POLAR32 => 4,
            Self::THREECOMP40 => 5,
            Self::THREECOMP48 => 6,
            Self::THREECOMP24 => 3,
            Self::STRAIGHT16 => 2,
            Self::UNCOMPRESSED => 16,
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

    pub fn bytes_per_component(&self) -> usize {
        match self {
            Self::BITS8 => 1,
            Self::BITS16 => 2,
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

    #[allow(non_snake_case)]
    fn find_span(n: usize, p: usize, u: u8, U: &[u8]) -> usize {
        if u >= U[n + 1] {
            return n;
        }
        if u <= U[0] {
            return p;
        }

        let mut low = p;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        while u < U[mid] || u >= U[mid + 1] {
            if u < U[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }

    fn read_knots(data: &mut ByteReader, u: u8, frame_duration: f32) -> (usize, usize, Vec<f32>, usize) {
        let n = u16::from_le_bytes(data.read_bytes(2).try_into().unwrap()) as usize;
        let p = data.read() as usize;
        let raw = data.raw();
        let span = Self::find_span(n, p, u, raw);

        #[allow(non_snake_case)]
        let mut U = vec![0.; 2 * p];

        for i in 0..2 * p {
            let item = raw[i + 1] as usize + span - p;
            U[i] = (item as f32) * frame_duration;
        }

        data.seek(n + p + 2);

        (n, p, U, span)
    }

    #[allow(unused_variables)]
    fn unpack_quaternion(quantization: &RotationQuantization, data: &[u8]) -> [f32; 4] {
        // TODO
        [0., 0., 0., 1.]
    }

    fn read_packed_quaternions(quantization: RotationQuantization, data: &mut ByteReader, n: usize, p: usize, span: usize) -> Vec<[f32; 4]> {
        data.align(quantization.align());
        let bytes_per_quaternion = quantization.bytes_per_quaternion();

        let mut result = Vec::new();
        for i in 0..(p + 1) {
            result.push(Self::unpack_quaternion(
                &quantization,
                &data.raw()[bytes_per_quaternion * (i + span - p)..],
            ));
        }

        data.seek(bytes_per_quaternion * (n + 1));

        result
    }

    fn unpack_vec_8(min_p: [f32; 4], max_p: [f32; 4], vals: &[u8]) -> [f32; 4] {
        let mut result = [0., 0., 0., 1.];
        for i in 0..4 {
            result[i] = ((vals[i] as f32) / 255.) * (max_p[i] - min_p[i]) + min_p[i];
        }

        result
    }

    fn unpack_vec_16(min_p: [f32; 4], max_p: [f32; 4], vals: &[u16]) -> [f32; 4] {
        let mut result = [0., 0., 0., 1.];
        for i in 0..4 {
            result[i] = ((vals[i] as f32) / 65535.) * (max_p[i] - min_p[i]) + min_p[i];
        }

        result
    }

    #[allow(non_snake_case)]
    fn recompose(stat_mask: u8, dyn_mask: u8, S: [f32; 4], I: [f32; 4], in_out: &mut [f32; 4]) {
        for i in 0..4 {
            if stat_mask & (1 << i) != 0 {
                in_out[i] = S[i];
            }
        }

        for i in 0..4 {
            if dyn_mask & (1 << i) != 0 {
                in_out[i] = I[i];
            }
        }
    }

    #[allow(non_snake_case)]
    fn evaluate(time: f32, p: usize, U: &[f32], P: &[[f32; 4]]) -> [f32; 4] {
        let mut result = [0., 0., 0., 1.];
        if p == 1 {
            let t = (time - U[0]) / (U[1] - U[0]);

            for (i, item) in result.iter_mut().enumerate() {
                *item = P[0][i] + t * (P[1][i] - P[0][i]);
            }
        } else {
            // evaluate interpolation.
            let v31 = p - 1;
            let mut v29 = [1.; 16];
            let mut v22 = [0.; 16];
            let mut v24 = [0.; 16];

            for i in 1..(p + 1) {
                v24[4 * i] = time - U[v31 + 1 - i];
                v22[4 * i] = U[i + v31] - time;
                let mut v21 = 0.;
                for j in 0..i {
                    let v19 = v22[4 * (j + 1)] + v24[4 * (i - j)];
                    let v18 = v29[4 * j] / v19;
                    let v17 = v22[4 * (j + 1)] * v18;
                    v29[4 * j] = v21 + v17;
                    v21 = v24[4 * (i - j)] * v18;
                }
                v29[4 * i] = v21;
            }
            for i in 0..(p + 1) {
                for (j, item) in result.iter_mut().enumerate() {
                    *item += v29[4 * i] * P[i][j];
                }
            }
        }

        result
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

    #[allow(non_snake_case)]
    fn read_nurbs_curve(
        quantization: ScalarQuantization,
        data: &mut ByteReader,
        quantized_time: u8,
        frame_duration: f32,
        u: f32,
        mask: u8,
        I: [f32; 4],
    ) -> [f32; 4] {
        let mut max_p = [0., 0., 0., 1.];
        let mut min_p = [0., 0., 0., 1.];
        let mut S = [0., 0., 0., 1.];

        let (n, p, U, span) = if mask & 0xf0 != 0 {
            Self::read_knots(data, quantized_time, frame_duration)
        } else {
            (0, 0, vec![0.; 10], 0)
        };
        data.align(4);

        for i in 0..3 {
            if (1 << i) & mask != 0 {
                S[i] = f32::from_le_bytes(data.read_bytes(4).try_into().unwrap());
            } else if (1 << (i + 4)) & mask != 0 {
                min_p[i] = f32::from_le_bytes(data.read_bytes(4).try_into().unwrap());
                max_p[i] = f32::from_le_bytes(data.read_bytes(4).try_into().unwrap());
            }
        }

        let stat_mask = mask & 0x0f;
        let dyn_mask = (!mask >> 4) & (!mask & 0x0f);

        if mask & 0xf0 != 0 {
            let bytes_per_component = quantization.bytes_per_component();
            data.align(2);

            let sizes = [0, 1, 1, 2, 1, 2, 2, 3];
            let size = sizes[((mask >> 4) & 7) as usize];
            let mut new_data = data.clone();
            new_data.seek(bytes_per_component * size * (span - p));

            let mut P = [[0., 0., 0., 1.]; 4];
            for pv in P.iter_mut().take(p + 1) {
                match quantization {
                    ScalarQuantization::BITS8 => {
                        let mut vals = [0; 4];
                        for (j, item) in vals.iter_mut().enumerate().take(3) {
                            if (1 << (j + 4)) & mask != 0 {
                                *item = new_data.read();
                            }
                        }

                        *pv = Self::unpack_vec_8(min_p, max_p, &vals);
                    }
                    ScalarQuantization::BITS16 => {
                        let mut vals = [0; 4];
                        for (j, item) in vals.iter_mut().enumerate().take(3) {
                            if (1 << (j + 4)) & mask != 0 {
                                *item = u16::from_le_bytes(new_data.read_bytes(2).try_into().unwrap());
                            }
                        }

                        *pv = Self::unpack_vec_16(min_p, max_p, &vals);
                    }
                }

                Self::recompose(stat_mask, dyn_mask, S, I, pv);
            }

            let result = Self::evaluate(u, p, &U, &P);

            data.seek(bytes_per_component * size * (n + 1));

            result
        } else {
            let mut result = I;
            Self::recompose(stat_mask, dyn_mask, S, I, &mut result);

            result
        }
    }

    #[allow(non_snake_case)]
    fn read_nurbs_quaternion(
        quantization: RotationQuantization,
        data: &mut ByteReader,
        quantized_time: u8,
        frame_duration: f32,
        u: f32,
        mask: u8,
    ) -> [f32; 4] {
        if mask & 0xf0 != 0 {
            let (n, p, U, span) = Self::read_knots(data, quantized_time, frame_duration);
            let P = Self::read_packed_quaternions(quantization, data, n, p, span);
            Self::evaluate(u, p, &U, &P)
        } else if mask & 0x0f != 0 {
            data.align(quantization.align());
            let result = Self::unpack_quaternion(&quantization, data.raw());
            data.seek(quantization.bytes_per_quaternion());

            result
        } else {
            [0., 0., 0., 1.]
        }
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
