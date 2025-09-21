// src/lib/physics/src/lib.rs
// This Rust code is compiled to WebAssembly (WASM).
// It runs the core physics simulation, offloading heavy work from JavaScript.

use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator for smaller WASM file sizes.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const G: f32 = 0.1; // Gravitational constant

/// Represents a single particle of dust or gas in the universe.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32, // For 2.5D parallax effect
    vx: f32,
    vy: f32,
    vz: f32,
}

/// Represents a star, which exerts gravitational force.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub age: u32,
    pub is_ignited: bool,
}

/// Represents a black hole, a massive gravitational sink.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct BlackHole {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
}

/// The main Universe struct that holds the entire state of the simulation.
#[wasm_bindgen]
pub struct Universe {
    width: f32,
    height: f32,
    particles: Vec<Particle>,
    stars: Vec<Star>,
    black_holes: Vec<BlackHole>,
    debug_mode: bool,
}

#[wasm_bindgen]
impl Universe {
    /// Creates a new Universe instance.
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, particle_count: i32, debug_mode: bool) -> Universe {
        let mut particles = Vec::with_capacity(particle_count as usize);
        for _ in 0..particle_count {
            particles.push(Particle {
                x: js_sys::Math::random() as f32 * width,
                y: js_sys::Math::random() as f32 * height,
                z: js_sys::Math::random() as f32,
                vx: (js_sys::Math::random() as f32 - 0.5) * 0.2,
                vy: (js_sys::Math::random() as f32 - 0.5) * 0.2,
                vz: 0.0,
            });
        }
        
        if debug_mode {
            console::log_1(&"WASM Physics Engine Initialized in DEBUG mode.".into());
        }

        Universe {
            width,
            height,
            particles,
            stars: Vec::new(),
            black_holes: Vec::new(),
            debug_mode,
        }
    }

    /// The main simulation loop, called once per frame from JavaScript.
    /// It calculates all physics interactions and updates the state.
    pub fn tick(&mut self) {
        for particle in self.particles.iter_mut() {
            let mut fx: f32 = 0.0;
            let mut fy: f32 = 0.0;
            
            for star in &self.stars {
                let dx = star.x - particle.x;
                let dy = star.y - particle.y;
                let dist_sq = dx*dx + dy*dy;
                if dist_sq > 10.0 {
                    let force = (G * star.mass) / dist_sq;
                    fx += force * dx / dist_sq.sqrt();
                    fy += force * dy / dist_sq.sqrt();
                }
            }

            for black_hole in &self.black_holes {
                 let dx = black_hole.x - particle.x;
                 let dy = black_hole.y - particle.y;
                 let dist_sq = dx*dx + dy*dy;
                 if dist_sq > 25.0 {
                    let force = (G * black_hole.mass) / dist_sq;
                    fx += force * dx / dist_sq.sqrt();
                    fy += force * dy / dist_sq.sqrt();
                 } else if dist_sq < 1.0 {
                    particle.x = -100.0; // Mark for removal
                 }
            }
            
            particle.vx += fx;
            particle.vy += fy;
            particle.x += particle.vx;
            particle.y += particle.vy;

            if particle.x < 0.0 || particle.x > self.width { particle.vx *= -1.0; }
            if particle.y < 0.0 || particle.y > self.height { particle.vy *= -1.0; }
        }

        self.particles.retain(|p| p.x > -50.0);

        if self.debug_mode && js_sys::Math::random() < 0.01 {
            console::log_1(&format!("Simulating {} particles.", self.particles.len()).into());
        }
    }
    
    /// Returns a copy of the current particle data.
    pub fn get_particles_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(self.particles.len() * 3);
        for p in &self.particles {
            data.push(p.x);
            data.push(p.y);
            data.push(p.z);
        }
        data
    }

    /// Adds a new star to the simulation.
    pub fn add_star(&mut self, x: f32, y: f32, mass: f32) {
        self.stars.push(Star { x, y, mass, age: 0, is_ignited: true });
    }

    /// Adds a new black hole to the simulation.
    pub fn add_black_hole(&mut self, x: f32, y: f32, mass: f32) {
        self.black_holes.push(BlackHole { x, y, mass });
    }
}

