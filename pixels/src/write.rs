use super::dimensions::Dimensions;

#[derive(Debug)]
pub enum Error {
    Dimensions,
}

pub fn f32_array(
    dimensions: Dimensions,
    pixels: Vec<f32>,
    output_file: &str,
) -> Result<image::error::ImageResult<()>, Error> {
    let img = image::Rgba32FImage::from_raw(
        dimensions.width,
        dimensions.height,
        pixels,
    )
    .ok_or(Error::Dimensions)?;
    let img = image::DynamicImage::ImageRgba32F(img).to_rgba8();
    Ok(img.save_with_format(output_file, image::ImageFormat::Jpeg))
}
