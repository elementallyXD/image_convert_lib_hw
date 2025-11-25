use image::{self, ImageBuffer, ImageFormat, Rgba};
use std::{io::Cursor, marker::PhantomData};

// Public enum for supported user formats
#[derive(Debug, Clone, Copy)]
pub enum UserFormat {
    Jpeg,
    Png,
}

// Conversion from UserFormat to image::ImageFormat
impl From<UserFormat> for ImageFormat {
    fn from(fmt: UserFormat) -> Self {
        match fmt {
            UserFormat::Jpeg => ImageFormat::Jpeg,
            UserFormat::Png => ImageFormat::Png,
        }
    }
}

// Typestate wrapper for image conversion
pub struct ImageConvert<State> {
    bytes: Vec<u8>,
    format: UserFormat,
    state: State,
}

// States for the typestate pattern
pub struct Loaded;

// Raw state holds width and height
pub struct Raw {
    width: u32,
    height: u32,
}

// Encoded state holds a PhantomData for the format
pub struct Encoded<F> {
    _format: PhantomData<F>,
}

impl ImageConvert<Loaded> {
    // Create a new ImageConvert instance in the Loaded state
    pub fn new(bytes: Vec<u8>, format: UserFormat) -> image::ImageResult<Self> {
        // Validate that the declared format matches the file signature.
        let declared = ImageFormat::from(format);
        let guessed = image::guess_format(&bytes)?;
        if guessed != declared {
            return Err(image::ImageError::Unsupported(
                image::error::UnsupportedError::from_format_and_kind(
                    image::error::ImageFormatHint::Exact(declared),
                    image::error::UnsupportedErrorKind::Format(
                        image::error::ImageFormatHint::Exact(guessed),
                    ),
                ),
            ));
        }

        Ok(Self {
            bytes,
            format,
            state: Loaded,
        })
    }

    // Convert loaded image to raw pixel data.
    // Returns ImageConvert in Raw state.
    pub fn to_raw(self) -> image::ImageResult<ImageConvert<Raw>> {
        let img = image::load_from_memory_with_format(&self.bytes, ImageFormat::from(self.format))?;

        let rgba = img.to_rgba8();

        let (width, height) = rgba.dimensions();

        Ok(ImageConvert {
            bytes: rgba.into_raw(),
            format: self.format,
            state: Raw { width, height },
        })
    }

    // Get current bytes and format, and reset original Raw state
    pub fn get_and_reset(self) -> image::ImageResult<(Vec<u8>, UserFormat, ImageConvert<Raw>)> {
        let raw = self.to_raw()?;

        let reset = ImageConvert {
            bytes: raw.bytes.clone(),
            format: raw.format,
            state: Raw {
                width: raw.state.width,
                height: raw.state.height,
            },
        };

        Ok((raw.bytes, raw.format, reset))
    }
}

impl ImageConvert<Raw> {
    // Convert raw pixel data to specified format.
    // Returns ImageConvert in Encoded state.
    pub fn to_format(
        self,
        target: UserFormat,
    ) -> image::ImageResult<ImageConvert<Encoded<UserFormat>>> {
        let width = self.state.width;
        let height = self.state.height;

        // Create ImageBuffer from raw bytes. On error, return DimensionMismatch.
        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, self.bytes.clone())
            .ok_or_else(|| {
                image::ImageError::Parameter(image::error::ParameterError::from_kind(
                    image::error::ParameterErrorKind::DimensionMismatch,
                ))
            })?;

        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        image::DynamicImage::ImageRgba8(img).write_to(&mut cursor, ImageFormat::from(target))?;

        Ok(ImageConvert {
            bytes: buf,
            format: target,
            state: Encoded {
                _format: PhantomData,
            },
        })
    }

    // Get current bytes and format, and reset original Raw state
    pub fn get_and_reset(self) -> image::ImageResult<(Vec<u8>, UserFormat, ImageConvert<Raw>)> {
        let reset = ImageConvert {
            bytes: self.bytes.clone(),
            format: self.format,
            state: Raw {
                width: self.state.width,
                height: self.state.height,
            },
        };

        Ok((self.bytes, self.format, reset))
    }
}

impl<F> ImageConvert<Encoded<F>> {
    // Get current bytes and format, and reset original Raw state
    pub fn get_and_reset(self) -> image::ImageResult<(Vec<u8>, UserFormat, ImageConvert<Raw>)> {
        let decoded =
            image::load_from_memory_with_format(&self.bytes, ImageFormat::from(self.format))?;

        let rgba = decoded.to_rgba8();
        let (width, height) = rgba.dimensions();
        let raw_bytes = rgba.into_raw();

        let reset = ImageConvert {
            bytes: raw_bytes.clone(),
            format: self.format,
            state: Raw { width, height },
        };

        Ok((self.bytes, self.format, reset))
    }
}
