use nalgebra::{Vector2, Vector3};

pub trait Primitive2 {
    fn vertices(&self) -> Vec<Vector2<f64>>;
    fn texture_coords(&self) -> Vec<Vector2<f64>>;
    fn normals(&self) -> Vec<Vector2<f64>>;
}
pub trait Primitive3 {
    fn vertices(&self) -> Vec<Vector3<f64>>;
    fn texture_coords(&self) -> Vec<Vector2<f64>>;
    fn normals(&self) -> Vec<Vector3<f64>>;
}

// (TODO): generic?

#[derive(Debug,Copy,Clone)]
pub struct Triangle2 {
    pub v: [Vector2<f64>; 3],
    pub t: [Vector2<f64>; 3],
    pub n: [Vector2<f64>; 3]
}
#[derive(Debug,Copy,Clone)]
pub struct Triangle3 {
    pub v: [Vector3<f64>; 3],
    pub t: [Vector2<f64>; 3],
    pub n: [Vector3<f64>; 3]
}

impl Primitive2 for Triangle2 {
    fn vertices(&self) -> Vec<Vector2<f64>> { self.v.to_vec() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { self.t.to_vec() }
    fn normals(&self) -> Vec<Vector2<f64>> { self.n.to_vec() }
}
impl Primitive3 for Triangle3 {
    fn vertices(&self) -> Vec<Vector3<f64>> { self.v.to_vec() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { self.t.to_vec() }
    fn normals(&self) -> Vec<Vector3<f64>> { self.n.to_vec() }
}

#[derive(Debug,Copy,Clone)]
pub struct Line2 {
    pub v: [Vector2<f64>; 2],
}
#[derive(Debug,Copy,Clone)]
pub struct Line3 {
    pub v: [Vector3<f64>; 2],
}

impl Primitive2 for Line2 {
    fn vertices(&self) -> Vec<Vector2<f64>> { self.v.to_vec() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { vec![] }
    fn normals(&self) -> Vec<Vector2<f64>> { vec![] }
}
impl Primitive3 for Line3 {
    fn vertices(&self) -> Vec<Vector3<f64>> { self.v.to_vec() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { vec![] }
    fn normals(&self) -> Vec<Vector3<f64>> { vec![] }
}

#[derive(Debug,Clone)]
pub struct Polygon2 {
    pub v: Vec<Vector2<f64>>,
    pub t: Vec<Vector2<f64>>,
    pub n: Vec<Vector2<f64>>
}
#[derive(Debug,Clone)]
pub struct Polygon3 {
    pub v: Vec<Vector3<f64>>,
    pub t: Vec<Vector2<f64>>,
    pub n: Vec<Vector3<f64>>
}

impl Primitive2 for Polygon2 {
    fn vertices(&self) -> Vec<Vector2<f64>> { self.v.clone() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { self.t.clone() }
    fn normals(&self) -> Vec<Vector2<f64>> { self.n.clone() }
}
impl Primitive3 for Polygon3 {
    fn vertices(&self) -> Vec<Vector3<f64>> { self.v.clone() }
    fn texture_coords(&self) -> Vec<Vector2<f64>> { self.t.clone() }
    fn normals(&self) -> Vec<Vector3<f64>> { self.n.clone() }
}

// equivalent to uniforms in shaders
#[derive(Debug,Copy,Clone)]
pub struct VertexMetadata {
    pub world_pos: Option<Vector3<f64>>, // position in world space, for lighting calculations
    pub texture_coord: Option<Vector2<f64>>,
    pub normal: Option<Vector3<f64>>
}
impl VertexMetadata {
    pub fn empty() -> Self { Self { world_pos: None, texture_coord: None, normal: None } }
}
