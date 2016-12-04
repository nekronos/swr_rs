
extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

use std::f64;

mod math;

use math::vector::{Vector2, Vector3};
use math::matrix::Matrix4;
use math::quaternion::Quaternion;

#[derive(Debug)]
struct Camera {
    position: Vector3,
    target: Vector3,
    fov: f64,
    zfar: f64,
    znear: f64,
}

#[derive(Debug)]
struct Mesh {
    name: String,
    vertices: Vec<Vector3>,
    position: Vector3,
    rotation: Vector3,
}

impl Mesh {
    fn cube() -> Mesh {
        Mesh {
            name: "Cube".to_string(),
            vertices: vec![
                Vector3::new(-1.0, 1.0, 1.0),
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(-1.0, -1.0, 1.0),
                Vector3::new(-1.0, -1.0, -1.0),
                Vector3::new(-1.0, 1.0, -1.0),
                Vector3::new(1.0, 1.0, -1.0),
                Vector3::new(1.0, -1.0, 1.0),
                Vector3::new(1.0, -1.0, -1.0),
	    	],
            position: Vector3::zero(),
            rotation: Vector3::zero(),
        }
    }
}

#[derive(Debug)]
struct Device {
    width: usize,
    height: usize,
    backbuffer: Box<[u32]>,
}

impl Device {
    fn new(width: usize, height: usize) -> Device {
        Device {
            width: width,
            height: height,
            backbuffer: vec![0; width * height].into_boxed_slice(),
        }
    }

    fn clear(&mut self, clear_color: u32) {
        for i in 0..self.backbuffer.len() {
            self.backbuffer[i] = clear_color
        }
    }

    fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        let offset = (y as usize * self.width) + x as usize;
        self.backbuffer[offset] = color
    }

    fn draw_point(&mut self, point: Vector2) {
        if point.x >= 0.0 && point.y >= 0.0 && point.x < self.width as f64 &&
           point.y < self.height as f64 {
            self.put_pixel(point.x as u32, point.y as u32, 0xff00ccff)
        }
    }

    fn project(&mut self, coord: &Vector3, trans: &Matrix4) -> Vector2 {
        let point = Vector3::transform_coordinate(coord, trans);

        let x = point.x * self.width as f64 + self.width as f64 / 2.0;
        let y = -point.y * self.height as f64 + self.height as f64 / 2.0;

        Vector2::new(x, y)
    }

    fn render(&mut self, camera: &Camera, meshes: &Vec<&Mesh>) {
        let view_mat = Matrix4::look_at_lh(camera.position, camera.target, Vector3::unit_y());
        let projection_mat =
            Matrix4::perspective_rh(camera.fov, self.width as f64 / self.height as f64, camera.znear, camera.zfar);
        for mesh in meshes {

            let world_mat = Matrix4::rotation(Quaternion::from_euler_angle(mesh.rotation)) *
                            Matrix4::translation(mesh.position);
            let transform_mat = world_mat * view_mat * projection_mat;

            for vertex in &mesh.vertices {
                let p = self.project(vertex, &transform_mat);
                self.draw_point(p)
            }

        }

    }
}

fn main() {

    let mut device = Device::new(WIDTH, HEIGHT);

    let mut window = Window::new("SWR_RS",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions { scale: minifb::Scale::X2, ..Default::default() })
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let camera = Camera {
        position: Vector3::new(0.0, 0.0, 10.0),
        target: Vector3::zero(),
        fov: 45.0 * f64::consts::PI / 180.0,
        znear: 0.01,
        zfar: 1.0,
    };

    let mut mesh = Mesh::cube();

    let sleep_time = std::time::Duration::from_millis(16);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        {
            let mut meshes = Vec::new();
            meshes.push(&mesh);

            device.clear(0xff111111);
            device.render(&camera, &meshes);
        }

        mesh.rotation = mesh.rotation + Vector3::new(0.01, 0.01, 0.01);

        window.update_with_buffer(&device.backbuffer);

        let elapsed = now.elapsed();
        if sleep_time > elapsed {
            let sleep = sleep_time - elapsed;
            std::thread::sleep(sleep)
        }
    }

}
