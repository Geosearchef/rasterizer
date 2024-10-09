use crate::image::DynamicImage;
use crate::pipeline::primitives::{Triangle2, VertexMetadata};
use crate::pipeline::rasterization::rasterize;
use nalgebra::{Vector2, Vector4};

pub fn clear_image(image: &mut DynamicImage) {
    image.set_black()
}

pub fn render_rasterization_to_image(points: Vec<(Vector2<i32>, VertexMetadata)>, image: &mut DynamicImage) {
    for p in points {
        if p.0.x >= 0 && p.0.x < image.width as i32 && p.0.y >= 0 && p.0.y < image.height as i32 {
            if let Some(color) = p.1.color {
                image.set_rgba(p.0.x as u32, p.0.y as u32, ((color.x * 255.0) as u8, (color.y * 255.0) as u8, (color.z * 255.0) as u8, (color.w * 255.0) as u8));
            } else {
                image.set_rgba(p.0.x as u32, p.0.y as u32, (255, 127, 0, 255));
            }
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
        meta: [
            VertexMetadata { world_pos: None, texture_coord: None, normal: None, color: Some(Vector4::new(1.0, 0.5, 0.0, 1.0)) },
            VertexMetadata { world_pos: None, texture_coord: None, normal: None, color: Some(Vector4::new(0.0, 1.0, 0.5, 1.0)) },
            VertexMetadata { world_pos: None, texture_coord: None, normal: None, color: Some(Vector4::new(0.5, 0.0, 1.0, 1.0)) }
        ]
    };

    render_triangle_to_image(&tri, image);
}