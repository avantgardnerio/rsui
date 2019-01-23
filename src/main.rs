extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate gfx_device_gl;

use gfx::traits::*;
use graphics::*;
use piston::input::*;

use gfx::memory::Typed;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use glutin_window::{GlutinWindow, OpenGL};
use piston::window::{WindowSettings};
use piston::window::OpenGLWindow;
use piston::window::Window;
use piston::event_loop::{Events, EventSettings, EventLoop};

use gfx_graphics::{Gfx2d};

fn main() {
    println!("Hello, world!");

    let opengl = OpenGL::V3_2;
    let samples = 4;
    let mut window: GlutinWindow = WindowSettings::new(
        "piston: draw_state",
        [600, 600]
    )
        .exit_on_esc(true)
        .samples(samples)
        .opengl(opengl)
        .build()
        .unwrap();

    let (mut device, mut factory) = gfx_device_gl::create(|s|
        window.get_proc_address(s) as *const std::os::raw::c_void);
    let mut encoder = factory.create_command_buffer().into();

    let draw_size = window.draw_size();
    let aa = samples as gfx::texture::NumSamples;
    let dim = (draw_size.width as u16, draw_size.height as u16, 1, aa.into());
    let color_format = <Srgba8 as Formatted>::get_format();
    let depth_format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim,
                                               color_format.0,
                                               depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);

    let mut g2d = Gfx2d::new(opengl, &mut factory);
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            g2d.draw(&mut encoder, &output_color, &output_stencil, args.viewport(), |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                Rectangle::new([1.0, 0.0, 0.0, 1.0])
                    .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);
            });
            encoder.flush(&mut device);
        }

        if let Some(_) = e.after_render_args() {
            device.cleanup();
        }
    }
}

