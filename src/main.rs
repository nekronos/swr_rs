
extern crate minifb;

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 1200;
const HEIGHT: usize = 720;

use std::f64;

mod math;
mod geometry;

use math::vector::{Vector2, Vector3};
use math::matrix::Matrix4;
use math::quaternion::Quaternion;

use geometry::mesh::{Mesh, Face};

#[derive(Debug)]
struct Camera {
    position: Vector3,
    target: Vector3,
    fov: f64,
    zfar: f64,
    znear: f64,
}

#[derive(Debug)]
struct Device {
    width: usize,
    height: usize,
    backbuffer: Box<[u32]>,
}

fn round(x: f64) -> f64 {
    (x + 0.5).round()
}

fn fpart(x: f64) -> f64 {
    x.fract().abs()
}

fn rfpart(x: f64) -> f64 {
    1.0 - fpart(x)
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
            self.put_pixel(point.x as u32, point.y as u32, 0xffff2222)
        }
    }

    fn plot(&mut self, x: i32, y: i32, c: f64) {

        let c = (255.0 * c) as u32;
        let c = 255 - c;
        let color = (0xff << 24) | (c << 16) | (c << 8) | (c);

        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.put_pixel(x as u32, y as u32, color)
        }
    }

    fn draw_line(&mut self, p1: Vector2, p2: Vector2) {
        let len = (p1 - p2).length().abs();

        for i in 0..len as u32 {
            self.draw_point(Vector2::lerp(p1, p2, i as f64 / len));
        }
    }

    fn draw_line_aa(&mut self, p1: Vector2, p2: Vector2) {
        let x0 = p1.x;
        let x1 = p2.x;
        let y0 = p1.y;
        let y1 = p2.y;

        let steep = {
            (y1 - y0).abs() > (x1 - x0).abs()
        };

        let (x0, y0, x1, y1) = if steep {
            (y0, x0, y1, x1)
        } else if x0 > x1 {
            (x1, y1, x0, y0)
        } else {
            (x0, y0, x1, y1)
        };

        let dx = x1 - x0;
        let dy = y1 - y0;

        let slope = dy / dx;
        let xend = round(x0);
        let yend = y0 + slope * (xend - x0);
        let xgap = rfpart(x0 + 0.5);
        let xpxl1 = xend as i32;
        let ypxl1 = yend as i32;

        if steep {
            self.plot(ypxl1, xpxl1, rfpart(yend) * xgap);
            self.plot(ypxl1 + 1, xpxl1, fpart(yend) * xgap);
        } else {
            self.plot(xpxl1, ypxl1, rfpart(yend) * xgap);
            self.plot(xpxl1, ypxl1 + 1, fpart(yend) * xgap);
        }

        let mut intery = yend + slope;

        let xend = round(x1);
        let yend = y1 + slope * (xend - x1);
        let xgap = fpart(x1 + 0.5);
        let xpxl2 = xend as i32;
        let ypxl2 = yend as i32;

        if steep {
            self.plot(ypxl2, xpxl2, rfpart(yend) * xgap);
            self.plot(ypxl2 + 1, xpxl2, fpart(yend) * xgap);
        } else {
            self.plot(xpxl2, ypxl2, rfpart(yend) * xgap);
            self.plot(xpxl2, ypxl2 + 1, fpart(yend) * xgap);
        }

        if steep {
            for x in (xpxl1 + 1)..(xpxl2 - 1) {
                self.plot(intery as i32, x, rfpart(intery));
                self.plot(intery as i32 + 1, x, fpart(intery));
                intery = intery + slope
            }
        } else {
            for x in (xpxl1 + 1)..(xpxl2 - 1) {
                self.plot(x, intery as i32, rfpart(intery));
                self.plot(x, intery as i32 + 1, fpart(intery));
                intery = intery + slope
            }
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
        let projection_mat = Matrix4::perspective_rh(camera.fov,
                                                     self.width as f64 / self.height as f64,
                                                     camera.znear,
                                                     camera.zfar);
        for mesh in meshes {

            let world_mat = Matrix4::rotation(Quaternion::from_euler_angle(mesh.rotation)) *
                            Matrix4::translation(mesh.position);
            let transform_mat = world_mat * view_mat * projection_mat;

            for face in &mesh.faces {
                let p1 = self.project(&mesh.vertices[face.a as usize], &transform_mat);
                let p2 = self.project(&mesh.vertices[face.b as usize], &transform_mat);
                let p3 = self.project(&mesh.vertices[face.c as usize], &transform_mat);
                self.draw_line_aa(p1, p2);
                self.draw_line_aa(p2, p3);
                self.draw_line_aa(p3, p1);
            }

        }

    }
}

fn main() {

    let mut device = Device::new(WIDTH, HEIGHT);

    let mut window = Window::new("SWR_RS",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions { scale: minifb::Scale::X1, ..Default::default() })
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

    let mut mesh = Mesh::sphere(Vector3::zero(), 1.0, 24, 24);

    let sleep_time = std::time::Duration::from_millis(16);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::Instant::now();

        {
            let mut meshes = Vec::new();
            meshes.push(&mesh);

            device.clear(0xffeeeeee);
            device.render(&camera, &meshes);
        }

        mesh.rotation = mesh.rotation + Vector3::new(0.005, 0.005, 0.005);

        window.update_with_buffer(&device.backbuffer);

        let elapsed = now.elapsed();
        if sleep_time > elapsed {
            let sleep = sleep_time - elapsed;
            std::thread::sleep(sleep)
        }

    }
}
