## Physics Simulation in Rust 🦀
---
**Functions helper**
- define new particle:
  ```rust
  // argument order
  // mass, position-x, position-y, 
  // velocity-x, velocity-y -> f32
  let mut p1 = Particle::new(10.0, 0.0, 15.0, 0.0, 0.0)

  // using pretty_display()
  p1.pretty_display()
  // Output
  // ---------------
  // mass: 10.0
  // Positions: (0.0, 15.0)
  // Velocity: (0.0, 0.0, 0.0)
  // ---------------
  ```
- exert __*constant*__ external force:
  ```rust
  // argument order : force-x, force-y, 
  // time-of-application -> f32
  p1.ext_force_simu2(1.0, 1.0, 5.0);

  // using pretty_display():
  // velocity order: force-x, force-y, resultant
  p1.pretty_display();
  // Output
  // ---------------
  // mass: 10.0
  // Positions: (3.0, 17.5)
  // Velocity: (0.5, 0.5, 0.70710677)
  // ---------------
  ```
- 