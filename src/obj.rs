use glam::prelude::*;

use crate::{lights::centroide, obj};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        Self {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }
}

#[derive(Clone)]
pub struct Obj3d {
    pub color: (u8, u8, u8),
    pub pos: Vec3,
    pub scale: Vec3,
    pub rotate: Vec3,

    pub mesh: Vec<Triangle>,
}

impl Obj3d {
    pub fn new_solid_obj(
        mesh: Vec<Triangle>,
        pos: Vec3,
        scale: Vec3,
        rotate: Vec3,
        color: (u8, u8, u8),
    ) -> Self {
        Self {
            color: color,
            scale: scale,
            rotate: rotate,

            pos: pos,
            mesh: mesh,
        }
    }

    pub fn scale(&self, scale_factor: Vec3, tri: &Triangle) -> Triangle {
        let mut the_scale = scale_factor;

        if the_scale.x == 0.0 && the_scale.y == 0.0 && the_scale.z == 0.0 {
            the_scale = Vec3::new(1.0, 1.0, 1.0);
        }

        let scale_matrix = Mat4::from_scale(the_scale);

        let p1 = scale_matrix.transform_point3(tri.p1);
        let p2 = scale_matrix.transform_point3(tri.p2);
        let p3 = scale_matrix.transform_point3(tri.p3);

        Triangle::new(p1, p2, p3)
    }

    fn trasladar(&mut self, p_final: Vec3, velocity: f32, dt: f32) {
        if !(self.pos.x.round() >= p_final.x.round()
            && self.pos.y.round() >= p_final.y.round()
            && self.pos.z.round() >= p_final.z.round())
        {
            self.pos = (self.pos + p_final) + velocity * dt;
        }
    }

    pub fn rotate(&mut self, dt: f32) {
        let rot1 = self.rotate.x * dt;
        let rot2 = self.rotate.y * dt;
        let rot3 = self.rotate.z * dt;

        let rotacion: Mat4 = Mat4::from_euler(glam::EulerRot::XYZ, rot1, rot2, rot3);

        for triangle in &mut self.mesh {
            triangle.p1 = rotacion.transform_point3(triangle.p1);
            triangle.p2 = rotacion.transform_point3(triangle.p2);
            triangle.p3 = rotacion.transform_point3(triangle.p3);
        }
    }
}
