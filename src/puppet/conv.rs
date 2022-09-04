//! here are some utilities that im pretty sure aren't supposed to exist. three_d's examples work
//! without doing this

fn interpolation(asset: three_d_asset::Interpolation) -> three_d::Interpolation {
    match asset {
        three_d_asset::Interpolation::Nearest => three_d::Interpolation::Nearest,
        three_d_asset::Interpolation::Linear => three_d::Interpolation::Linear,
    }
}

fn wrapping(asset: three_d_asset::Wrapping) -> three_d::Wrapping {
    match asset {
        three_d_asset::Wrapping::Repeat => three_d::Wrapping::Repeat,
        three_d_asset::Wrapping::MirroredRepeat => three_d::Wrapping::MirroredRepeat,
        three_d_asset::Wrapping::ClampToEdge => three_d::Wrapping::ClampToEdge,
    }
}

fn half(asset: &three_d_asset::f16) -> three_d::f16 {
    three_d::f16::from_bits(asset.to_bits())
}

fn rhalf(asset: Vec<three_d_asset::f16>) -> Vec<three_d::f16> {
    let mut output = Vec::new();
    output.extend(asset.iter().map(|d| half(d)));
    output
}

// rghalf..rgbahalf might work better as macros

fn rghalf(asset: Vec<[three_d_asset::f16; 2]>) -> Vec<[three_d::f16; 2]> {
    let mut output = Vec::new();
    output.extend(asset.iter().map(|d| [half(&d[0]), half(&d[1])]));
    output
}

fn rgbhalf(asset: Vec<[three_d_asset::f16; 3]>) -> Vec<[three_d::f16; 3]> {
    let mut output = Vec::new();
    output.extend(
        asset
            .iter()
            .map(|d| [half(&d[0]), half(&d[1]), half(&d[2])]),
    );
    output
}

fn rgbahalf(asset: Vec<[three_d_asset::f16; 4]>) -> Vec<[three_d::f16; 4]> {
    let mut output = Vec::new();
    output.extend(
        asset
            .iter()
            .map(|d| [half(&d[0]), half(&d[1]), half(&d[2]), half(&d[3])]),
    );
    output
}

pub fn from_asset(asset: three_d_asset::Texture2D) -> three_d::CpuTexture {
    three_d::CpuTexture {
        data: match asset.data {
            three_d_asset::TextureData::RU8(d) => three_d::TextureData::RU8(d),
            three_d_asset::TextureData::RgU8(d) => three_d::TextureData::RgU8(d),
            three_d_asset::TextureData::RgbU8(d) => three_d::TextureData::RgbU8(d),
            three_d_asset::TextureData::RgbaU8(d) => three_d::TextureData::RgbaU8(d),
            three_d_asset::TextureData::RF16(d) => three_d::TextureData::RF16(rhalf(d)),
            three_d_asset::TextureData::RgF16(d) => three_d::TextureData::RgF16(rghalf(d)),
            three_d_asset::TextureData::RgbF16(d) => three_d::TextureData::RgbF16(rgbhalf(d)),
            three_d_asset::TextureData::RgbaF16(d) => three_d::TextureData::RgbaF16(rgbahalf(d)),
            three_d_asset::TextureData::RF32(d) => three_d::TextureData::RF32(d),
            three_d_asset::TextureData::RgF32(d) => three_d::TextureData::RgF32(d),
            three_d_asset::TextureData::RgbF32(d) => three_d::TextureData::RgbF32(d),
            three_d_asset::TextureData::RgbaF32(d) => three_d::TextureData::RgbaF32(d),
        },
        height: asset.height,
        width: asset.width,
        mag_filter: interpolation(asset.mag_filter),
        min_filter: interpolation(asset.min_filter),
        mip_map_filter: match asset.mip_map_filter {
            None => None,
            Some(i) => Some(interpolation(i)),
        },
        wrap_s: wrapping(asset.wrap_s),
        wrap_t: wrapping(asset.wrap_t),
    }
}
