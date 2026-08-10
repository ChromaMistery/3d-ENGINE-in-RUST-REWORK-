//#![allow(warnings)] debug
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::{
    fmt::format,
    io::{self, prelude::*},
};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use glam::{Vec2, Vec3};

use crate::{
    obj::Obj3d,
    render::{draw_point, write},
};

mod obj;

mod render;

mod objreader;

mod modelos;

fn main() -> std::io::Result<()> {
    //cursor
    render::Cursor::ocultar_cursor();
    enable_raw_mode()?;

    // C    U   B   E
    let mut cube: obj::Obj3d = obj::Obj3d {
        letter: '█',
        color: (255, 0, 0),
        pos: Vec3::new(-2.0, 0.0, 3.5),
        rotate: Vec3::new(0.0, 0.0, 0.0),
        mesh: vec![
            // Front (z = +0.5)
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ),
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ),
            // Back (z = -0.5)
            obj::Triangle::new(
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ),
            obj::Triangle::new(
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ),
            // Right (x = +0.5)
            obj::Triangle::new(
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ),
            obj::Triangle::new(
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ),
            // Left (x = -0.5)
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ),
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ),
            // Top (y = +0.5)
            obj::Triangle::new(
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ),
            obj::Triangle::new(
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ),
            // Bottom (y = -0.5)
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
            ),
            obj::Triangle::new(
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, -0.5, 0.5),
            ),
        ],
    };
    // C    U   B   E

    // P I V O T

    let mut pivot: obj::Obj3d = obj::Obj3d {
        letter: '█',
        color: (0, 255, 0),
        pos: Vec3::new(0.0, 0.0, 6.5),
        rotate: Vec3::new(0.0, 0.0, 0.0),
        mesh: modelos::cheap_model("cheap-teapot"),
    };

    let mut pyramid: obj::Obj3d = obj::Obj3d {
        letter: '█',
        color: (0, 0, 255),
        pos: Vec3::new(3.0, 0.0, 4.5),
        rotate: Vec3::new(0.0, 0.0, 0.0),
        mesh: modelos::cheap_model("pyramid"),
    };

    let mut luz = obj::Obj3d {
        letter: '█',
        color: (255, 255, 255),
        pos: Vec3::new(1.0, 1.0, 1.0),
        rotate: Vec3::new(0.0, 0.0, 0.0),
        mesh: modelos::cheap_model("cube"),
    };

    let mut cam = obj::CAM::new(Vec3::new(0.0, 0.0, 0.0), 3.0, 0.0, 0.0, 500.0, 10.0);

    //mouse
    execute!(stdout(), EnableMouseCapture)?;

    // 1. Instante inicial

    let mut prev = Instant::now();
    //instante inicial para el delta time

    let (mut cols, mut rows) = size()?;

    let mut frame_buff = render::FrameBuffer::new(cols as usize, rows as usize);
    let mut stdout = stdout();

    let mut rotate = true;

    loop {
        (cols, rows) = size()?;

        let now = Instant::now();

        let dt = now.duration_since(prev).as_secs_f32();

        prev = now;

        //I    N    P    U    T----------------------------------------------------------
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('w') => {
                            cam.pos += cam.forward * cam.move_velo * dt;
                        }
                        KeyCode::Char('s') => {
                            cam.pos -= cam.forward * cam.move_velo * dt;
                        }

                        KeyCode::Char('a') => {
                            cam.pos.x -= cam.right.x * cam.move_velo * dt;
                            cam.pos.z -= cam.right.z * cam.move_velo * dt;
                        }

                        KeyCode::Char('d') => {
                            cam.pos.x += cam.right.x * cam.move_velo * dt;
                            cam.pos.z += cam.right.z * cam.move_velo * dt;
                        }

                        KeyCode::Char('o') => cam.pos.y += cam.move_velo * dt,

                        KeyCode::Char('l') => cam.pos.y -= cam.move_velo * dt,

                        KeyCode::Char('i') => cam.picth += (3.0_f32).to_radians(),
                        KeyCode::Char('k') => cam.picth -= (3.0_f32).to_radians(),
                        KeyCode::Char('u') => cam.yaw += (3.0_f32).to_radians(),
                        KeyCode::Char('j') => cam.yaw -= (3.0_f32).to_radians(),
                        KeyCode::Char('r') => rotate = !rotate,
                        _ => {}
                    }
                }
            }
        }

        //I    N    P    U    T---------------------------------------------------------

        //render::FrameBuffer::background_clear(&mut frame_buff, (0, 0, 0));
        render::FrameBuffer::basic_clear(&mut frame_buff);

        // I M P O R T A N T E  crear una funcion de delta time pa automatizar maaaaa
        //
        if rotate {
            obj::Obj3d::rotate(&mut cube, Vec3::new(1.0, 0.0, 2.5), dt);
            obj::Obj3d::rotate(&mut pivot, Vec3::new(0.0, 1.0, 0.0), dt);
            obj::Obj3d::rotate(&mut pyramid, Vec3::new(0.0, 1.0, 0.5), dt);
        }

        //debug

        let p_luz = luz.pos.clone();

        //debug

        let mut escene = vec![&mut cube, &mut pivot, &mut pyramid];

        render::render(
            &mut frame_buff,
            &mut escene,
            0.001, //z near
            50.0,  //z far
            &0.5,  //ambient light
            p_luz,
            &mut cam,
        );

        //agregar z buffer

        cam.update_rotation();
        render::FrameBuffer::print_buffer(&frame_buff);
    }

    //cursor

    render::Cursor::mostrar_cursor();
    disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;

    stdout.flush()?;

    Ok(())
}
