#[allow(unused_variables)]

struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    // Create a new vector
    fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn _set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn _set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn _pretty_define(&self) {
        println!("x: {:?} \ny: {:?}", self.get_x(), self.get_y());
    }
}

struct Particle {
    mass: f32,
    positions: Vector,
    velocity: Vector,
}

impl Particle {
    fn new(mass: f32, x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Particle {
            mass,
            positions: Vector::new(x, y),
            velocity: Vector::new(vx, vy),
        }
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_positions(&self) -> (f32, f32) {
        (self.positions.get_x(), self.positions.get_y())
    }

    fn set_positions(&mut self, x: f32, y: f32) {
        let vec = Vector::new(x, y);
        self.positions = vec;
    }

    fn get_velocity(&self) -> (f32, f32, f32) {
        let x = self.velocity.get_x();
        let y = self.velocity.get_y();
        let res = (x.powi(2) + y.powi(2)).sqrt();
        (self.velocity.get_x(), self.velocity.get_y(), res)
    }

    fn _set_velocity(&mut self, vx: f32, vy: f32) {
        let v_vec = Vector::new(vx, vy);
        self.velocity = v_vec;
    }

    fn pretty_define(&self) {
        println!(
            "---------------\nMass: {:?}\nPositions: {:?}\nVecloity: {:?}\n----------------",
            self.get_mass(),
            self.get_positions(),
            self.get_velocity()
        );
    }

    fn ext_force_simu2(&mut self, fx: f32, fy: f32, t: f32) {
        // let mut px = self.positions.get_x();
        // let mut py = self.positions.get_y();
        let mut vx = self.velocity.get_x();
        let mut vy = self.velocity.get_y();

        // 1st eqn of motion
        // v = u + at
        vx = vx + (fx / self.mass) * t;
        vy = vy + (fy / self.mass) * t;
        // 2nd equation
        let px = self.positions.get_x() + vx * t;
        let py = self.positions.get_y() + vy * t;
        self.velocity = Vector::new(vx, vy);
        self.positions = Vector::new(px, py);
    }
}

fn main() {
    println!("Hello, world!");

    let mut p1 = Particle::new(10.0, 0.0, 0.0, 0.0, 0.0);

    p1.pretty_define();

    p1.set_positions(0.0, 15.0);
    p1.pretty_define();

    p1.ext_force_simu2(1.0, 1.0, 5.0);
    p1.pretty_define();
}

#[test]
fn run_simu2() {
    let mut p = Particle::new(10.0, 0.0, 15.0, 0.0, 0.0);
    p.pretty_define();
    p.ext_force_simu2(1.0, 1.0, 5.0);
    p.pretty_define();
}
