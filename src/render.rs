use bresenham::Bresenham;
use crossterm::{cursor, queue, style::Print};

use std::{
    clone, f32,
    fmt::{Write, format},
    io::{self},
    os::raw::c_short,
    result, thread, time,
};

use std::cmp::{max, min};

use crate::{
    cam::{self, CAM},
    engine_setup::EngineConf,
    lights::{calculate_all_scene_lights, difusse_light},
    obj::Triangle,
};
use crate::{
    lights::Light,
    obj::{self, Obj3d},
};
use glam::{camera, prelude::*};
use std::time::{Duration, Instant};

/*
1. Posición del cursor   →  \x1B[{fila};{columna}H
2. Color (fg y/o bg)     →  \x1b[38;2;R;G;Bm   (foreground)
                             \x1b[48;2;R;G;Bm   (background)
3. El carácter en sí     →  letter
4. Reset (opcional pero recomendable)  →  \x1b[0m


voy poner en todas las funciones \x1b[0m para evitar bugs, ya cuando el pepiline grafico funcione correctamente lo quito para mas rendimiento
*/
#[derive(Clone)]
pub struct FrameBuffer {
    pub widht: usize,
    pub height: usize,
    pub frame: String,
    pub z_buffer: Vec<Vec<f32>>,
}
//esto es para meterle vainas
// write!(buf, "\x1b[38;2;{};{};{}m█", x as u8, y as u8, 0).unwrap(); es para valores dinamicos
//println para imprimir todo el frame, ahora toca adaptar todo

//buf.push_str("\x1b[0m");  // string fijo, ya conocido de antemano or push -> char

//FIXEAR EL ASPECT RATIO

impl FrameBuffer {
    pub fn new(w: usize, h: usize) -> Self {
        let capacidad = w * h * 60;

        Self {
            widht: w,
            height: h,
            frame: String::with_capacity(capacidad),
            z_buffer: vec![vec![f32::INFINITY; w]; h],
        }
    }

    pub fn update_size(&mut self, w: usize, h: usize) {
        if self.widht != w || self.height != h {
            self.widht = w;
            self.height = h;
            self.frame = String::with_capacity(w * h * 25);
            self.z_buffer = vec![vec![f32::INFINITY; w]; h];
        }
    }

    pub fn print_frame(&self) {
        println!("{}", self.frame);
    }

    pub fn clear(&mut self) {
        self.frame.clear();
        self.frame.push_str("\x1B[2J\x1B[H");
        self.frame.push_str("\x1b[H");

        for row in &mut self.z_buffer {
            row.fill(f32::INFINITY);
        }
    }

    //screen traducir ndc a pantalla
    pub fn screen(&self, v: Vec2) -> Vec2 {
        let aspect_ratio = 0.5; //es clave
        Vec2::new(
            ((v.x * aspect_ratio) + 1.0) / 2.0 * self.widht as f32,
            (1.0 - (v.y + 1.0) / 2.0) * self.height as f32,
        )
    }
}

pub mod cursor_options {
    pub fn ocultar_cursor() {
        println!("\x1b[?25l:");
    }

    pub fn mostrar_cursor() {
        println!("\x1b[?25h:");
    }
}

///dentro del triangulo == edge fn
fn signed_area(v1: Vec2, v2: Vec2, v3: Vec2) -> f32 {
    (v2.x - v1.x) * (v3.y - v1.y) - (v3.x - v1.x) * (v2.y - v1.y)
}

/// devuelve los vertices reordenados para garantizar orden antihorario CCW
/// asume sistema de coordenadas co hacia arriba
fn ensure_ccw(v1: Vec2, v2: Vec2, v3: Vec2) -> (Vec2, Vec2, Vec2) {
    if signed_area(v1, v2, v3) < 0.0 {
        //cw horario -> resultado negativo si esta correcto
        //cww antihorario -> resutado positivo si esta correcto
        // estaba en CW -> intercambiar dos vrtices lo invierte a CCW
        (v1, v3, v2)
    } else {
        (v1, v2, v3)
    }
}

pub fn draw_ascii_line(
    buff: &mut FrameBuffer,
    v: Vec2,
    v2: Vec2,
    letter: char,
    color: (u8, u8, u8),
) {
    if !(v.x > buff.widht as f32 || v.y > buff.height as f32)
        && !(v2.x > buff.widht as f32 || v2.y > buff.height as f32)
    {
        //let aspect_ratio = 0.5;
        let v = buff.screen(Vec2::new(v.x, v.y));
        let v2 = buff.screen(Vec2::new(v2.x, v2.y));
        //v.x = v.x * aspect_ratio;
        //v2.x = v2.x * aspect_ratio;

        for (x, y) in Bresenham::new(
            (v.x.round() as isize, v.y.round() as isize),
            (v2.x.round() as isize, v2.y.round() as isize),
        ) {
            write!(
                buff.frame,
                "\x1B[{};{}H\x1b[38;2;{};{};{}m{}",
                y, x, color.0, color.1, color.2, letter
            )
            .unwrap();
        }
    }
}

pub fn draw_point(buff: &mut FrameBuffer, v: Vec2, letter: char, color: (u8, u8, u8)) {
    //let aspect_ratio = 0.5;

    let v0 = buff.screen(v.clone());
    //v0.x = v.x * aspect_ratio;

    write!(
        buff.frame,
        "\x1B[{};{}H\x1b[38;2;{};{};{}m{} \n",
        v0.y, v0.x, color.0, color.1, color.2, letter
    )
    .unwrap();
}

pub fn write(buff: &mut FrameBuffer, coords: Vec2, words: String, color: (u8, u8, u8)) {
    //let aspect_ratio = 2.0;

    let v0 = buff.screen(coords.clone());
    // v0.x = v.x //* aspect_ratio;

    write!(
        buff.frame,
        "\x1B[{};{}H\x1b[38;2;{};{};{}m{}\n",
        v0.y.round(),
        v0.x.round(),
        color.0,
        color.1,
        color.2,
        words
    )
    .unwrap();
}

pub fn draw_ascii_triangle(
    buff: &mut FrameBuffer,
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
    letter: char,
    color: (u8, u8, u8),
) {
    let (v1, v2, v3) = ensure_ccw(v1, v2, v3);

    let v1_inside = v1.x.round() > buff.widht as f32 || v1.y.round() > buff.height as f32;
    let v2_inside = v2.x.round() > buff.widht as f32 || v2.y.round() > buff.height as f32;
    let v3_inside = v3.x.round() > buff.widht as f32 || v3.y.round() > buff.height as f32;

    if !(v1_inside || v2_inside || v3_inside) {
        draw_ascii_line(buff, v1, v2, letter, color);
        draw_ascii_line(buff, v2, v3, letter, color);
        draw_ascii_line(buff, v3, v1, letter, color);
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

//proyeccion 3d
pub fn proyect_3d(v: &Vec3, cam: &mut cam::CAM) -> Vec2 {
    //d*x\z
    //d*y\z

    //tengo que contar tambien la rotacion de la camara

    let real_d: f32;
    let minimun_d = 0.1;

    if minimun_d > cam.d_fov {
        real_d = minimun_d;
    } else {
        real_d = cam.d_fov;
    }

    let x = (v.x / v.z) * real_d;
    let y = (v.y / v.z) * real_d;

    Vec2::new(x, y)
}

//cada funcion de aqui debe de ser adaptada para recibir el engine
pub fn render(context: &mut EngineConf, l: Light) {
    //difusse light
    //revertir esta monda, eliminar esa funcion de procesar tris, y meter todas las tranformaciones directamente aqui
    //if obj.type == light obj.light . append in lights of scene
    for obj in &mut context.escene.objs {
        for triangle in &obj.mesh {
            let traslacion = Mat4::from_translation(obj.pos);

            let tri = Obj3d::scale(&obj, obj.scale, &triangle);

            let p1 = traslacion.transform_point3(tri.p1);
            let p2 = traslacion.transform_point3(tri.p2);
            let p3 = traslacion.transform_point3(tri.p3);

            let r1 = obtener_p_relativo(p1, context.cam);
            let r2 = obtener_p_relativo(p2, context.cam);
            let r3 = obtener_p_relativo(p3, context.cam);

            if r1.z < context.z_near || r2.z < context.z_near || r3.z < context.z_near {
                continue;
            }

            if r1.z > context.z_far && r2.z > context.z_far && r3.z > context.z_far {
                continue;
            }

            //esto no es physics accuarate

            //let mut char = ' ';
            //for light in &context.escene.lights {

            if es_cara_trasera_cw(&p1, &p2, &p3, &context.cam.pos) {
                let cf = calculate_all_scene_lights(
                    &context.escene.lights,
                    obj.color,
                    Triangle::new(p1, p2, p3),
                    context.escene.ambient_light,
                );
                draw_full_triangle(cf, &mut context.frame_buffer, &mut context.cam, r1, r2, r3);
            }
        }
    }
}

fn obtener_p_relativo(v: Vec3, cam: cam::CAM) -> Vec3 {
    let camera_space = cam.rotation.transform_point3(v - cam.pos);

    camera_space
}

pub fn draw_full_triangle(
    color: (u8, u8, u8),
    buff: &mut FrameBuffer,
    cam: &mut cam::CAM,
    v1: Vec3,
    v2: Vec3,
    v3: Vec3,
) {
    //let aspect_ratio = 0.5;
    let z1 = v1.z;
    let z2 = v2.z;
    let z3 = v3.z;

    let v1_2d = proyect_3d(&v1, cam);
    let v2_2d = proyect_3d(&v2, cam);
    let v3_2d = proyect_3d(&v3, cam);

    //  NDC -> espacio de pantalla, una sola vez
    let v1_s = buff.screen(v1_2d);
    let v2_s = buff.screen(v2_2d);
    let v3_s = buff.screen(v3_2d);

    let (v1_s, v2_s, v3_s) = ensure_ccw(v1_s, v2_s, v3_s);

    //  bounding box ya en pantalla
    let x_min = min(v1_s.x as i32, min(v2_s.x as i32, v3_s.x as i32)).max(0);
    let x_max = max(v1_s.x as i32, max(v2_s.x as i32, v3_s.x as i32)).min(buff.widht as i32);
    let y_min = min(v1_s.y as i32, min(v2_s.y as i32, v3_s.y as i32)).max(0);
    let y_max = max(v1_s.y as i32, max(v2_s.y as i32, v3_s.y as i32)).min(buff.height as i32);

    for y in y_min..y_max {
        for x in x_min..x_max {
            let p = Vec2::new(x as f32 + 1.0, y as f32 + 1.0);

            let e0 = signed_area(v1_s, v2_s, p);
            let e1 = signed_area(v2_s, v3_s, p);
            let e2 = signed_area(v3_s, v1_s, p);

            let inv_z = (e2 / z3) + (e1 / z1) + (e0 / z2);
            let pixel_z = 1.0 / inv_z;
            /*
            inz_z
            =

            e0 * (1/z0) +
            e1 * (1/z1) +
            e2 * (1/z2)

            z = 1 / inv_z
            */

            if e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0 {
                if pixel_z < buff.z_buffer[y as usize][x as usize] {
                    write!(
                        buff.frame,
                        "\x1B[{};{}H\x1b[38;2;{};{};{}m{}",
                        y, x, color.0, color.1, color.2, '█'
                    )
                    .unwrap();

                    buff.z_buffer[y as usize][x as usize] = pixel_z;
                    //}
                }
            }
        }
    }
}

fn es_cara_trasera_cw(v1: &Vec3, v2: &Vec3, v3: &Vec3, camera_pos: &Vec3) -> bool {
    let arista1 = v1 - v2;
    let arista2 = v2 - v3;
    let normal = arista2.cross(arista1);

    // Vector del triángulo hacia la cámara
    let hacia_camara = *camera_pos - *v1;

    // Si la normal apunta AWAY de la cámara, es trasera
    normal.dot(hacia_camara) > 0.0
}
