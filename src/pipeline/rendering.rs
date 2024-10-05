use crate::image::DynamicImage;
use crate::pipeline::primitives::{Triangle2, VertexMetadata};
use crate::pipeline::rasterization::rasterize;
use nalgebra::Vector2;

pub fn clear_image(image: &mut DynamicImage) {
    image.set_black()
}

pub fn render_rasterization_to_image(points: Vec<(Vector2<i32>, VertexMetadata)>, image: &mut DynamicImage) {
    for p in points {
        if p.0.x >= 0 && p.0.x < image.width as i32 && p.0.y >= 0 && p.0.y < image.height as i32 {
            image.set_rgba(p.0.x as u32, p.0.y as u32, (255, 127, 0, 255));
        }
    }
}
pub fn render_triangle_to_image(triangle: &Triangle2, image: &mut DynamicImage) {
    let ras = rasterize(triangle);
    println!("Rasterized points: {}", ras.len());
    render_rasterization_to_image(ras, image);
}

pub fn render_scene_to_image(image: &mut DynamicImage) {
    let tri = Triangle2 {
        v: [Vector2::new(100.0, 250.0), Vector2::new(300.0, 300.0), Vector2::new(300.0, 100.0)],
        t: [Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)],
        n: [Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)]
    };

    render_triangle_to_image(&tri, image);
}