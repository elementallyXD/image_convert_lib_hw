use image_convert_lib_hw::{ImageConvert, Loaded, UserFormat};
use std::fs;

fn main() -> image::ImageResult<()> {
    // To test this library, you can use any JPEG or PNG files.
    // Here I am using sample images from examples/ directory.

    // Such JPG files contains in examples/ directory:
    // sample-jpg-files-sample_1920x1280.jpg
    // sample-jpg-files-sample-5.jpg

    // Such PNG files contains in examples/ directory:
    // sample-png-files-sample-5.png
    // sample-png-files-sample_1920x1280.png

    // Load an image from file
    let input_bytes = fs::read("examples/sample-jpg-files-sample-5.jpg")?;
    let loaded = ImageConvert::<Loaded>::new(input_bytes, UserFormat::Jpeg)?;

    // Convert image to raw bytes
    let raw = loaded.to_raw()?;

    // Convert raw bytes to specific format
    let png = raw.to_format(UserFormat::Png)?;

    // Get encoded bytes and reset back to Raw
    let (png_bytes, fmt, _reset) = png.get_and_reset()?;

    // Save the resulting bytes to an output file
    assert!(matches!(fmt, UserFormat::Png));
    fs::write("examples/output.png", png_bytes)?;

    Ok(())
}
