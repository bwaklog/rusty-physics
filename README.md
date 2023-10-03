# Physics Simulation in Rust 🦀  
### version - 0001
___
> Update: Im currently rewriting the whole thing. For a more functioning version check out my [Gravitas](https://github.com/bwaklog/Gravitas) repo
> 
> ps im still very new to this <•_•>

**Explanation of my very broken code**
- define new particle:
  ```rust
  // argument order
  // here mass, velocities(vx, vy) and positions(px, py)
  // are f32
  let mut p1: Particle = Particle {mass, Vector2d{px, py}, Vector2d{vx, vy}}

  // using pretty_display()
  let particle: Particle = Particle {
        mass: 1.0,
        poistion: Vector2d { x: 10.0, y: 15.2 },
        velocity: Vector2d { x: 12.0, y: 12. },
    };
  p1.debug_display();
  // Output
  // ---------------
  // Particle Details 
  //     mass : 1
  //     position : (10, 15.2)
  //     velocity : (12, 12)
  // ---------------
  ```

  as of now i'm remaking the entire project. So force simulation is a commit behind. Its ready but im not ready to deploy it :D

  So just for my weird ocd heres how it will shows the output for now
  ```rust
  // initally an array is created to store all particles.
  // they are generated using `initialize_particles`
  let particles: Vec<Particles> = Vec::new();
  let particles = initialize_particles(particles);
  // `show_system` is going to be used to show the updated
  // particels at the clock intervals
  show_system(&particles);
  ```
  Output : 
  ```
  Particle 1
    mass: 5
    position : 26i + 20j
    velocity : 1i + -12j -> 12.0415945
    
  Particle 2
      mass: 4
      position : 6i + -39j
      velocity : -22i + 1j -> 22.022715
    
  Particle 3
      mass: 3
      position : 23i + 40j
      velocity : 34i + 40j -> 52.49762
    
  Particle 4
      mass: 4
      position : -50i + 41j
      velocity : -21i + -23j -> 31.144823
    
  Particle 5
      mass: 3
      position : 21i + -29j
      velocity : -40i + 21j -> 45.17743
  ```

---