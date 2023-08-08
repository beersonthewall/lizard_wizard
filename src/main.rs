mod bus;
mod cartridge;
mod controller;
mod cpu;
mod emulator;
mod err;
mod mapper;
mod opcodes;
mod ppu;

use controller::{Button, Controller};
use emulator::Emulator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use ppu::Ppu;

const WIDTH: u32 = 400; // 400
const HEIGHT: u32 = 300; // 300

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("LizardWizard: NES Emulator", 800, 600)
	.position_centered()
	.build()
	.unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let creator = canvas.texture_creator();
    let mut _texture = creator
	.create_texture_target(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
	.unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let update_fn = Box::from(move |_ppu: &Ppu, controller: &mut Controller| {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                },
		Event::KeyDown { keycode: Some(code), .. } => {
			if let Ok(button) = Button::try_from(code) {
			    controller.press_button(button);
			}
		},
		Event::KeyUp {keycode: Some(code), .. } => {
		    if let Ok(button) = Button::try_from(code) {
			controller.release_button(button);
		    }
		},
                _ => {}
            }
        }
    });

    let mut emu = Emulator::new(update_fn);
    emu.init("./testrom.nes").unwrap();

    loop {
	emu.step().unwrap();
    }
}
