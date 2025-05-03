use libraw;

#[test]
fn test_libraw() {
    use image;
    let file = "./tests/demo.dng";
    let file = "C:/Users/panda/Pictures/colorchart-5D2-6000K.dng";
    let out = libraw::raw_file_rgb8(file);
    assert_eq!(out.is_ok(), true);
    let output = out.unwrap();
    let width = output.width;
    let height = output.height;
    let data = output.data;
    assert_eq!(width, 5634);
    assert_eq!(height, 3752);
    assert_eq!(data.len(), (width * height * 3) as usize);

    let buffer = image::ImageBuffer::from_raw(width, height, data);
    assert_eq!(buffer.is_some(), true);
    let buffer = buffer.unwrap();
    let img = image::DynamicImage::ImageRgb8(buffer);
    let ok = img.save("tests/output.png").is_ok();
    assert!(ok);
}
