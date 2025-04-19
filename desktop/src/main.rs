
use emu_core::{VirtualMachine, SCREEN_WIDTH, SCREEN_HEIGHT};

use std::{env, fs::File, io::Read};
use eframe::{egui::{self, Color32, Rect}, NativeOptions};
use egui::{CentralPanel, Key, Vec2, ViewportBuilder};

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
        NativeOptions {
            window_builder: Some(Box::new(|builder| {
                builder.with_drag_and_drop(true)
            })),
            vsync: false,
            viewport: ViewportBuilder {
                inner_size: Some(Vec2::new((SCREEN_WIDTH * 20) as f32, (SCREEN_HEIGHT * 20) as f32)),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Chip8App::new(&cc, vm))))).unwrap();
}

#[derive(Default)]
struct Chip8App {
    vm: VirtualMachine
}

impl Chip8App {
    pub fn new(_cc: &eframe::CreationContext<'_>, vm: VirtualMachine) -> Self {
        Self {
            vm
        }
    }
}
pub fn key_to_button(key: &Key) -> Option<u8> {
    match key {
        Key::Num1 => Some(0x1),
        Key::Num2 => Some(0x2),
        Key::Num3 => Some(0x3),
        Key::Num4 => Some(0x4),
        Key::Q    => Some(0x5),
        Key::W    => Some(0x6),
        Key::E    => Some(0x7),
        Key::R    => Some(0x8),
        Key::A    => Some(0x9),
        Key::S    => Some(0x0),
        Key::D    => Some(0xA),
        Key::F    => Some(0xB),
        Key::Z    => Some(0xC),
        Key::X    => Some(0xD),
        Key::C    => Some(0xE),
        Key::V    => Some(0xF),
        _         => None
    }
}

impl eframe::App for Chip8App {
    fn update(
        &mut self, 
        ctx: &egui::Context, 
        _frame: &mut eframe::Frame
    ) {
        ctx.input(|reader| {
            for key in &reader.keys_down {
                if let Some(k) = key_to_button(key) {
                    self.vm.set_key(k as usize, true);
                }
            }
        });

        for _ in 0..10 {
            self.vm.execute();
        }

        CentralPanel::default()
            .show(&ctx, |ui| {
                let painter = ui.painter();

                let screen_buffer = self.vm.get_display();

                for (i, buffer) in screen_buffer.iter().enumerate() {
                    let x = (i % SCREEN_WIDTH) as f32;
                    let y = (i / SCREEN_WIDTH) as f32;

                    let color = if *buffer {
                        Color32::WHITE
                    } else {
                        Color32::BLACK
                    };

                    painter.rect_filled(Rect::from_min_max([x, y].into(), [20., 20.].into()), 0., color);
                }

            });
    }
}