extern crate sdl2;
extern crate gl;

use lerp::Lerp;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{KeyboardState, Scancode};

use rusty_fractals::fractal::Fractal;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let timer_subsystem = sdl.timer().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(2, 1);

    let mut window_size = (1000, 800);

    let window = video_subsystem
        .window("Rusty Fractals", window_size.0, window_size.1)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let fractal = Fractal::new().unwrap();

    unsafe {
        gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    let mut zoom: f64 = 0.6;
    let mut target_zoom: f64 = 0.3;

    let mut ratio: f32 = window_size.0 as f32 / window_size.1 as f32;
    let mut fractal_pos: (f32, f32) = (-1.0, 1.0);

    let mut now: u64 = timer_subsystem.performance_counter();
    let mut last: u64;

    let mut event_pump = sdl.event_pump().unwrap();
    let mut time = 0f32;
    'main: loop {
        last = now;
        now = timer_subsystem.performance_counter();
        let mut delta_time: f64 = (((now - last) * 1000) as f64 / timer_subsystem.performance_frequency() as f64) as f64;
        if delta_time > 670.0 { delta_time = 670.0 };
        time += delta_time as f32;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                    unsafe { gl::Viewport(0, 0, width, height); }
                    window_size = (width as u32, height as u32);
                    ratio = width as f32 / height as f32;
                },
                _ => ()
            };
        }

        if target_zoom < 0.1 { target_zoom = 0.1; }

        let keyboard_state = KeyboardState::new(&event_pump);
        if keyboard_state.is_scancode_pressed(Scancode::Left)
            || keyboard_state.is_scancode_pressed(Scancode::A) { fractal_pos.0 -= ((delta_time / 1000.0) / zoom) as f32; }
        if keyboard_state.is_scancode_pressed(Scancode::Right)
            || keyboard_state.is_scancode_pressed(Scancode::D) { fractal_pos.0 += ((delta_time / 1000.0) / zoom) as f32; }
        if keyboard_state.is_scancode_pressed(Scancode::Up)
            || keyboard_state.is_scancode_pressed(Scancode::W) { fractal_pos.1 -= ((delta_time / 1000.0) / zoom) as f32; }
        if keyboard_state.is_scancode_pressed(Scancode::Down)
            || keyboard_state.is_scancode_pressed(Scancode::S) { fractal_pos.1 += ((delta_time / 1000.0) / zoom) as f32; }
        if keyboard_state.is_scancode_pressed(Scancode::KpPlus) { target_zoom *= 1.02; }
        if keyboard_state.is_scancode_pressed(Scancode::KpMinus) { target_zoom /= 1.02 }

        zoom = zoom.lerp(target_zoom, delta_time / 700.0);
        println!("Ratio: {}, Zoom: {}\r", ratio, zoom as f32);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let fractal_matrix: &[f32] = &[
            (-1.0 / (zoom as f32)), 0.0, 0.0, 0.0,
            0.0, (1.0 / ((zoom as f32) * ratio)), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -fractal_pos.0, -fractal_pos.1, 0.0, 1.0
        ];

        fractal.draw(fractal_matrix, window_size, time);
        // fractal.draw_bifurcation(window_size, time);
        window.gl_swap_window();
    }
}
