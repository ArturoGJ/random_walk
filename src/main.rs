use rand::prelude::SliceRandom;
use std::time::Duration;

use sdl2::pixels::Color;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Random Walk", WIDTH, HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .map_err(|err| err.to_string())?;
    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let mut simulation_context = SimulationContext::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        simulation_context.update();

        canvas.set_draw_color(Color::WHITE);
        canvas.draw_line(
            simulation_context.walker.prev_position,
            simulation_context.walker.position,
        )?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }

    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Walker {
    position: (i32, i32),
    prev_position: (i32, i32),
    step_size: u32,
}

struct SimulationContext {
    walker: Walker,
}

impl SimulationContext {
    fn new() -> SimulationContext {
        SimulationContext {
            walker: Walker {
                position: ((WIDTH / 2) as i32, (HEIGHT / 2) as i32),
                prev_position: ((WIDTH / 2) as i32, (HEIGHT / 2) as i32),
                step_size: 5,
            },
        }
    }

    fn update(&mut self) {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let mut rng = rand::thread_rng();

        let random_direction = directions.choose(&mut rng).unwrap();
        let walker = &mut self.walker;
        walker.prev_position = walker.position;
        match random_direction {
            Direction::Up => {
                walker.position = (
                    walker.position.0,
                    walker.position.1 + walker.step_size as i32,
                )
            }
            Direction::Down => {
                walker.position = (
                    walker.position.0,
                    walker.position.1 - walker.step_size as i32,
                )
            }
            Direction::Left => {
                walker.position = (
                    walker.position.0 - walker.step_size as i32,
                    walker.position.1,
                )
            }
            Direction::Right => {
                walker.position = (
                    walker.position.0 + walker.step_size as i32,
                    walker.position.1,
                )
            }
        }
    }
}
