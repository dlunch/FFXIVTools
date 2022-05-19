use std::io::Cursor;

use image::RgbaImage;

use ffxiv_parser::{Tex, TextureType};
use util::cast_array;

fn convert_5551_to_rgba(raw: &[u8]) -> Vec<u8> {
    let raw = cast_array::<u16>(raw);

    raw.iter()
        .flat_map(|i| {
            let b = ((i & 0x1f) * 8) as u8;
            let g = (((i >> 5) & 0x1f) * 8) as u8;
            let r = (((i >> 10) & 0x1f) * 8) as u8;
            let a = (((i >> 15) & 0x1) * 255) as u8;

            [r, g, b, a]
        })
        .collect()
}

fn decode_dxtn(format: TextureType, raw: &[u8], width: usize, height: usize) -> Vec<u8> {
    let format = match format {
        TextureType::DXT1 => squish::Format::Bc1,
        TextureType::DXT3 => squish::Format::Bc2,
        TextureType::DXT5 => squish::Format::Bc3,
        _ => unreachable!(),
    };

    let mut result = vec![0; width * height * 4];

    format.decompress(raw, width, height, &mut result);

    result
}

pub fn tex_to_png(tex: &Tex) -> Vec<u8> {
    let rgba = match tex.texture_type() {
        TextureType::RGBA5551 => convert_5551_to_rgba(tex.data(0)),
        TextureType::DXT1 | TextureType::DXT3 | TextureType::DXT5 => {
            decode_dxtn(tex.texture_type(), tex.data(0), tex.width() as usize, tex.height() as usize)
        }

        _ => unimplemented!(),
    };

    let result = Vec::new();
    let mut writer = Cursor::new(result);

    let image = RgbaImage::from_raw(tex.width() as u32, tex.height() as u32, rgba).unwrap();
    image.write_to(&mut writer, image::ImageOutputFormat::Png).unwrap();

    writer.into_inner()
}
