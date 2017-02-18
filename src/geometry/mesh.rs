
use super::super::math::vector::Vector3;

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
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn shell(inner_radius: f64,
                 final_shell_radius: f64,
                 height: f64,
                 number_of_spirals: u32,
                 slices: usize,
                 stacks: usize)
                 -> Mesh {
        let vertices_per_row = slices + 1;
        let vertices_per_col = stacks + 1;

        let vertex_count = vertices_per_row * vertices_per_col;
        let mut vertices = Vec::with_capacity(vertex_count as usize);

        let vertical_angle = (f64::consts::PI * 2.0) / slices as f64;
        let horizontal_angle = (f64::consts::PI * 2.0) / stacks as f64;

        let n = number_of_spirals as f64;
        let a = final_shell_radius as f64;
        let b = height as f64;
        let c = inner_radius as f64;

        for v in 0..vertices_per_col {
            let t = vertical_angle * v as f64;

            for h in 0..vertices_per_row {
                let s = horizontal_angle * h as f64;

                let t2pi = t / (f64::consts::PI * 2.0);
                let cos_nt = (n * t).cos();
                let cos_s = s.cos();
                let sin_nt = (n * t).sin();
                let sin_s = s.sin();

                let x = a * (1.0 - t2pi) * cos_nt * (1.0 + cos_s) + c * cos_nt;
                let y = a * (1.0 - t2pi) * sin_nt * (1.0 + cos_s) + c * sin_nt;
                let z = b * t2pi + a * (1.0 - t2pi) * sin_s;

                vertices.push(Vector3::new(x, y, z))
            }
        }

        let face_count = slices * stacks * 6;
        let mut faces = Vec::with_capacity(face_count as usize);

        for v in 0..slices {
            for h in 0..stacks {
                let lt = (h + v * vertices_per_row) as u32;
                let rt = ((h + 1) + v * vertices_per_row) as u32;
                let lb = (h + (v + 1) * vertices_per_row) as u32;
                let rb = ((h + 1) + (v + 1) * vertices_per_row) as u32;

                faces.push(Face::new(lt, rt, lb));
                faces.push(Face::new(rt, rb, lb))
            }
        }

        Mesh {
            name: "Shell".to_string(),
            vertices: vertices,
            faces: faces,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn torus(radius: f64, ring_radius: f64, sides: u32, rings: u32) -> Mesh {
        let vertices_per_row = sides + 1;
        let vertices_per_col = rings + 1;

        let vertex_count = vertices_per_row * vertices_per_col;
        let mut vertices = Vec::with_capacity(vertex_count as usize);

        let vertical_angle = (f64::consts::PI * 2.0) / rings as f64;
        let horizontal_angle = (f64::consts::PI * 2.0) / sides as f64;

        for v in 0..vertices_per_col {
            let theta = vertical_angle * v as f64;
            for h in 0..vertices_per_row {
                let phi = horizontal_angle * h as f64;
                let x = theta.cos() * (radius + ring_radius * phi.cos());
                let y = theta.sin() * (radius + ring_radius * phi.cos());
                let z = ring_radius * phi.sin();
                vertices.push(Vector3::new(x, y, z))
            }
        }

        let face_count = sides * rings * 6;
        let mut faces = Vec::with_capacity(face_count as usize);

        for v in 0..rings {
            for h in 0..sides {
                let lt = h + v * vertices_per_row;
                let rt = (h + 1) + v * vertices_per_row;
                let lb = h + (v + 1) * vertices_per_row;
                let rb = (h + 1) + (v + 1) * vertices_per_row;

                faces.push(Face::new(lt, rt, lb));
                faces.push(Face::new(rt, rb, lb))
            }
        }

        Mesh {
            name: "Torus".to_string(),
            vertices: vertices,
            faces: faces,
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn tetrahedron(radius: f64) -> Mesh {
        let angle = 2.0 * f64::consts::PI / 3.0;
        let peak = (2.0 * radius).sqrt() / 2.0;
        let mut vertices = Vec::new();
        for i in 0..3 {
            let t = radius * (i as f64 * angle);
            let x = t.cos();
            let y = t.sin();
            vertices.push(Vector3::new(x, y, -peak))
        }
        vertices.push(Vector3::new(0.0, 0.0, peak));
        Mesh {
            name: "Tetrahedron".to_string(),
            vertices: vertices,
            faces: vec![
                Face::new(0, 1, 2),
                Face::new(0, 1, 3),
                Face::new(1, 2, 3),
                Face::new(2, 0, 3),
            ],
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }

    pub fn octahedron(radius: f64) -> Mesh {
        let angle = f64::consts::PI / 2.0;
        let mut vertices = Vec::new();
        for i in 0..4 {
            let t = radius * (i as f64 * angle);
            let x = t.cos();
            let y = t.sin();
            vertices.push(Vector3::new(x, y, 0.0))
        }
        let top = Vector3::new(0.0, 0.0, radius);
        vertices.push(top);
        let bot = Vector3::new(0.0, 0.0, -radius);
        vertices.push(bot);
        Mesh {
            name: "Octahedron".to_string(),
            vertices: vertices,
            faces: vec![
                Face::new(0, 1, 4),
                Face::new(1, 2, 4),
                Face::new(2, 3, 4),
                Face::new(3, 0, 4),
                Face::new(0, 1, 5),
                Face::new(1, 2, 5),
                Face::new(2, 3, 5),
                Face::new(3, 0, 5),
            ],
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
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::one(),
        }
    }
}
