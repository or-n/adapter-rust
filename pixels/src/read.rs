use super::dimensions::Dimensions;

pub fn f32_array(
    input_file: &str,
) -> image::error::ImageResult<(Dimensions, Vec<f32>)> {
    let img = image::open(input_file)?.to_rgba32f();
    let (width, height) = img.dimensions();
    Ok((Dimensions { width, height }, img.into_raw()))
}
