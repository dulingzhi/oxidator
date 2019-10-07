extern crate nalgebra as na;
use na::{Matrix4, Point3, Rotation3, Vector3};
use std::collections::HashSet;
use std::time::Instant;

pub struct State {
    pub position: Point3<f32>,
    pub dir: Vector3<f32>,
    pub key_pressed: HashSet<winit::event::VirtualKeyCode>,
    pub last_scroll: f32,
    pub fps: u64,
    pub last_frame: Instant,
    pub debug_i1: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            position: Point3::new(0.0, 0.0, 30.0),
            dir: Vector3::new(0.3, 0.0, -1.0),
            key_pressed: HashSet::new(),
            last_scroll: 0.0,
            fps: 144,
            last_frame: Instant::now(),
            debug_i1: 1,
        }
    }
}