#[allow(unused_imports)]
use rand::{thread_rng, Rng};

// defining a vector
#[derive(Debug, Clone)]
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

// Defining a particle
#[derive(Debug, Clone)]
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

    fn set_pos(&mut self, px: f32, py: f32) {
        self.poistion = Vector2d { x: px, y: py }
    }

    fn set_vel(&mut self, vx: f32, vy: f32) {
        self.velocity = Vector2d { x: vx, y: vy }
    }
}

// function to initialize particles
fn initialize_particles(mut particles: Vec<Particle>) -> Vec<Particle> {
    for _ in 0..NUMBER_OF_PARTICLES {
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

//
// From this line onwards...its simulation baby!
fn fnet_2d(particle: &mut Particle) {
    let mass: f32 = particle.mass;
    let px: f32 = particle.poistion.x;
    let py: f32 = particle.poistion.y;
    let vx: f32 = particle.velocity.x;
    let vy: f32 = particle.velocity.y;

    // now we compute stuff cuz of dat gravity existing
    let py = vy + 0.5 * _GACC * _DT.powi(2);
    let vy = vy + _GACC * _DT;
    particle.set_pos(px, py);
    particle.set_vel(vx, vy);
}

fn run_simu(particles: &Vec<Particle>) {
    for t in 1.._SIMULATION_TIME as i32 {
        for particle in particles.iter() {
            fnet_2d(&mut particle.clone());
            show_system(particles);
        }
    }
}

// defining constants
const NUMBER_OF_PARTICLES: u32 = 2;
const _SIMULATION_TIME: f32 = 10.0;
const _GACC: f32 = 9.8;
const _DT: f32 = 0.5;
fn main() {
    // creating an empty vector to hold all particle objects
    let mut particles: Vec<Particle> = Vec::new();
    println!("RUSTY PHISIKS 🦀🦀");

    let particles = initialize_particles(particles);
    // particles.push(Particle {
    //    mass: 1.0,
    //    poistion: Vector2d { x: 0.0, y: 0.0 },
    //    velocity: Vector2d { x: 0.0, y: 0.0 },
    // });
    // show_system(&particles);
    run_simu(&particles)
}
