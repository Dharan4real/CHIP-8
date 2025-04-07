
use emu_core::{VirtualMachine, SCREEN_WIDTH, SCREEN_HEIGHT};

use std::{env, fs::File};
use std::io::Read;
use egui::{Context, Window};
use eframe::egui;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected arguement")
    }

    let mut vm = VirtualMachine::default();
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    vm.load_rom(&buffer);

    eframe::run_native(
        "CHIP-8",
        eframe::NativeOptions{
            window_builder: Some(Box::new(|builder| {
                builder.with_drag_and_drop(true)
            })),
            vsync: false,
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Chip8App::new(&cc, vm))))).unwrap();
}

#[derive(Default)]
struct Chip8App {
    vm: VirtualMachine,
    window_width: usize,
    window_height: usize,
}

impl Chip8App {
    pub fn new(cc: &eframe::CreationContext<'_>, vm: VirtualMachine) -> Self {
        Self {
            vm,
            window_width: SCREEN_WIDTH * 20,
            window_height: SCREEN_HEIGHT * 20,
        }
    }
}

impl eframe::App for Chip8App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        Window::new("Hello, CHIP-8")
            .show(&ctx, |ui| 
                ui.heading("CHIP-8")
            );
    }
}