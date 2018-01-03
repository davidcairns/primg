use image::RgbaImage;
use rand;
use rand::ThreadRng;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

use core;
use core::Pixels;
use scanline::Scanline;
use shape::{Shape, ShapeType};
use state::State;

pub struct Worker {
    pub w: usize,
    pub h: usize,
    pub target: Rc<Pixels>,
    pub current: Rc<RefCell<Pixels>>,
    pub buffer: Pixels,
    pub rng: ThreadRng,
    pub scanlines: Vec<Scanline>,
    pub score: f64,
}

impl Worker {
    pub fn new(target: Rc<Pixels>, current: Rc<RefCell<Pixels>>) -> Worker {
        let w = target.w;
        let h = target.h;
        let buffer = Pixels::new(w, h);
        let rng = rand::thread_rng();
        let scanlines = (0..h + 1).map(|_| Scanline::empty()).collect();
        let score = -1.0;
        Worker { w, h, target, current, buffer, rng, scanlines, score }
    }

    pub fn init(&mut self, score: f64) {
        self.score = score;
    }

    pub fn energy(&mut self, shape: &Shape, alpha: u8) -> f64 {
        let lines = shape.rasterize(self.w, self.h, &mut self.scanlines);
        let current = self.current.borrow();
        let color = current.compute_color(self.target.as_ref(), lines, alpha);
        self.buffer.copy_lines(&current, lines);
        self.buffer.draw_lines(&color, lines);
        Pixels::difference_partial(&self.target, &current, &self.buffer, self.score, lines)
    }

    pub fn best_hill_climb_state(&self, t: ShapeType, a: u8, n: u32, m: u8) -> State {
        unimplemented!()
    }

    pub fn best_random_state(&mut self, t: ShapeType, a: u8, n: u32) -> State {
        let mut best_state = self.random_state(t, a);
        let mut best_energy = best_state.energy(self);
        for i in 1..n {
            let mut state = self.random_state(t, a);
            let energy = state.energy(self);
            if i == 0 || energy < best_energy {
                best_energy = energy;
                best_state = state;
            }
        }
        best_state
    }

    pub fn random_state(&mut self, t: ShapeType, alpha: u8) -> State {
        let shape = Shape::random(t, self.w, self.h, &mut self.rng);
        State::new(shape, alpha)
    }
}
