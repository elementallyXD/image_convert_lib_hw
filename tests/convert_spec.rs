use std::{fs, path::PathBuf};

use image_convert_lib_hw::{ImageConvert, Loaded, UserFormat};

// Helper to load fixture files from examples/ directory
// Panics if the file is missing.
fn fixture(name: &str) -> Vec<u8> {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", name]
        .iter()
        .collect();
    fs::read(&path).expect(&format!("missing fixture: {}", path.display()))
}

#[test]
// Test converting PNG fixtures to JPEG
fn png_fixtures_convert_to_jpeg() {
    for name in [
        "sample-png-files-sample_1920x1280.png",
    ] {
        let png_bytes = fixture(name);

        let raw = ImageConvert::<Loaded>::new(png_bytes, UserFormat::Png)
            .expect("validate png format")
            .to_raw()
            .expect("decode png");

        let jpeg = raw.to_format(UserFormat::Jpeg).expect("encode jpeg");

        let (jpeg_bytes, fmt, reset) = jpeg.get_and_reset().expect("reset from encoded jpeg");

        assert!(matches!(fmt, UserFormat::Jpeg));
        assert!(
            !jpeg_bytes.is_empty(),
            "encoded jpeg should not be empty for {}",
            name
        );

        // Ensure the reset Raw state can still encode again
        let reencoded = reset
            .to_format(UserFormat::Jpeg)
            .expect("re-encode after reset");

        let (bytes2, fmt2, _) = reencoded
            .get_and_reset()
            .expect("reset from re-encoded jpeg");

        assert!(matches!(fmt2, UserFormat::Jpeg));
        assert!(
            !bytes2.is_empty(),
            "re-encoded jpeg should not be empty for {}",
            name
        );
    }
}

#[test]
// Test converting JPEG fixtures to PNG
fn jpeg_fixtures_convert_to_png() {
    for name in [
        "sample-jpg-files-sample-5.jpg",
        "sample-jpg-files-sample_1920x1280.jpg",
    ] {
        let jpeg_bytes = fixture(name);

        let raw = ImageConvert::<Loaded>::new(jpeg_bytes, UserFormat::Jpeg)
            .expect("validate jpeg format")
            .to_raw()
            .expect("decode jpeg");

        let png = raw.to_format(UserFormat::Png).expect("encode png");

        let (png_bytes, fmt, reset) = png.get_and_reset().expect("reset from encoded png");

        assert!(matches!(fmt, UserFormat::Png));
        assert!(
            !png_bytes.is_empty(),
            "encoded png should not be empty for {}",
            name
        );

        // Ensure the reset Raw state can encode again to JPEG (cross check another path)
        let reencoded = reset
            .to_format(UserFormat::Jpeg)
            .expect("encode jpeg after reset");

        let (jpeg_bytes, fmt2, _) = reencoded.get_and_reset().expect("reset from encoded jpeg");

        assert!(matches!(fmt2, UserFormat::Jpeg));
        assert!(
            !jpeg_bytes.is_empty(),
            "jpeg from reset raw should not be empty for {}",
            name
        );
    }
}

#[test]
// Test that corrupted input data fails to load
fn corrupted_input_fails() {
    let bad = vec![0u8; 16];

    let err = ImageConvert::<Loaded>::new(bad, UserFormat::Png);

    assert!(err.is_err());
}

#[test]
// Test that declared format mismatch errors are raised
fn declared_format_mismatch_errors() {
    let jpeg_bytes = fixture("sample-jpg-files-sample-5.jpg");

    let err = ImageConvert::<Loaded>::new(jpeg_bytes, UserFormat::Png);

    assert!(err.is_err());
}
