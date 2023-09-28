#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::env;

// defining a vector
#[derive(Debug)]
struct Vector2d {
    x: f32,
    y: f32,
}
impl Vector2d {
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

// Defining a particle ⚛️
#[derive(Debug)]
struct Particle {
    mass: f32,
    poistion: Vector2d,
    velocity: Vector2d,
}
impl Particle {
    fn debug_display(&self) {
        println!(
            "Particle Details 
    mass : {}
    position : ({}, {})
    velocity : ({}, {})",
            self.mass, self.poistion.x, self.poistion.y, self.velocity.x, self.velocity.y
        )
    }
}

// function to initialize particles
fn initialize_particles(mut particles: Vec<Particle>) -> Vec<Particle> {
    for _ in 0..=NUMBER_OF_PARTICLES {
        let px = thread_rng().gen_range(-50..=50) as f32;
        let py = thread_rng().gen_range(-50..=50) as f32;
        let vx = thread_rng().gen_range(-50..=50) as f32;
        let vy = thread_rng().gen_range(-50..=50) as f32;
        let mass = thread_rng().gen_range(0..=5) as f32;
        let particle: Particle = Particle {
            mass: mass,
            poistion: Vector2d { x: px, y: py },
            velocity: Vector2d { x: vx, y: vy },
        };
        particles.push(particle);
    }
    particles
}

// function to diplay state of every particle in the system
fn show_system(particles: &Vec<Particle>) {
    let mut i: u32 = 1;
    for particle in particles.iter() {
        let mass: f32 = particle.mass;
        let px: f32 = particle.poistion.x;
        let py: f32 = particle.poistion.y;
        let vx: f32 = particle.velocity.x;
        let vy: f32 = particle.velocity.y;
        let res: f32 = (vx.powi(2) + vy.powi(2)).sqrt();
        println!(
            "Particle {}
    mass: {}
    position : {}i + {}j
    velocity : {}i + {}j -> {}
    ",
            i, mass, px, py, vx, vy, res
        );
        i = i + 1;
    }
}

// fn create_reference(mut particles: Vec<Particle>) -> Vec<Particle> {
//     particles
// }

// fn run_simulation(mut particles: Vec<Particle>) -> Vec<Particle> {
//     let particle_reference: &Vec<Particle> = &particles;

//     let dt: &f32 = &DT;
//     let mut t: f32 = 0;
//     for _ in (0..=SIMULATION_TIME as u32 * 2).step_by(2) {
//         for particle in
//     }

//     particles
// }

// defining constants
const NUMBER_OF_PARTICLES: u32 = 4;
const SIMULATION_TIME: f32 = 10.0;
const DT: f32 = 0.5;
fn main() {
    // creating an empty vector to hold all particle objects
    let mut particles: Vec<Particle> = Vec::new();
    println!("RUSTY PHISIKS 🦀🦀");

    let particles = initialize_particles(particles);
    show_system(&particles);
}
