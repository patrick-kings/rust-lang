/*

# Snow particles
    The snow-like dots will start to float from the bottom and fade as they approach the top.

It has 3 major sections:
    - A memory allocator (the ReportingAllocator struct) records the time that dynamic memory allocations take.
    - Definitions of the structs World and Particle and how these behave over time.
    - The executor() function deals with window creation and initialization.

*/

// Vec2d provides mathematical operations and conversion functionality for 2D vectors.
use graphics::math::{add, mul_scalar, Vec2d};

// piston_window provides the tools to create a GUI program and draws shapes to it.
use piston_window::*;

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

// std::alloc provies facilities for controlling memory allocation.
use std::alloc::{GlobalAlloc, Layout, System};

use std::time::Instant;

// prints the time taken for each allocation to STDOUT as the program runs.
// This provies a fairly accurate indication of the time taken for dynamic memory allocation.
struct ReportingAllocator;

// #[gloabl_allocator] marks the following value (ALLOCATOR) as satisfying the GloablAlloc trait
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();

        // Defers the actual memory allocation to the system's default memory allocator
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

// contains the data that is useful for the lifetime of the program
struct World {
    current_turn: u64,
    particles: Vec<Box<Particle>>,
    height: f64,
    width: f64,
    rng: ThreadRng,
}

// Defines the object in 2D space
struct Particle {
    height: f64,
    width: f64,
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
    color: [f32; 4],
}

impl Particle {
    fn new(world: &World) -> Self {
        let mut rng = thread_rng();

        // starts a random position along the bottom of the window.
        let x = rng.gen_range(0.0..=world.width);
        let y = world.height;

        // Rises vertically over time
        let x_velocity = 0.0;
        let y_velocity = rng.gen_range(-2.0..0.0);

        // Increases the speed of the rise over time.
        let x_acceleration = 0.0;
        let y_acceleration = rng.gen_range(0.0..0.15);

        return Particle {
            height: 4.0,
            width: 4.0,

            // into converts the arrays of type [f64; 2] into Vec2d
            position: [x, y].into(),
            velocity: [x_velocity, y_velocity].into(),
            acceleration: [x_acceleration, y_acceleration].into(),

            // Inserts a fully saturated white that has a tiny amount of transparency
            color: [1.0, 1.0, 1.0, 0.99],
        };
    }

    fn update(&mut self) {
        // moves the particle to its next position
        self.velocity = add(self.velocity, self.acceleration);
        self.position = add(self.position, self.velocity);

        // slows down the particle's rate of increase as it travels across the screen
        self.acceleration = mul_scalar(self.acceleration, 0.7);

        // makes the particle more transparent over time
        self.color[3] *= 0.995;
    }
}

impl World {
    fn new(width: f64, height: f64) -> Self {
        return World {
            current_turn: 0,

            // Uses Box<Particle> rather than Particle inorder to incur an extra memory allocation when every particle is created
            particles: Vec::<Box<Particle>>::new(),

            height,
            width,
            rng: thread_rng(),
        };
    }

    fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            // Creates a Particle as a local variable on the stack
            let particle = Particle::new(&self);

            // Takes ownership of particle, moving its data to the heap and creates a reference to
            // that data on the stack.
            let boxed_particles = Box::new(particle);

            // Pushes the reference into self.shapes
            self.particles.push(boxed_particles);
        }
    }

    fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete: Option<usize> = None;

            // particle_iter is split into its own variable to more easily fit on the page.
            let particle_iter = self.particles.iter().enumerate();

            // For n iterations, removes the first particle that's invisible. If thers are no
            // invisible particles, then removes the oldest.
            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            };
        }
    }

    fn update(&mut self) {
        // Returns a random integer between -3 and 3, inclusive.
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();

        for shape in &mut self.particles {
            shape.update();
        }

        self.current_turn += 1;
    }
}

pub fn execute() {
    let (width, height) = (1280.0, 960.0);
    let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create a window.");

    let mut world = World::new(width, height);
    world.add_shapes(2000);

    while let Some(event) = window.next() {
        world.update();

        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([0.15, 0.17, 0.17, 0.9], renderer);

            for s in &mut world.particles {
                let size = [s.position[0], s.position[1], s.width, s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
