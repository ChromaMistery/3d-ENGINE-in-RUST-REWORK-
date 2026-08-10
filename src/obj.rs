use std::ffi::CStr;

use crate::{obj, render};
use glam::prelude::*;

#[derive(Clone)]
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
    pub letter: char,
    pub color: (u8, u8, u8),
    pub pos: Vec3,
    pub rotate: Vec3,
    pub mesh: Vec<Triangle>,
}

impl Obj3d {
    pub fn new_ascii_obj(
        mesh: Vec<Triangle>,
        pos: Vec3,
        rotate: Vec3,
        letter: char,
        color: (u8, u8, u8),
    ) -> Self {
        Self {
            letter: letter,
            color: color,
            rotate: rotate,
            pos: pos,
            mesh: mesh,
        }
    }

    pub fn new_solid_obj(
        mesh: Vec<Triangle>,
        pos: Vec3,
        rotate: Vec3,
        color: (u8, u8, u8),
    ) -> Self {
        Self {
            letter: '█',
            color: color,
            rotate: rotate,
            pos: pos,
            mesh: mesh,
        }
    }

    pub fn trasladar(&mut self, p_final: Vec3, velocity: f32, dt: f32) {
        if !(self.pos.x.round() >= p_final.x.round()
            && self.pos.y.round() >= p_final.y.round()
            && self.pos.z.round() >= p_final.z.round())
        {
            self.pos = (self.pos + p_final) + velocity * dt;
        }
    }

    pub fn rotate(&mut self, velocity: Vec3, dt: f32) {
        let rot1 = self.rotate.x + velocity.x * dt;
        let rot2 = self.rotate.y + velocity.y * dt;
        let rot3 = self.rotate.z + velocity.z * dt;

        let rotacion: Mat4 = Mat4::from_euler(
            glam::EulerRot::XYZ,
            rot1, // radianes, rotación en X
            rot2, // radianes, rotación en Y
            rot3, // radianes, rotación en Z (esto sería tu CCW en 2D si ves desde +Z)
        );

        for triangle in &mut self.mesh {
            triangle.p1 = rotacion.transform_point3(triangle.p1);
            triangle.p2 = rotacion.transform_point3(triangle.p2);
            triangle.p3 = rotacion.transform_point3(triangle.p3);
        }
    }
}

/*
camara proceso

restar la pos de la camara = p relativo

rotar ese punto relativo

y despues proyectarlo


yaw -> rotacion sobre el eje y

pitch -> rotacion sobre el eje x

como inclinar la cabezal pero no sera necesario por ahora
roll -> rotacion sobre el eje z
*/
//CAM
pub struct CAM {
    pub pos: Vec3,
    pub rotation: Mat4,
    pub d_fov: f32,
    pub yaw: f32,
    pub picth: f32,
    pub sensivity: f32,
    pub move_velo: f32,
    pub forward: Vec3,
    pub right: Vec3,
}
//CAM

impl CAM {
    pub fn new(
        pos: Vec3,
        d_fov: f32,
        yaw: f32,
        picth: f32,
        sensivity: f32,
        move_velo: f32,
    ) -> Self {
        let rot = Mat4::from_euler(
            glam::EulerRot::XYZ,
            -yaw,   // x
            -picth, // y
            0.0,    // z
        );

        Self {
            pos: (pos),
            rotation: rot,
            d_fov: (d_fov),
            yaw: (yaw),
            picth: (picth),
            sensivity: sensivity,
            move_velo: (move_velo),
            forward: Vec3::new(picth.cos() * yaw.sin(), 0.0, yaw.sin()),
            right: Vec3::new(yaw.cos(), 0.0, -yaw.sin()),
        }
    }

    pub fn update_rotation(&mut self) {
        self.picth = self
            .picth
            .clamp((-88.0_f32).to_radians(), (88.0_f32).to_radians());
        self.rotation = Mat4::from_euler(glam::EulerRot::XYZ, -self.picth, -self.yaw, 0.0);
        self.forward = Vec3::new(
            self.picth.cos() * self.yaw.sin(),
            0.0,
            self.picth.cos() * self.yaw.cos(),
        );
        self.right = Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin());
    }
}

/*aproximacion imperfecta
pub fn orbit_rotate(pivot: &mut Obj3d, objs: Vec<&mut Obj3d>, rot: Vec3, dt: f32) {
    let rot1 = pivot.rotate.x + rot.x * dt;
    let rot2 = pivot.rotate.y + rot.y * dt;
    let rot3 = pivot.rotate.z + rot.z * dt;

    let rotacion: Mat4 = Mat4::from_euler(
        glam::EulerRot::XYZ,
        rot1, // x
        rot2, // y
        rot3, // z
    );

    for triangle in &mut pivot.mesh {
        triangle.p1 = rotacion.transform_point3(triangle.p1);
        triangle.p2 = rotacion.transform_point3(triangle.p2);
        triangle.p3 = rotacion.transform_point3(triangle.p3);
    }

    for obj in objs {
        obj.pos = rotacion.transform_point3(obj.pos + pivot.rotate * dt);
    }
}
*/
