use glam::prelude::*;

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

#[derive(Clone, Copy)]
pub struct CAM {
    pub movent: bool,
    pub fly: bool,
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
        movee: bool,
        fly: bool,
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
            movent: (movee),
            fly: (fly),
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
