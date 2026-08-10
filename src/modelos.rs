use crate::obj::Triangle;
use glam::Vec3;

pub fn cheap_model(name: &str) -> Vec<Triangle> {
    let mut mesh: Vec<Triangle> = Vec::new();

    if name == "cube" {
        mesh = vec![
            // Cara Sur (Z = 0)
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(0.0, 1.0, 0.0), p3: Vec3::new(1.0, 1.0, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(1.0, 1.0, 0.0), p3: Vec3::new(1.0, 0.0, 0.0) },
            // Cara Norte (Z = 1)
            Triangle { p1: Vec3::new(1.0, 0.0, 1.0), p2: Vec3::new(1.0, 1.0, 1.0), p3: Vec3::new(0.0, 1.0, 1.0) },
            Triangle { p1: Vec3::new(1.0, 0.0, 1.0), p2: Vec3::new(0.0, 1.0, 1.0), p3: Vec3::new(0.0, 0.0, 1.0) },
            // Cara Oeste (X = 0)
            Triangle { p1: Vec3::new(0.0, 0.0, 1.0), p2: Vec3::new(0.0, 1.0, 1.0), p3: Vec3::new(0.0, 1.0, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 1.0), p2: Vec3::new(0.0, 1.0, 0.0), p3: Vec3::new(0.0, 0.0, 0.0) },
            // Cara Este (X = 1)
            Triangle { p1: Vec3::new(1.0, 0.0, 0.0), p2: Vec3::new(1.0, 1.0, 0.0), p3: Vec3::new(1.0, 1.0, 1.0) },
            Triangle { p1: Vec3::new(1.0, 0.0, 0.0), p2: Vec3::new(1.0, 1.0, 1.0), p3: Vec3::new(1.0, 0.0, 1.0) },
            // Cara Inferior (Y = 0)
            Triangle { p1: Vec3::new(0.0, 0.0, 1.0), p2: Vec3::new(0.0, 0.0, 0.0), p3: Vec3::new(1.0, 0.0, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 1.0), p2: Vec3::new(1.0, 0.0, 0.0), p3: Vec3::new(1.0, 0.0, 1.0) },
            // Cara Superior (Y = 1)
            Triangle { p1: Vec3::new(0.0, 1.0, 0.0), p2: Vec3::new(0.0, 1.0, 1.0), p3: Vec3::new(1.0, 1.0, 1.0) },
            Triangle { p1: Vec3::new(0.0, 1.0, 0.0), p2: Vec3::new(1.0, 1.0, 1.0), p3: Vec3::new(1.0, 1.0, 0.0) },
        ];
    }

    if name == "pyramid" {
        mesh = vec![
            // Base
            Triangle { p1: Vec3::new(-0.5, -0.5, -0.5), p2: Vec3::new( 0.5, -0.5,  0.5), p3: Vec3::new(-0.5, -0.5,  0.5) },
            Triangle { p1: Vec3::new(-0.5, -0.5, -0.5), p2: Vec3::new( 0.5, -0.5, -0.5), p3: Vec3::new( 0.5, -0.5,  0.5) },
            // Cara Frente
            Triangle { p1: Vec3::new(-0.5, -0.5, -0.5), p2: Vec3::new( 0.0,  0.5,  0.0), p3: Vec3::new( 0.5, -0.5, -0.5) },
            // Cara Derecha
            Triangle { p1: Vec3::new( 0.5, -0.5, -0.5), p2: Vec3::new( 0.0,  0.5,  0.0), p3: Vec3::new( 0.5, -0.5,  0.5) },
            // Cara Atras
            Triangle { p1: Vec3::new( 0.5, -0.5,  0.5), p2: Vec3::new( 0.0,  0.5,  0.0), p3: Vec3::new(-0.5, -0.5,  0.5) },
            // Cara Izquierda
            Triangle { p1: Vec3::new(-0.5, -0.5,  0.5), p2: Vec3::new( 0.0,  0.5,  0.0), p3: Vec3::new(-0.5, -0.5, -0.5) },
        ];
    }

    if name == "cheap-teapot" {
        mesh = vec![
            // ── BASE ──────────────────────────────────────────────────────────
            Triangle { p1: Vec3::new(0.5, 0.0, 0.3), p2: Vec3::new(0.3, 0.0, 0.1), p3: Vec3::new(0.7, 0.0, 0.1) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.3), p2: Vec3::new(0.7, 0.0, 0.1), p3: Vec3::new(0.7, 0.0, 0.5) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.3), p2: Vec3::new(0.7, 0.0, 0.5), p3: Vec3::new(0.3, 0.0, 0.5) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.3), p2: Vec3::new(0.3, 0.0, 0.5), p3: Vec3::new(0.3, 0.0, 0.1) },
            // ── CUERPO: anillo inferior y=0.0 → y=0.2 ───────────────────────
            Triangle { p1: Vec3::new(0.3, 0.0, 0.1), p2: Vec3::new(0.7, 0.2, 0.1), p3: Vec3::new(0.7, 0.0, 0.1) },
            Triangle { p1: Vec3::new(0.3, 0.0, 0.1), p2: Vec3::new(0.3, 0.2, 0.1), p3: Vec3::new(0.7, 0.2, 0.1) },
            Triangle { p1: Vec3::new(0.7, 0.0, 0.5), p2: Vec3::new(0.7, 0.2, 0.5), p3: Vec3::new(0.3, 0.2, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.0, 0.5), p2: Vec3::new(0.3, 0.2, 0.5), p3: Vec3::new(0.3, 0.0, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.0, 0.1), p2: Vec3::new(0.7, 0.2, 0.1), p3: Vec3::new(0.7, 0.2, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.0, 0.1), p2: Vec3::new(0.7, 0.2, 0.5), p3: Vec3::new(0.7, 0.0, 0.5) },
            Triangle { p1: Vec3::new(0.3, 0.0, 0.5), p2: Vec3::new(0.3, 0.2, 0.5), p3: Vec3::new(0.3, 0.2, 0.1) },
            Triangle { p1: Vec3::new(0.3, 0.0, 0.5), p2: Vec3::new(0.3, 0.2, 0.1), p3: Vec3::new(0.3, 0.0, 0.1) },
            // ── CUERPO: anillo medio y=0.2 → y=0.4 ───────────────────────────
            Triangle { p1: Vec3::new(0.25, 0.2, 0.05), p2: Vec3::new(0.75, 0.4, 0.05), p3: Vec3::new(0.75, 0.2, 0.05) },
            Triangle { p1: Vec3::new(0.25, 0.2, 0.05), p2: Vec3::new(0.25, 0.4, 0.05), p3: Vec3::new(0.75, 0.4, 0.05) },
            Triangle { p1: Vec3::new(0.75, 0.2, 0.55), p2: Vec3::new(0.75, 0.4, 0.55), p3: Vec3::new(0.25, 0.4, 0.55) },
            Triangle { p1: Vec3::new(0.75, 0.2, 0.55), p2: Vec3::new(0.25, 0.4, 0.55), p3: Vec3::new(0.25, 0.2, 0.55) },
            Triangle { p1: Vec3::new(0.75, 0.2, 0.05), p2: Vec3::new(0.75, 0.4, 0.05), p3: Vec3::new(0.75, 0.4, 0.55) },
            Triangle { p1: Vec3::new(0.75, 0.2, 0.05), p2: Vec3::new(0.75, 0.4, 0.55), p3: Vec3::new(0.75, 0.2, 0.55) },
            Triangle { p1: Vec3::new(0.25, 0.2, 0.55), p2: Vec3::new(0.25, 0.4, 0.55), p3: Vec3::new(0.25, 0.4, 0.05) },
            Triangle { p1: Vec3::new(0.25, 0.2, 0.55), p2: Vec3::new(0.25, 0.4, 0.05), p3: Vec3::new(0.25, 0.2, 0.05) },
            Triangle { p1: Vec3::new(0.3, 0.2, 0.1), p2: Vec3::new(0.25, 0.2, 0.05), p3: Vec3::new(0.75, 0.2, 0.05) },
            Triangle { p1: Vec3::new(0.3, 0.2, 0.1), p2: Vec3::new(0.75, 0.2, 0.05), p3: Vec3::new(0.7, 0.2, 0.1) },
            // ── CUERPO: anillo superior y=0.4 → y=0.55 ───────────────────────
            Triangle { p1: Vec3::new(0.3, 0.4, 0.1), p2: Vec3::new(0.7, 0.55, 0.1), p3: Vec3::new(0.7, 0.4, 0.1) },
            Triangle { p1: Vec3::new(0.3, 0.4, 0.1), p2: Vec3::new(0.3, 0.55, 0.1), p3: Vec3::new(0.7, 0.55, 0.1) },
            Triangle { p1: Vec3::new(0.7, 0.4, 0.5), p2: Vec3::new(0.7, 0.55, 0.5), p3: Vec3::new(0.3, 0.55, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.4, 0.5), p2: Vec3::new(0.3, 0.55, 0.5), p3: Vec3::new(0.3, 0.4, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.4, 0.1), p2: Vec3::new(0.7, 0.55, 0.1), p3: Vec3::new(0.7, 0.55, 0.5) },
            Triangle { p1: Vec3::new(0.7, 0.4, 0.1), p2: Vec3::new(0.7, 0.55, 0.5), p3: Vec3::new(0.7, 0.4, 0.5) },
            Triangle { p1: Vec3::new(0.3, 0.4, 0.5), p2: Vec3::new(0.3, 0.55, 0.5), p3: Vec3::new(0.3, 0.55, 0.1) },
            Triangle { p1: Vec3::new(0.3, 0.4, 0.5), p2: Vec3::new(0.3, 0.55, 0.1), p3: Vec3::new(0.3, 0.4, 0.1) },
            // ── CUELLO y=0.55 → y=0.65 ───────────────────────────────────────
            Triangle { p1: Vec3::new(0.35, 0.55, 0.15), p2: Vec3::new(0.65, 0.65, 0.15), p3: Vec3::new(0.65, 0.55, 0.15) },
            Triangle { p1: Vec3::new(0.35, 0.55, 0.15), p2: Vec3::new(0.35, 0.65, 0.15), p3: Vec3::new(0.65, 0.65, 0.15) },
            Triangle { p1: Vec3::new(0.65, 0.55, 0.45), p2: Vec3::new(0.65, 0.65, 0.45), p3: Vec3::new(0.35, 0.65, 0.45) },
            Triangle { p1: Vec3::new(0.65, 0.55, 0.45), p2: Vec3::new(0.35, 0.65, 0.45), p3: Vec3::new(0.35, 0.55, 0.45) },
            Triangle { p1: Vec3::new(0.65, 0.55, 0.15), p2: Vec3::new(0.65, 0.65, 0.15), p3: Vec3::new(0.65, 0.65, 0.45) },
            Triangle { p1: Vec3::new(0.65, 0.55, 0.15), p2: Vec3::new(0.65, 0.65, 0.45), p3: Vec3::new(0.65, 0.55, 0.45) },
            Triangle { p1: Vec3::new(0.35, 0.55, 0.45), p2: Vec3::new(0.35, 0.65, 0.45), p3: Vec3::new(0.35, 0.65, 0.15) },
            Triangle { p1: Vec3::new(0.35, 0.55, 0.45), p2: Vec3::new(0.35, 0.65, 0.15), p3: Vec3::new(0.35, 0.55, 0.15) },
            // ── TAPA ─────────────────────────────────────────────────────────
            Triangle { p1: Vec3::new(0.5, 0.65, 0.3), p2: Vec3::new(0.35, 0.65, 0.15), p3: Vec3::new(0.65, 0.65, 0.15) },
            Triangle { p1: Vec3::new(0.5, 0.65, 0.3), p2: Vec3::new(0.65, 0.65, 0.15), p3: Vec3::new(0.65, 0.65, 0.45) },
            Triangle { p1: Vec3::new(0.5, 0.65, 0.3), p2: Vec3::new(0.65, 0.65, 0.45), p3: Vec3::new(0.35, 0.65, 0.45) },
            Triangle { p1: Vec3::new(0.5, 0.65, 0.3), p2: Vec3::new(0.35, 0.65, 0.45), p3: Vec3::new(0.35, 0.65, 0.15) },
            // Perilla (knob)
            Triangle { p1: Vec3::new(0.5, 0.8, 0.3), p2: Vec3::new(0.55, 0.65, 0.25), p3: Vec3::new(0.45, 0.65, 0.25) },
            Triangle { p1: Vec3::new(0.5, 0.8, 0.3), p2: Vec3::new(0.55, 0.65, 0.35), p3: Vec3::new(0.55, 0.65, 0.25) },
            Triangle { p1: Vec3::new(0.5, 0.8, 0.3), p2: Vec3::new(0.45, 0.65, 0.35), p3: Vec3::new(0.55, 0.65, 0.35) },
            Triangle { p1: Vec3::new(0.5, 0.8, 0.3), p2: Vec3::new(0.45, 0.65, 0.25), p3: Vec3::new(0.45, 0.65, 0.35) },
            // ── PICO (spout) ─────────────────────────────────────────────────
            Triangle { p1: Vec3::new(0.75, 0.4, 0.2), p2: Vec3::new(1.2, 0.5, 0.3), p3: Vec3::new(0.75, 0.2, 0.2) },
            Triangle { p1: Vec3::new(0.75, 0.4, 0.4), p2: Vec3::new(0.75, 0.2, 0.4), p3: Vec3::new(1.2, 0.5, 0.3) },
            Triangle { p1: Vec3::new(0.75, 0.2, 0.2), p2: Vec3::new(1.2, 0.5, 0.3), p3: Vec3::new(0.75, 0.2, 0.4) },
            Triangle { p1: Vec3::new(0.75, 0.4, 0.2), p2: Vec3::new(0.75, 0.4, 0.4), p3: Vec3::new(1.2, 0.5, 0.3) },
            // ── ASA (handle) ─────────────────────────────────────────────────
            Triangle { p1: Vec3::new(0.25, 0.55, 0.25), p2: Vec3::new(-0.05, 0.5, 0.3), p3: Vec3::new(0.25, 0.55, 0.35) },
            Triangle { p1: Vec3::new(-0.05, 0.5, 0.3), p2: Vec3::new(-0.1, 0.3, 0.3), p3: Vec3::new(0.25, 0.4, 0.35) },
            Triangle { p1: Vec3::new(-0.05, 0.5, 0.3), p2: Vec3::new(0.25, 0.4, 0.35), p3: Vec3::new(0.25, 0.4, 0.25) },
            Triangle { p1: Vec3::new(-0.1, 0.3, 0.3), p2: Vec3::new(0.25, 0.2, 0.35), p3: Vec3::new(0.25, 0.2, 0.25) },
            Triangle { p1: Vec3::new(-0.1, 0.3, 0.3), p2: Vec3::new(0.25, 0.2, 0.25), p3: Vec3::new(0.25, 0.4, 0.25) },
            Triangle { p1: Vec3::new(0.25, 0.2, 0.25), p2: Vec3::new(-0.1, 0.3, 0.3), p3: Vec3::new(0.25, 0.2, 0.35) },
        ];
    }

    if name == "star" {
        mesh = vec![
            Triangle { p1: Vec3::new(0.5, 0.5, 0.5), p2: Vec3::new(0.5, 1.0, 0.5), p3: Vec3::new(0.8, 0.5, 0.1) },
            Triangle { p1: Vec3::new(0.5, 0.5, 0.5), p2: Vec3::new(0.5, 1.0, 0.5), p3: Vec3::new(0.9, 0.5, 0.6) },
            Triangle { p1: Vec3::new(0.5, 0.5, 0.5), p2: Vec3::new(0.5, 1.0, 0.5), p3: Vec3::new(0.5, 0.5, 1.0) },
            Triangle { p1: Vec3::new(0.5, 0.5, 0.5), p2: Vec3::new(0.5, 1.0, 0.5), p3: Vec3::new(0.1, 0.5, 0.6) },
            Triangle { p1: Vec3::new(0.5, 0.5, 0.5), p2: Vec3::new(0.5, 1.0, 0.5), p3: Vec3::new(0.2, 0.5, 0.1) },
        ];
    }

    if name == "diamante" {
        mesh = vec![
            // Mitad superior (vértice en y=1.0)
            Triangle { p1: Vec3::new(0.5, 1.0, 0.5), p2: Vec3::new(0.0, 0.5, 0.0), p3: Vec3::new(1.0, 0.5, 0.0) },
            Triangle { p1: Vec3::new(0.5, 1.0, 0.5), p2: Vec3::new(1.0, 0.5, 0.0), p3: Vec3::new(1.0, 0.5, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.0, 0.5), p2: Vec3::new(1.0, 0.5, 1.0), p3: Vec3::new(0.0, 0.5, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.0, 0.5), p2: Vec3::new(0.0, 0.5, 1.0), p3: Vec3::new(0.0, 0.5, 0.0) },
            // Mitad inferior (vértice en y=0.0)
            Triangle { p1: Vec3::new(0.5, 0.0, 0.5), p2: Vec3::new(1.0, 0.5, 0.0), p3: Vec3::new(0.0, 0.5, 0.0) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.5), p2: Vec3::new(1.0, 0.5, 1.0), p3: Vec3::new(1.0, 0.5, 0.0) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.5), p2: Vec3::new(0.0, 0.5, 1.0), p3: Vec3::new(1.0, 0.5, 1.0) },
            Triangle { p1: Vec3::new(0.5, 0.0, 0.5), p2: Vec3::new(0.0, 0.5, 0.0), p3: Vec3::new(0.0, 0.5, 1.0) },
        ];
    }

    if name == "flecha" {
        mesh = vec![
            // Mango: caja delgada de x=0.0..0.5, y=0.35..0.65, z=0.35..0.65
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.5, 0.35, 0.35), p3: Vec3::new(0.0, 0.35, 0.35) },
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.5, 0.65, 0.35), p3: Vec3::new(0.5, 0.35, 0.35) },
            Triangle { p1: Vec3::new(0.0, 0.35, 0.65), p2: Vec3::new(0.5, 0.35, 0.65), p3: Vec3::new(0.0, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.5, 0.35, 0.65), p2: Vec3::new(0.5, 0.65, 0.65), p3: Vec3::new(0.0, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.0, 0.35, 0.35), p3: Vec3::new(0.0, 0.35, 0.65) },
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.0, 0.35, 0.65), p3: Vec3::new(0.0, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.5, 0.35, 0.35), p2: Vec3::new(0.5, 0.65, 0.35), p3: Vec3::new(0.5, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.5, 0.35, 0.35), p2: Vec3::new(0.5, 0.65, 0.65), p3: Vec3::new(0.5, 0.35, 0.65) },
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.5, 0.65, 0.35), p3: Vec3::new(0.5, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.0, 0.65, 0.35), p2: Vec3::new(0.5, 0.65, 0.65), p3: Vec3::new(0.0, 0.65, 0.65) },
            Triangle { p1: Vec3::new(0.0, 0.35, 0.35), p2: Vec3::new(0.5, 0.35, 0.65), p3: Vec3::new(0.5, 0.35, 0.35) },
            Triangle { p1: Vec3::new(0.0, 0.35, 0.35), p2: Vec3::new(0.0, 0.35, 0.65), p3: Vec3::new(0.5, 0.35, 0.65) },
            // Cabeza: pirámide de base cuadrada x=0.5..1.0, base y=0.1..0.9, z=0.1..0.9
            Triangle { p1: Vec3::new(1.0, 0.5, 0.5), p2: Vec3::new(0.5, 0.1, 0.1), p3: Vec3::new(0.5, 0.9, 0.1) },
            Triangle { p1: Vec3::new(1.0, 0.5, 0.5), p2: Vec3::new(0.5, 0.9, 0.1), p3: Vec3::new(0.5, 0.9, 0.9) },
            Triangle { p1: Vec3::new(1.0, 0.5, 0.5), p2: Vec3::new(0.5, 0.9, 0.9), p3: Vec3::new(0.5, 0.1, 0.9) },
            Triangle { p1: Vec3::new(1.0, 0.5, 0.5), p2: Vec3::new(0.5, 0.1, 0.9), p3: Vec3::new(0.5, 0.1, 0.1) },
            // Base de la cabeza (tapa en x=0.5, normal -X)
            Triangle { p1: Vec3::new(0.5, 0.1, 0.1), p2: Vec3::new(0.5, 0.1, 0.9), p3: Vec3::new(0.5, 0.9, 0.9) },
            Triangle { p1: Vec3::new(0.5, 0.1, 0.1), p2: Vec3::new(0.5, 0.9, 0.9), p3: Vec3::new(0.5, 0.9, 0.1) },
        ];
    }

    if name == "casa" {
        mesh = vec![
            // Paredes: cubo de x=0..1, y=0..0.6, z=0..1
            Triangle { p1: Vec3::new(0.0, 0.6, 0.0), p2: Vec3::new(1.0, 0.0, 0.0), p3: Vec3::new(0.0, 0.0, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.6, 0.0), p2: Vec3::new(1.0, 0.6, 0.0), p3: Vec3::new(1.0, 0.0, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 1.0), p2: Vec3::new(1.0, 0.0, 1.0), p3: Vec3::new(0.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(1.0, 0.0, 1.0), p2: Vec3::new(1.0, 0.6, 1.0), p3: Vec3::new(0.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(1.0, 0.0, 0.0), p2: Vec3::new(1.0, 0.6, 0.0), p3: Vec3::new(1.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(1.0, 0.0, 0.0), p2: Vec3::new(1.0, 0.6, 1.0), p3: Vec3::new(1.0, 0.0, 1.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(0.0, 0.0, 1.0), p3: Vec3::new(0.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(0.0, 0.6, 1.0), p3: Vec3::new(0.0, 0.6, 0.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(1.0, 0.0, 0.0), p3: Vec3::new(1.0, 0.0, 1.0) },
            Triangle { p1: Vec3::new(0.0, 0.0, 0.0), p2: Vec3::new(1.0, 0.0, 1.0), p3: Vec3::new(0.0, 0.0, 1.0) },
            // Techo: caballete en y=1.1 a lo largo de z
            Triangle { p1: Vec3::new(0.5, 1.1, 0.0), p2: Vec3::new(0.0, 0.6, 0.0), p3: Vec3::new(1.0, 0.6, 0.0) },
            Triangle { p1: Vec3::new(0.5, 1.1, 1.0), p2: Vec3::new(1.0, 0.6, 1.0), p3: Vec3::new(0.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.1, 0.0), p2: Vec3::new(1.0, 0.6, 0.0), p3: Vec3::new(1.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.1, 0.0), p2: Vec3::new(1.0, 0.6, 1.0), p3: Vec3::new(0.5, 1.1, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.1, 0.0), p2: Vec3::new(0.5, 1.1, 1.0), p3: Vec3::new(0.0, 0.6, 1.0) },
            Triangle { p1: Vec3::new(0.5, 1.1, 0.0), p2: Vec3::new(0.0, 0.6, 1.0), p3: Vec3::new(0.0, 0.6, 0.0) },
            // Tímpanos laterales
            Triangle { p1: Vec3::new(0.0, 0.6, 0.0), p2: Vec3::new(0.0, 0.6, 1.0), p3: Vec3::new(0.5, 1.1, 0.5) },
            Triangle { p1: Vec3::new(1.0, 0.6, 1.0), p2: Vec3::new(1.0, 0.6, 0.0), p3: Vec3::new(0.5, 1.1, 0.5) },
        ];
    }

    if name == "silla" {
        mesh = vec![
            // Pata delantera izquierda
            Triangle { p1: Vec3::new(0.05, 0.5, 0.05), p2: Vec3::new(0.15, 0.0, 0.05), p3: Vec3::new(0.05, 0.0, 0.05) },
            Triangle { p1: Vec3::new(0.05, 0.5, 0.05), p2: Vec3::new(0.15, 0.5, 0.05), p3: Vec3::new(0.15, 0.0, 0.05) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.15), p2: Vec3::new(0.15, 0.0, 0.15), p3: Vec3::new(0.05, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.15, 0.0, 0.15), p2: Vec3::new(0.15, 0.5, 0.15), p3: Vec3::new(0.05, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.15, 0.0, 0.05), p2: Vec3::new(0.15, 0.5, 0.05), p3: Vec3::new(0.15, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.15, 0.0, 0.05), p2: Vec3::new(0.15, 0.5, 0.15), p3: Vec3::new(0.15, 0.0, 0.15) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.05), p2: Vec3::new(0.05, 0.0, 0.15), p3: Vec3::new(0.05, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.05), p2: Vec3::new(0.05, 0.5, 0.15), p3: Vec3::new(0.05, 0.5, 0.05) },
            // Pata delantera derecha
            Triangle { p1: Vec3::new(0.85, 0.5, 0.05), p2: Vec3::new(0.85, 0.0, 0.05), p3: Vec3::new(0.95, 0.0, 0.05) },
            Triangle { p1: Vec3::new(0.85, 0.5, 0.05), p2: Vec3::new(0.95, 0.0, 0.05), p3: Vec3::new(0.95, 0.5, 0.05) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.15), p2: Vec3::new(0.85, 0.5, 0.15), p3: Vec3::new(0.95, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.15), p2: Vec3::new(0.95, 0.5, 0.15), p3: Vec3::new(0.95, 0.0, 0.15) },
            Triangle { p1: Vec3::new(0.95, 0.0, 0.05), p2: Vec3::new(0.95, 0.0, 0.15), p3: Vec3::new(0.95, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.95, 0.0, 0.05), p2: Vec3::new(0.95, 0.5, 0.15), p3: Vec3::new(0.95, 0.5, 0.05) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.05), p2: Vec3::new(0.85, 0.5, 0.05), p3: Vec3::new(0.85, 0.5, 0.15) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.05), p2: Vec3::new(0.85, 0.5, 0.15), p3: Vec3::new(0.85, 0.0, 0.15) },
            // Pata trasera izquierda
            Triangle { p1: Vec3::new(0.05, 0.5, 0.85), p2: Vec3::new(0.05, 0.0, 0.85), p3: Vec3::new(0.15, 0.0, 0.85) },
            Triangle { p1: Vec3::new(0.05, 0.5, 0.85), p2: Vec3::new(0.15, 0.0, 0.85), p3: Vec3::new(0.15, 0.5, 0.85) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.95), p2: Vec3::new(0.15, 0.5, 0.95), p3: Vec3::new(0.15, 0.0, 0.95) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.95), p2: Vec3::new(0.05, 0.5, 0.95), p3: Vec3::new(0.15, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.15, 0.0, 0.85), p2: Vec3::new(0.15, 0.0, 0.95), p3: Vec3::new(0.15, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.15, 0.0, 0.85), p2: Vec3::new(0.15, 0.5, 0.95), p3: Vec3::new(0.15, 0.5, 0.85) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.85), p2: Vec3::new(0.05, 0.5, 0.85), p3: Vec3::new(0.05, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.05, 0.0, 0.85), p2: Vec3::new(0.05, 0.5, 0.95), p3: Vec3::new(0.05, 0.0, 0.95) },
            // Pata trasera derecha
            Triangle { p1: Vec3::new(0.85, 0.5, 0.85), p2: Vec3::new(0.95, 0.0, 0.85), p3: Vec3::new(0.85, 0.0, 0.85) },
            Triangle { p1: Vec3::new(0.85, 0.5, 0.85), p2: Vec3::new(0.95, 0.5, 0.85), p3: Vec3::new(0.95, 0.0, 0.85) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.95), p2: Vec3::new(0.85, 0.5, 0.95), p3: Vec3::new(0.95, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.95), p2: Vec3::new(0.95, 0.5, 0.95), p3: Vec3::new(0.95, 0.0, 0.95) },
            Triangle { p1: Vec3::new(0.95, 0.0, 0.85), p2: Vec3::new(0.95, 0.5, 0.85), p3: Vec3::new(0.95, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.95, 0.0, 0.85), p2: Vec3::new(0.95, 0.5, 0.95), p3: Vec3::new(0.95, 0.0, 0.95) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.85), p2: Vec3::new(0.85, 0.0, 0.95), p3: Vec3::new(0.85, 0.5, 0.95) },
            Triangle { p1: Vec3::new(0.85, 0.0, 0.85), p2: Vec3::new(0.85, 0.5, 0.95), p3: Vec3::new(0.85, 0.5, 0.85) },
            // Asiento
            Triangle { p1: Vec3::new(0.0, 0.55, 0.0), p2: Vec3::new(1.0, 0.55, 0.0), p3: Vec3::new(1.0, 0.55, 1.0) },
            Triangle { p1: Vec3::new(0.0, 0.55, 0.0), p2: Vec3::new(1.0, 0.55, 1.0), p3: Vec3::new(0.0, 0.55, 1.0) },
            Triangle { p1: Vec3::new(0.0, 0.5,  0.0), p2: Vec3::new(1.0, 0.5,  1.0), p3: Vec3::new(1.0, 0.5,  0.0) },
            Triangle { p1: Vec3::new(0.0, 0.5,  0.0), p2: Vec3::new(0.0, 0.5,  1.0), p3: Vec3::new(1.0, 0.5,  1.0) },
            // Respaldo
            Triangle { p1: Vec3::new(0.0, 1.1, 0.85), p2: Vec3::new(1.0, 0.55, 0.85), p3: Vec3::new(0.0, 0.55, 0.85) },
            Triangle { p1: Vec3::new(0.0, 1.1, 0.85), p2: Vec3::new(1.0, 1.1, 0.85), p3: Vec3::new(1.0, 0.55, 0.85) },
            Triangle { p1: Vec3::new(0.0, 0.55, 0.95), p2: Vec3::new(1.0, 0.55, 0.95), p3: Vec3::new(0.0, 1.1, 0.95) },
            Triangle { p1: Vec3::new(1.0, 0.55, 0.95), p2: Vec3::new(1.0, 1.1, 0.95), p3: Vec3::new(0.0, 1.1, 0.95) },
            Triangle { p1: Vec3::new(0.0, 0.55, 0.85), p2: Vec3::new(0.0, 0.55, 0.95), p3: Vec3::new(0.0, 1.1, 0.95) },
            Triangle { p1: Vec3::new(0.0, 0.55, 0.85), p2: Vec3::new(0.0, 1.1, 0.95), p3: Vec3::new(0.0, 1.1, 0.85) },
            Triangle { p1: Vec3::new(1.0, 0.55, 0.85), p2: Vec3::new(1.0, 1.1, 0.85), p3: Vec3::new(1.0, 1.1, 0.95) },
            Triangle { p1: Vec3::new(1.0, 0.55, 0.85), p2: Vec3::new(1.0, 1.1, 0.95), p3: Vec3::new(1.0, 0.55, 0.95) },
            Triangle { p1: Vec3::new(0.0, 1.1, 0.85), p2: Vec3::new(0.0, 1.1, 0.95), p3: Vec3::new(1.0, 1.1, 0.95) },
            Triangle { p1: Vec3::new(0.0, 1.1, 0.85), p2: Vec3::new(1.0, 1.1, 0.95), p3: Vec3::new(1.0, 1.1, 0.85) },
        ];
    }

    mesh
}
