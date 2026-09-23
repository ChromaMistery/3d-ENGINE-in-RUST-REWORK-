//#![allow(warnings)] debug
use crossterm::terminal::size;

use std::thread;
use std::time::{Duration, Instant};

use glam::{Vec2, Vec3};

use crate::{
    engine_setup::{EngineConf, Escene},
    lights::{Light, LightType},
};
//files
mod obj;

mod render;

mod cam;

mod engine_setup;

mod objloader;

mod lights;

//files

fn main() -> std::io::Result<()> {
    EngineConf::start()?;

    let mut objeto1: obj::Obj3d = obj::Obj3d {
        color: (255, 127, 80),
        pos: Vec3::new(0.0, 0.0, 4.0),
        scale: Vec3::new(0.0, 0.0, 0.0),
        rotate: Vec3::new(0.0, 1.0, 0.0),

        mesh: objloader::load_obj("3d_models/Charizard.obj"),
    };

    //anadir caracteristica de angulo
    //anadir funcion del centro
    //normalizar objetos a espacio de pantalla con su radio (funcion de centro)
    //draw line 3d
    //alternativa para crossterm, termwiz, backend de wezterm
    // anadir visualizacion de funciones, como si fuera a predecir el futuro
    // anadir sombreado por rayos
    // anadir ray tracing para objetos reflectantes
    // anadir optimizacion de centro y de triangulos
    // anadir rayon
    // agregar check entre fotogramas, si el objeto esta en la misma pos se salta todo

    let mut prev = Instant::now();
    //instante inicial para el delta time

    let (mut cols, mut rows) = size()?;

    let cam = cam::CAM::new(
        true,
        true,
        Vec3::new(0.0, 0.0, 0.0),
        1.0, //d_fov
        0.0,
        0.0,
        100.0,
        25.0,
    );
    let sol = Light::new_directional_light(Vec3::new(0.0, 255.0, 2.0), (255, 255, 255));
    //anadir luz especular
    let l = Light::new_point_light(Vec3::new(20.0, 0.0, 0.0), (1, 255, 255), 25.0, 2.0);

    let l2 = Light::new_point_light(Vec3::new(-20.0, 0.0, 0.0), (255, 1, 1), 25.0, 2.0);

    let frame_buff = render::FrameBuffer::new(cols as usize, rows as usize);
    let escene = Escene {
        objs: vec![objeto1],
        lights: vec![sol, l, l2],
        ambient_light: 0.1,
    };
    let mut the_engine = EngineConf::new(true, cam, frame_buff, escene, 50.0, 0.0005, 100.0);

    loop {
        if !the_engine.running {
            break;
        }

        (cols, rows) = size()?;

        let now = Instant::now();

        let dt = now.duration_since(prev).as_secs_f32();

        prev = now;

        the_engine.dt = dt;
        //                                                                      temporal pa la presentacion
        engine_setup::EngineConf::update_input_event(&mut the_engine)?;
        render::FrameBuffer::clear(&mut the_engine.frame_buffer);

        the_engine.escene.objs[0].rotate(dt);

        the_engine.cam.update_rotation();
        render::render(&mut the_engine, l);
        render::FrameBuffer::print_frame(&the_engine.frame_buffer);

        render::FrameBuffer::update_size(
            &mut the_engine.frame_buffer,
            cols as usize,
            rows as usize,
        );
    }

    EngineConf::end()?;

    Ok(())
}
