use crate::pipeline::primitives::{Primitive2, Triangle2, VertexMetadata};
use crate::pipeline::util::vector2i_lerp;
use iced::widget::canvas::path::lyon_path::geom::euclid::num::Round;
use nalgebra::Vector2;

pub fn rasterize(triangle: &Triangle2) -> Vec<(Vector2<i32>, VertexMetadata)> {

    let mut vertices: Vec<Vector2<i32>> = triangle.vertices().iter()
        .map(|v| Vector2::new(v.x.round() as i32, v.y.round() as i32))
        .collect();
    vertices.sort_by(|v1, v2| v1.y.cmp(&(v2.y)));

    if (&vertices[0]).y == (&vertices[2]).y { // all match
        // line, width == 0
        vec![]
    } else if (&vertices[0]).y == (&vertices[1]).y {
        // tip down
        rasterize_tip_down_unsorted(&vertices[2], [&vertices[0], &vertices[1]])
    } else if (&vertices[1]).y == (&vertices[2]).y {
        // tip up
        rasterize_tip_up_unsorted(&vertices[0], [&vertices[1], &vertices[2]])
    } else {
        // both

        let mid_point = vector2i_lerp( &vertices[0], &vertices[2], (vertices[1].y - vertices[0].y) as f64 / (vertices[2].y - vertices[0].y) as f64);
        let mut res1 = rasterize_tip_up_unsorted(&vertices[0], [&vertices[1], &mid_point]);
        let mut res2 = rasterize_tip_down_unsorted(&vertices[2], [&vertices[1], &mid_point]);
        res1.append(&mut res2);
        res1
    }
}

// fn rasterize_tip_down_unsorted(d: &Vector2<i32>, lr: [&Vector2<i32>; 2]) -> Vec<(Vector2<i32>, Vector2<f64>, Vector2<f64>)> {
//     let res = rasterize_tip_up_unsorted(&Vector2::new(d.x, -d.y), [&Vector2::new(lr[0].x, -lr[0].y), &Vector2::new(lr[1].x, -lr[1].y)]);
//     res.iter().map(|(p, t, n)| (Vector2::new(p.x, -p.y), *t, *n)).collect()
// }
fn rasterize_tip_up_unsorted(u: &Vector2<i32>, lr: [&Vector2<i32>; 2]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    if lr[0].x < lr[1].x {
        rasterize_tip_up(u, lr[0], lr[1])
    } else {
        rasterize_tip_up(u, lr[1], lr[0])
    }
}
//TODO: switch inputs to f64
fn rasterize_tip_up(u: &Vector2<i32>, l: &Vector2<i32>, r: &Vector2<i32>) -> Vec<(Vector2<i32>, VertexMetadata)> {
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
            let point: (Vector2<i32>, VertexMetadata) = (Vector2::new(x, y), VertexMetadata::empty());
            points.push(point);
        }
        left_bound += left_bound_drift;
        right_bound += right_bound_drift;
    }

    points
}

fn rasterize_tip_down(d: &Vector2<i32>, l: &Vector2<i32>, r: &Vector2<i32>) -> Vec<(Vector2<i32>, VertexMetadata)> {
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
            let point: (Vector2<i32>, VertexMetadata) = (Vector2::new(x, y), VertexMetadata::empty());
            points.push(point);
        }
        left_bound += left_bound_drift;
        right_bound += right_bound_drift;
    }

    points
}

fn rasterize_tip_down_unsorted(d: &Vector2<i32>, lr: [&Vector2<i32>; 2]) -> Vec<(Vector2<i32>, VertexMetadata)> {
    if lr[0].x < lr[1].x {
        rasterize_tip_down(d, lr[0], lr[1])
    } else {
        rasterize_tip_down(d, lr[1], lr[0])
    }
}