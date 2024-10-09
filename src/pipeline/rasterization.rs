use crate::pipeline::primitives::{Primitive2, Triangle2, VertexMetadata};
use crate::pipeline::util::vector2i_lerp;
use iced::widget::canvas::path::lyon_path::geom::euclid::num::Round;
use nalgebra::Vector2;

pub fn rasterize(triangle: &Triangle2) -> Vec<(Vector2<i32>, VertexMetadata)> {

    let mut vertices: Vec<(VertexMetadata, Vector2<i32>)> = triangle.vertices().iter().enumerate()
        .map(|(i, v)| (triangle.meta[i], Vector2::new(v.x.round() as i32, v.y.round() as i32)))
        .collect();
    vertices.sort_by(|v1, v2| v1.1.y.cmp(&(v2.1.y)));

    let meta: Vec<&VertexMetadata> = vertices.iter().map(|(m, _)| m).collect();
    let vertices: Vec<Vector2<i32>> = vertices.iter().map(|(_, v)| *v).collect();

    if (&vertices[0]).y == (&vertices[2]).y { // all match
        // line, width == 0
        vec![]
    } else if (&vertices[0]).y == (&vertices[1]).y {
        // tip down
        rasterize_tip_down_unsorted(&vertices[2], [&vertices[0], &vertices[1]], [meta[2], meta[0], meta[1]])
    } else if (&vertices[1]).y == (&vertices[2]).y {
        // tip up
        rasterize_tip_up_unsorted(&vertices[0], [&vertices[1], &vertices[2]], [meta[0], meta[1], meta[2]])
    } else {
        // both

        let mid_point = vector2i_lerp( &vertices[0], &vertices[2], (vertices[1].y - vertices[0].y) as f64 / (vertices[2].y - vertices[0].y) as f64);
        let mid_meta = interpolate_meta([0.5, 0.0, 0.5], [meta[0], meta[1], meta[2]]); // 50-50 between 0 and 2

        let mut res1 = rasterize_tip_up_unsorted(&vertices[0], [&vertices[1], &mid_point], [meta[0], meta[1], &mid_meta]);
        let mut res2 = rasterize_tip_down_unsorted(&vertices[2], [&vertices[1], &mid_point], [meta[2], meta[1], &mid_meta]);
        res1.append(&mut res2);
        res1
    }
}

fn rasterize_tip_up_unsorted(u: &Vector2<i32>, lr: [&Vector2<i32>; 2], meta: [&VertexMetadata; 3]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    if lr[0].x < lr[1].x {
        rasterize_tip_up(u, lr[0], lr[1], meta)
    } else {
        rasterize_tip_up(u, lr[1], lr[0], [meta[0], meta[2], meta[1]])
    }
}
//TODO: switch inputs to f64
fn rasterize_tip_up(u: &Vector2<i32>, l: &Vector2<i32>, r: &Vector2<i32>, meta: [&VertexMetadata; 3]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    let mut left_bound = u.x as f64;
    let mut right_bound = u.x as f64;

    let left_bound_drift = (l.x - u.x) as f64 / (l.y - u.y) as f64;
    let right_bound_drift = (r.x - u.x) as f64 / (r.y - u.y) as f64;

    let start_y = u.y.round() as i32; // this would normally return f64
    let end_y = l.y.round() as i32;

    let mut points: Vec<(Vector2<i32>, VertexMetadata)> = vec![];

    // TODO: could balance / offset drift for rounding here

    for y in start_y..=end_y {
        for x in (left_bound.round() as i32)..=(right_bound.round() as i32) {
            let v = Vector2::new(x, y);
            let bar_coords = barycentric_coords(&v, u, l, r);
            let new_meta = interpolate_meta(bar_coords, meta);

            let point: (Vector2<i32>, VertexMetadata) = (v, new_meta);
            points.push(point);
        }
        left_bound += left_bound_drift;
        right_bound += right_bound_drift;
    }

    points
}

fn rasterize_tip_down(d: &Vector2<i32>, l: &Vector2<i32>, r: &Vector2<i32>, meta: [&VertexMetadata; 3]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    let mut left_bound = l.x as f64;
    let mut right_bound = r.x as f64;

    let left_bound_drift = (d.x - l.x) as f64 / (d.y - l.y) as f64;
    let right_bound_drift = (d.x - r.x) as f64 / (d.y - r.y) as f64;

    let start_y = l.y.round() as i32; // this would normally return f64
    let end_y = d.y.round() as i32;

    let mut points: Vec<(Vector2<i32>, VertexMetadata)> = vec![];

    // TODO: could balance / offset drift for rounding here

    for y in start_y..=end_y {
        for x in (left_bound.round() as i32)..=(right_bound.round() as i32) {
            let v = Vector2::new(x, y);
            let bar_coords = barycentric_coords(&v, d, l, r);
            let new_meta = interpolate_meta(bar_coords, meta);

            let point: (Vector2<i32>, VertexMetadata) = (v, new_meta);
            points.push(point);
        }
        left_bound += left_bound_drift;
        right_bound += right_bound_drift;
    }

    points
}

fn rasterize_tip_down_unsorted(d: &Vector2<i32>, lr: [&Vector2<i32>; 2], meta: [&VertexMetadata; 3]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    if lr[0].x < lr[1].x {
        rasterize_tip_down(d, lr[0], lr[1], meta)
    } else {
        rasterize_tip_down(d, lr[1], lr[0],[meta[0], meta[2], meta[1]])
    }
}

// https://codeplea.com/triangular-interpolation
fn barycentric_coords(p: &Vector2<i32>, v1: &Vector2<i32>, v2: &Vector2<i32>, v3: &Vector2<i32>) -> [f64; 3] {
    let w1 = ((v2.y - v3.y) as f64 * (p.x - v3.x) as f64 + (v3.x - v2.x) as f64 * (p.y - v3.y) as f64)
                / ((v2.y - v3.y) as f64 * (v1.x - v3.x) as f64 + (v3.x - v2.x) as f64 * (v1.y - v3.y) as f64);
    let w2 = ((v3.y - v1.y) as f64 * (p.x - v3.x) as f64 + (v1.x - v3.x) as f64 * (p.y - v3.y) as f64)
                / ((v2.y - v3.y) as f64 * (v1.x - v3.x) as f64 + (v3.x - v2.x) as f64 * (v1.y - v3.y) as f64);
    let w3 = 1.0 - w1 - w2;
    [w1, w2, w3]
}

fn interpolate_meta(bar_coords: [f64; 3], meta: [&VertexMetadata; 3]) -> VertexMetadata {
    let mut res = VertexMetadata::empty();

    if let (Some(wp1), Some(wp2), Some(wp3)) = (meta[0].world_pos, meta[1].world_pos, meta[2].world_pos) {
        res.world_pos = Some(wp1 * bar_coords[0] + wp2 * bar_coords[1] + wp3 * bar_coords[2]);
    }
    if let (Some(tc1), Some(tc2), Some(tc3)) = (meta[0].texture_coord, meta[1].texture_coord, meta[2].texture_coord) {
        res.texture_coord = Some(tc1 * bar_coords[0] + tc2 * bar_coords[1] + tc3 * bar_coords[2]);
    }
    if let (Some(c1), Some(c2), Some(c3)) = (meta[0].color, meta[1].color, meta[2].color) {
        res.color = Some(c1 * bar_coords[0] + c2 * bar_coords[1] + c3 * bar_coords[2]);
    }
    if let (Some(n1), Some(n2), Some(n3)) = (meta[0].normal, meta[1].normal, meta[2].normal) {
        res.normal = Some(n1 * bar_coords[0] + n2 * bar_coords[1] + n3 * bar_coords[2]);
    }

    res
}