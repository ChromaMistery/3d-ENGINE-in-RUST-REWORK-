use bresenham::Bresenham;
use crossterm::{cursor, queue, style::Print};

use std::{
    f32,
    fmt::{Write, format},
    os::raw::c_short,
    result, thread,
};

use std::cmp::{max, min};

use crate::obj::{self, Obj3d};
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
        self.widht = w;
        self.height = h;
        self.frame = String::with_capacity(w * h * 25);
        self.z_buffer = vec![vec![f32::INFINITY; w]; h];
    }

    pub fn print_buffer(&self) {
        println!("{}", self.frame);
    }

    pub fn basic_clear(&mut self) {
        self.frame.clear();
        self.frame.push_str("\x1B[2J\x1B[H");

        for row in &mut self.z_buffer {
            row.fill(f32::INFINITY);
        }
    }

    pub fn background_clear(&mut self, color: (u8, u8, u8)) {
        self.frame.clear();
        self.frame.push_str("\x1B[2J\x1B[H");
        for i in 0..self.height {
            for j in 0..self.widht {
                write!(
                    self.frame,
                    "\x1B[{};{}H\x1b[48;2;{};{};{}m ",
                    i, j, color.0, color.1, color.2
                )
                .unwrap();
            }
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

    //debug, si quito optimizacion mas correcion
    //self.frame.push_str("\x1b[0m"); //reinciar toda la terminal y los ansi codes
}

pub mod Cursor {
    pub fn ocultar_cursor() {
        println!("\x1b[?25l:");
    }

    pub fn mostrar_cursor() {
        println!("\x1b[?25h:");
    }
}

//dentro del triangulo == edge fn
fn signed_area(v1: Vec2, v2: Vec2, v3: Vec2) -> f32 {
    (v2.x - v1.x) * (v3.y - v1.y) - (v3.x - v1.x) * (v2.y - v1.y)
}

/// devuelve los vertices reordenados para garantizar orden antihorario CCW
/// asume sistema de coordenadas co hacia arriba

//usar para leer con tobj
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

    let mut v0 = buff.screen(v.clone());
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
pub fn proyect_3d(v: &Vec3, cam: &mut obj::CAM) -> Vec2 {
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

pub fn render(
    buff: &mut FrameBuffer,
    escene: &mut Vec<&mut obj::Obj3d>,
    z_near: f32,
    z_far: f32,
    ambiente: &f32,
    luz_pos: Vec3,
    cam: &mut obj::CAM,
) {
    //sort de mayor a menor de z
    //escene.sort_by(|a, b| b.pos.z.total_cmp(&a.pos.z));

    for obj in escene {
        for triangle in &obj.mesh {
            let p1 = triangle.p1 + obj.pos;
            let p2 = triangle.p2 + obj.pos;
            let p3 = triangle.p3 + obj.pos;

            let r1 = obtener_p_relativo(p1, cam);
            let r2 = obtener_p_relativo(p2, cam);
            let r3 = obtener_p_relativo(p3, cam);

            /*
            if r1.x > buff.widht as f32 && r2.x > buff.widht as f32 && r3.x > buff.widht as f32 {
                continue;
            }

            if r1.x > buff.height as f32 && r2.y > buff.height as f32 && r3.y > buff.height as f32 {
                continue;
            }
            */

            if r1.z < z_near || r2.z < z_near || r3.z < z_near {
                continue;
            }

            if r1.z > z_far && r2.z > z_far && r3.z > z_far {
                continue;
            }
            //arreglar el z far y near con el p final de la camara
            let char = ascii_luz(&r1, &r2, &r3, &luz_pos, ambiente);

            //back face culling, cambiar con el modelo de camara movible,
            // esto solo sera temporal

            let s1 = proyect_3d(&r1, cam);
            let s2 = proyect_3d(&r2, cam);
            let s3 = proyect_3d(&r3, cam);

            //adaptar signeg area a la camara
            if signed_area(s1, s2, s3) < 0.0 {
                //draw_ascii_triangle(buff, s1, s2, s3, char, obj.color);

                draw_full_triangle(buff, cam, r1, r2, r3, char, obj.color);
                //draw_ascii_triangle(buff, s1, s2, s3, obj.letter, (0, 0, 0));
            }
        }
    }
}

fn obtener_p_relativo(v: Vec3, cam: &mut obj::CAM) -> Vec3 {
    let camera_space = cam.rotation.transform_point3(v - cam.pos);

    camera_space
}

pub fn draw_full_triangle(
    buff: &mut FrameBuffer,
    cam: &mut obj::CAM,
    v1: Vec3,
    v2: Vec3,
    v3: Vec3,
    letter: char,
    color: (u8, u8, u8),
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

    //asi se ajusta al len de la lista -1
    for y in y_min..y_max {
        for x in x_min..x_max {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

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
                        y, x, color.0, color.1, color.2, letter
                    )
                    .unwrap();

                    buff.z_buffer[y as usize][x as usize] = pixel_z;
                    //}
                }
            }
        }
    }
}

fn ascii_luz(v1: &Vec3, v2: &Vec3, v3: &Vec3, luz_pos: &Vec3, ambiente: &f32) -> char {
    let arista1 = v1 - v2;
    let arista2 = v2 - v3;

    let ambiente = ambiente;
    let normal = arista1.cross(arista2).normalize();
    let intensidad = (normal.dot(*luz_pos).max(0.0) + ambiente).min(1.0);

    let luz = vec![
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
        '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n',
        'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p',
        'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$', '░', '▒',
    ];

    let indice = (intensidad * (luz.len() - 1) as f32) as usize;
    luz[indice]
}
