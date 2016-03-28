use std::thread;
use glium::glutin::{WindowBuilder, Event,  ElementState as KeyState};
use glium::DisplayBuild;

use super::video::ZXScreenRenderer;
use super::keyboard::vkey_to_zxkey;
use z80::Z80;
use zx::*;
use time;
use std::time::Duration;

fn ns_to_ms(ns: u64) -> f64 {
    ns as f64 / 1_000_000f64
}

fn s_to_ns(s: f64) -> u64 {
    (s * 1_000_000_000_f64) as u64
}

fn ms_to_ns(s: f64) -> u64 {
    (s * 1_000_000_f64) as u64
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

pub struct RustZXApp;

impl RustZXApp {
    pub fn new() -> RustZXApp {
        RustZXApp
    }
    pub fn start(&mut self) {

        let mut controller = ZXController::new();
        let mut cpu = Z80::new();
        let mut memory = ZXMemory::new(RomType::K16, RamType::K48);
        memory.load_rom(0, include_bytes!("48.rom")).unwrap();
        controller.atach_memory(memory);
        controller.attach_screen(ZXScreen::new());

        // build new glium window
        let display = WindowBuilder::new()
            .with_dimensions(384 * 2, 288 * 2)
            .build_glium().unwrap();
        let mut renderer = ZXScreenRenderer::new(&display);

        // TODO: make changable, 10 x speed for now
        let _cpu_freq = 3_500_000u64;
        // NOTE: 2x speed
        let frame_clocks = 69888u64 * 2u64; // clocks per frame
        let frame_target_dt_ns = ms_to_ns((1000/50) as f64);
        //let frame_target_dt_ns = s_to_ns(frame_clocks as f64 / cpu_freq as f64);
        let mut excess_clocks = 0u64; // clocks, which were uncounted from prev frame
        let mut frame_counter = 0_usize;
        'render_loop : loop {
            frame_counter += 1;
            // start time, in ns
            let frame_start_ns = time::precise_time_ns();
            let mut clocks = 0;
            // interrupt on first instruction
            cpu.request_interrupt();
            while clocks < frame_clocks {
                clocks += cpu.emulate(&mut controller);
            };
            if clocks + excess_clocks > frame_clocks {
                excess_clocks = (clocks + excess_clocks) - frame_clocks;
            };
            let cpu_dt_ns =  time::precise_time_ns() - frame_start_ns;
            // render display
            if (frame_counter % 32) == 0 {
                renderer.invert_blink();
            }
            renderer.set_border_color(controller.get_border_color());
            renderer.draw_screen(&display, controller.get_screen_texture());
            // glutin events
            for event in display.poll_events() {
                match event {
                    Event::Closed => {
                        break 'render_loop;
                    }
                    Event::KeyboardInput(state, _, Some(key_code)) => {
                        if let Some(key) =  vkey_to_zxkey(key_code) {
                            match state {
                                KeyState::Pressed => controller.send_key(key, true),
                                KeyState::Released => controller.send_key(key, false),
                            }
                        }
                    }
                    Event::MouseWheel(_) => {
                        println!("pc: {:#02X}", cpu.regs.get_pc());
                    }
                    _ => {},
                }
            }
            let emulation_dt_ns = time::precise_time_ns() - frame_start_ns;

            // wait some time for 50 FPS
            if emulation_dt_ns < frame_target_dt_ns {
                thread::sleep(Duration::new(0, (frame_target_dt_ns - emulation_dt_ns) as u32));
            };
            let frame_dt_ns = time::precise_time_ns() - frame_start_ns;
            if let Some(wnd) = display.get_window() {
                wnd.set_title(&format!("CPU: {:7.3}ms; EMULATOR: {:7.3}ms; FRAME:{:7.3}ms",
                    ns_to_ms(cpu_dt_ns),
                    ns_to_ms(emulation_dt_ns),
                    ns_to_ms(frame_dt_ns)));
            }
        }
    }
}