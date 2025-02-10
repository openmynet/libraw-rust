use std::path::Path;
use std::io::{self, Read, Seek, SeekFrom};
pub mod libraw;
mod sys;

/// A raw RGB8 image.
pub struct Rgb8Image{
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Process a raw image buffer into a RGB8 image.
/// ```rust,no_run
///   let buffer = image::ImageBuffer::from_raw(width, height, data).ok_or("Cannot create ImageBuffer from raw image")?;
///   let img = image::DynamicImage::ImageRgb8(buffer);
/// ```

pub fn raw_rgb8(buff: &[u8]) -> Result<Rgb8Image, Box<dyn std::error::Error>>{
    
    let processor = crate::libraw::Processor::new();
    let processed = processor.process_8bit(&buff)?;

    let width = processed.width();
    let height = processed.height();

    let data = processed.to_vec();
    let data_len = data.len();
    if data_len != (width * height * 3) as usize {
        return Err("Invalid data length".into());
    }
    let image = Rgb8Image {
        width,
        height,
        data,
    };
    return Ok(image);
}

pub fn raw_file_rgb8<P: AsRef<Path>>(raw_file: P) -> Result<Rgb8Image, Box<dyn std::error::Error>>{
    let buf = std::fs::read(raw_file)?;
    return raw_rgb8(&buf);
}

pub fn raw_reader_rgb8<R: Read + Seek>(reader: &mut R ) -> Result<Rgb8Image, Box<dyn std::error::Error>>{
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    // reset the reader to the start
    reader.by_ref().seek(SeekFrom::Start(0))?;
    return raw_rgb8(&buf);
}
