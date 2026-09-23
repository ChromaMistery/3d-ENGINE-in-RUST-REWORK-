use std::collections::HashSet;

use std::io::Write;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use glam::Vec3;

use crate::{cam, lights::Light, obj::Obj3d, objloader::load_obj, render};
#[derive(Clone)]
pub struct EngineConf {
    pub running: bool,
    pub cam: cam::CAM,
    pub frame_buffer: render::FrameBuffer,
    pub escene: Escene,
    pub dt: f32,
    pub z_near: f32,
    pub z_far: f32,
}

//render solo podra manejar un escene composition
//asi cada escena se podra selecionar segun un evento
#[derive(Clone)]
pub struct Escene {
    pub objs: Vec<Obj3d>,
    pub lights: Vec<Light>,
    pub ambient_light: f32,
}

impl Escene {}
//pub pressed_key: HashSet<KeyCode>,

impl EngineConf {
    pub fn new(
        runnig: bool,
        cam: cam::CAM,
        frame_buffer: render::FrameBuffer,
        escene: Escene,
        dt: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            running: (runnig),
            cam: (cam),
            frame_buffer: (frame_buffer),
            escene: (escene),
            dt: (dt),
            z_near: (z_near),
            z_far: (z_far),
        }
    }
    //get input event?
    pub fn update_input_event(&mut self) -> std::io::Result<()> {
        let mut pressed_keys = HashSet::new();

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                match kind {
                    event::KeyEventKind::Repeat => {
                        pressed_keys.insert(code);
                    }
                    event::KeyEventKind::Release => {
                        pressed_keys.remove(&code);
                    }

                    _ => {}
                }

                if code == KeyCode::Esc {
                    self.running = false;
                }

                //if move or fly
                if self.cam.movent {
                    if code == KeyCode::Char('w') {
                        self.cam.pos += self.cam.forward * self.cam.move_velo * self.dt;
                    }

                    if code == KeyCode::Char('s') {
                        self.cam.pos -= self.cam.forward * self.cam.move_velo * self.dt;
                    }

                    if code == KeyCode::Char('d') {
                        self.cam.pos.x += self.cam.right.x * self.cam.move_velo * self.dt;

                        /*
                        if self.cam.fly {
                            self.cam.pos.y += self.cam.right.y * self.cam.move_velo * self.dt; //cuidado, depurar
                        }
                        */
                        self.cam.pos.z += self.cam.right.z * self.cam.move_velo * self.dt;
                    }

                    if code == KeyCode::Char('a') {
                        self.cam.pos.x -= self.cam.right.x * self.cam.move_velo * self.dt;
                        /*
                        if self.cam.fly {
                            self.cam.pos.y -= self.cam.right.y * self.cam.move_velo * self.dt; //cuidado, depurar
                        }
                        */
                        self.cam.pos.z -= self.cam.right.z * self.cam.move_velo * self.dt;
                    }

                    if code == KeyCode::Char('i') {
                        self.cam.picth += (3.0_f32).to_radians() * self.cam.sensivity * self.dt;
                    }
                    if code == KeyCode::Char('k') {
                        self.cam.picth -= (3.0_f32).to_radians() * self.cam.sensivity * self.dt;
                    }

                    if code == KeyCode::Char('u') {
                        self.cam.yaw += (3.0_f32).to_radians() * self.cam.sensivity * self.dt;
                    }
                    if code == KeyCode::Char('j') {
                        self.cam.yaw -= (3.0_f32).to_radians() * self.cam.sensivity * self.dt;
                    }

                    if self.cam.fly {
                        if code == KeyCode::Char('o') {
                            self.cam.pos.y += self.cam.move_velo * self.dt;
                        }

                        if code == KeyCode::Char('l') {
                            self.cam.pos.y -= self.cam.move_velo * self.dt;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn start() -> std::io::Result<()> {
        render::cursor_options::ocultar_cursor();
        enable_raw_mode()?;
        execute!(std::io::stdout(), EnableMouseCapture)?;

        Ok(())
    }

    pub fn end() -> std::io::Result<()> {
        render::cursor_options::mostrar_cursor();
        disable_raw_mode()?;
        execute!(std::io::stdout(), DisableMouseCapture)?;

        std::io::stdout().flush()?;

        Ok(())
    }
}
