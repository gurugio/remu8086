use eframe::egui;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "8086 Assembly Emulator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

#[derive(Default)]
struct MyApp {
    code: String,
    registers: HashMap<&'static str, u16>,
    memory: Vec<u8>,
}

impl MyApp {
    fn new() -> Self {
        let mut app = MyApp::default();
        app.registers.insert("AX", 0);
        app.registers.insert("BX", 0);
        app.registers.insert("CX", 0);
        app.registers.insert("DX", 0);
        app.registers.insert("SP", 0);
        app.registers.insert("CS", 0);
        app.registers.insert("SS", 0);
        app.memory.resize(0x100000, 0); // 1MB 메모리 초기화
        app
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 왼쪽: 어셈블리 코드 편집기
                ui.vertical(|ui| {
                    println!("{} {}", ui.available_width(), ui.available_height());
                    ui.set_width(ui.available_width() / 2.0);
                    ui.set_height(ui.available_height());
                    ui.heading("Assembly Editor");
                    ui.text_edit_multiline(&mut self.code);
                });

                ui.separator();

                // 오른쪽: 레지스터와 메모리 값
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());

                    // 레지스터 값
                    ui.group(|ui| {
                        ui.heading("Registers");
                        for (name, value) in &self.registers {
                            ui.label(format!("{}: {:04X}", name, value));
                        }
                    });

                    ui.separator();

                    // 메모리 값
                    ui.group(|ui| {
                        ui.heading("Memory");
                        /*let mut address = 0;
                        while address < self.memory.len() {
                            let byte1 = self.memory[address];
                            let byte2 = if address + 1 < self.memory.len() {
                                self.memory[address + 1]
                            } else {
                                0
                            };
                            ui.label(format!("{:05X}: {:02X} {:02X}", address, byte1, byte2));
                            address += 2;
                        }*/
                    });
                });
            });
        });
    }
}
