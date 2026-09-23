use std::{cmp::min, num::IntErrorKind};

use glam::Vec3;

use crate::{
    lights,
    obj::{self, Obj3d, Triangle},
};

#[derive(Clone, Copy)]
pub struct Light {
    pub pos: Vec3,
    pub color: (u8, u8, u8),
    pub light_type: LightType,
    pub ratio: f32,
    pub k: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LightType {
    Directional,
    Point,
}

///functions for Lights
impl Light {
    pub fn new_point_light(pos: Vec3, color: (u8, u8, u8), ratio: f32, k: f32) -> Self {
        Self {
            pos: pos,
            color: color,
            ratio: ratio * ratio, //radio al cuadrado para calcular la distancia
            k: k,
            light_type: LightType::Point,
        }
    }

    //hacer una sola funcion new

    pub fn new_directional_light(pos: Vec3, color: (u8, u8, u8)) -> Self {
        Self {
            pos: pos,
            color: color,
            ratio: 0.0,
            k: 0.0,
            light_type: LightType::Directional,
        }
    }
}

pub fn centroide(tri: &Triangle) -> Vec3 {
    let x = (tri.p1.x + tri.p2.x + tri.p3.x) / 3.0;
    let y = (tri.p1.y + tri.p2.y + tri.p3.y) / 3.0;
    let z = (tri.p1.z + tri.p2.z + tri.p3.z) / 3.0;

    Vec3::new(x, y, z)
}

//anadir atenuacion por distancia para las luces puntuales
///Calculate the difusse light of an obj
pub fn difusse_light(
    l: Light,
    tri: Triangle,
    color: (u8, u8, u8),
    ambiente: f32,
    distan2: f32,
) -> (f32, f32, f32) {
    let atenuacion = if l.light_type == LightType::Point {
        calculate_attenuation(distan2, l.ratio, l.k).min(1.0)
    } else {
        1.0
    };

    let final_c;

    let arista1 = tri.p1 - tri.p2;
    let arista2 = tri.p2 - tri.p3;

    let normal = arista2.cross(arista1).normalize();
    let light_dir = (l.pos - centroide(&tri)).normalize();

    let intensidad = normal.dot(light_dir).max(0.0);

    let factor;

    factor = ambiente + (1.0 - ambiente) * intensidad * atenuacion.min(1.0);

    //aqui                  / 255.0
    let mut r =
        ((color.0 as f32 / 255.0) * (l.color.0 as f32 / 255.0) * factor * 255.0).clamp(0.0, 255.0);
    let mut g =
        ((color.1 as f32 / 255.0) * (l.color.1 as f32 / 255.0) * factor * 255.0).clamp(0.0, 255.0);
    let mut b =
        ((color.2 as f32 / 255.0) * (l.color.2 as f32 / 255.0) * factor * 255.0).clamp(0.0, 255.0);

    (r, g, b) = (r, g, b);

    final_c = (r, g, b);

    final_c
    //}

    //final_c
}

//1 / (d^2 + K)
pub fn calculate_attenuation(d2: f32, radius2: f32, k: f32) -> f32 {
    let ventana = 1.0 - ((d2.powi(2)) / radius2); // .powi(2)
    (ventana * ventana) / (d2 + k)
}

pub fn calculate_all_scene_lights(
    lights: &Vec<Light>,
    base_color: (u8, u8, u8),
    tri: Triangle,
    ambiente: f32,
) -> (u8, u8, u8) {
    // arranca con la componente ambiental, aplicada UNA vez sobre el color base
    let mut acc = (
        base_color.0 as f32 * ambiente,
        base_color.1 as f32 * ambiente,
        base_color.2 as f32 * ambiente,
    );

    let centroide = centroide(&tri);

    for light in lights {
        let mut distance = 0.0;
        if light.light_type != LightType::Directional {
            let dis = light.pos - centroide;
            //                      plus
            distance = dis.x * dis.x + dis.y * dis.y + dis.z * dis.z;
            if distance >= light.ratio {
                continue;
            }
        }

        let (r, g, b) = difusse_light(*light, tri, base_color, ambiente, distance);
        acc.0 += r;
        acc.1 += g;
        acc.2 += b;
    }

    (
        acc.0.clamp(0.0, 255.0) as u8,
        acc.1.clamp(0.0, 255.0) as u8,
        acc.2.clamp(0.0, 255.0) as u8,
    )
}
