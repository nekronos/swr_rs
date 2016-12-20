
use super::super::math::vector::Vector3;
use super::super::math::vector::Vector2;

use std::f64;

#[derive(Debug)]
pub struct Face {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

impl Face {
    pub fn new(a: u32, b: u32, c: u32) -> Face {
        Face { a: a, b: b, c: c }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vector3>,
    pub faces: Vec<Face>,
    pub texcoords: Vec<Vector2>,
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

impl Mesh {
    pub fn bounds(&self) -> (Vector3, Vector3) {
        if self.vertices.len() > 0 {
            let mut min = *self.vertices.first().unwrap();
            let mut max = *self.vertices.first().unwrap();
            for vert in &self.vertices {
                min = min.min(*vert);
                max = max.max(*vert);
            }
            (min, max)
        } else {
            (Vector3::zero(), Vector3::zero())
        }
    }

    pub fn triangle() -> Mesh {
        Mesh {
            name: "Triangle".to_string(),
            vertices: vec![Vector3::new(-1.0, -1.0, -1.0),
                           Vector3::new(1.0, -1.0, -1.0),
                           Vector3::new(1.0, 1.0, -1.0)],
            faces: vec![Face::new(0, 1, 2)],
            texcoords: Vec::new(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn cube() -> Mesh {
        Mesh {
            name: "Cube".to_string(),
            vertices: vec![Vector3::new(-1.0, -1.0, -1.0),
                           Vector3::new(1.0, -1.0, -1.0),
                           Vector3::new(1.0, 1.0, -1.0),
                           Vector3::new(-1.0, 1.0, -1.0),
                           Vector3::new(-1.0, -1.0, 1.0),
                           Vector3::new(1.0, -1.0, 1.0),
                           Vector3::new(1.0, 1.0, 1.0),
                           Vector3::new(-1.0, 1.0, 1.0)],
            faces: vec![Face::new(0, 1, 2),
                        Face::new(2, 3, 0),
                        Face::new(1, 5, 6),
                        Face::new(6, 2, 1),
                        Face::new(4, 7, 6),
                        Face::new(6, 5, 4),
                        Face::new(0, 3, 7),
                        Face::new(7, 4, 0),
                        Face::new(5, 1, 0),
                        Face::new(0, 4, 5),
                        Face::new(2, 6, 7),
                        Face::new(7, 3, 2)],
            texcoords: Vec::new(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn sphere(pivot: Vector3, radius: f64, slices: usize, stacks: usize) -> Mesh {

        let hori_vertex_count = slices + 1;
        let vert_vertex_count = stacks + 1;

        let body_vertex_count = hori_vertex_count * vert_vertex_count;
        let body_face_count = slices * stacks * 2;

        let mut vertices: Vec<Vector3> = Vec::with_capacity(body_vertex_count);
        let mut faces: Vec<Face> = Vec::with_capacity(body_face_count);

        for j in 0..vert_vertex_count {
            for i in 0..hori_vertex_count {

                let mut u = i as f64 / slices as f64;
                let mut v = j as f64 / stacks as f64;

                u *= 2.0 * f64::consts::PI;
                v = v * f64::consts::PI - f64::consts::PI * 0.5;

                vertices.push(pivot +
                              Vector3::new(v.cos() * u.cos() * radius,
                                           v.cos() * u.sin() * radius,
                                           v.sin() * radius));
            }
        }

        for i in 0..slices {
            for j in 0..stacks {
                faces.push(Face::new(((i + j * hori_vertex_count) + 0) as u32,
                                     ((i + j * hori_vertex_count) + 1) as u32,
                                     ((i + (j + 1) * hori_vertex_count) + 0) as u32));

                faces.push(Face::new(((i + j * hori_vertex_count) + 1) as u32,
                                     ((i + (j + 1) * hori_vertex_count) + 1) as u32,
                                     ((i + (j + 1) * hori_vertex_count) + 0) as u32));
            }
        }

        Mesh {
            name: "Sphere".to_string(),
            vertices: vertices,
            faces: faces,
            texcoords: Vec::new(),
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}
