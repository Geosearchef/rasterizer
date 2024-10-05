use iced::widget::canvas::Image;
// use image::{ImageReader, RgbaImage};

pub fn load_image(path: &str) -> Image {
    // Image::from(ImageReader::open(path).expect(format!("Couldn't load image {}", path).as_str()).decode().unwrap().to_rgba8())
    Image::new(path)
}
