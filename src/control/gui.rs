use crate::control::executor::{Executors, MapPlace};
use crate::control::play::{send_message, AppMessage};
use eframe::egui;
use egui::{vec2, CentralPanel, Color32, FontId, RichText, TextStyle, Window};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[derive(Default)]
pub struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
    start: bool,
    timer: i64,
    pub map: Executors,
    behave_tx: Option<Sender<AppMessage>>,
    behave_rx: Option<Receiver<AppMessage>>,
    end_tx: Option<Sender<AppMessage>>,
    is_lose: bool,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            show_confirmation_dialog: false,
            allowed_to_close: false,
            start: false,
            timer: 0,
            map: Executors::new(),
            behave_tx: Some(tx),
            behave_rx: Some(rx),
            end_tx: None,
            is_lose: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        if self.map.is_lose == true {
            let tx = self.end_tx.clone();
            if let Some(tx) = tx {
                tx.send(AppMessage::End).unwrap();
            }
            self.map.is_lose = false;
            self.is_lose = true;
        }

        self.timer += 1;
        if self.timer >= 10 {
            self.timer = 0;
            if let Some(rx) = &mut self.behave_rx {
                match rx.try_recv() {
                    Ok(AppMessage::SpawnEnemy) => self.map.spawn(),
                    Ok(AppMessage::MoveEnemies) => self.map.enemy_move(),
                    Ok(AppMessage::Shoot) => self.map.shoot(),
                    Ok(AppMessage::MoveShoot) => self.map.shoot_move(),
                    Ok(AppMessage::SpawnBlock) => self.map.spawn_block(),
                    _ => {}
                }
            }
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            Window::new("Do you want to close?")
                .fixed_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("NO").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("YES").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(135.0);
                ui.heading(
                    RichText::new("A Simple GUI")
                        .color(Color32::from_rgb(255, 0, 0))
                        .size(64.0),
                );
                let start = ui.add(
                    egui::Button::new(RichText::new("Start").size(36.0))
                        .stroke(egui::Stroke::new(2.0, Color32::RED))
                        .min_size(vec2(200.0, 64.0)),
                );
                if start.clicked() {
                    if self.start == false {
                        self.start = true;
                        let tx_clone = self.behave_tx.as_ref().unwrap().clone();
                        let (end_tx, end_rx) = mpsc::channel();
                        self.end_tx = Some(end_tx);
                        thread::spawn(move || {
                            send_message(tx_clone, end_rx);
                        });
                    }
                }
                ui.label(
                    RichText::new(format!("Point:{}", self.map.point))
                        .color(Color32::from_rgb(255, 0, 0))
                        .size(64.0),
                );
                if self.is_lose {
                    ui.label(
                        RichText::new("  Game Over")
                            .color(Color32::from_rgb(255, 0, 0))
                            .size(64.0),
                    );
                }
            });
            ui.horizontal(|ui| {
                ui.add_space(440.0);
                let font_id = FontId::monospace(25.0);
                ui.style_mut().override_text_style = Some(TextStyle::Monospace);
                ui.style_mut()
                    .text_styles
                    .insert(TextStyle::Monospace, font_id);
                egui::Grid::new("array_grid")
                    .min_col_width(20.0)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        for row in &self.map.executors {
                            for cell in row {
                                if let MapPlace::Place = cell {
                                    ui.label("+".to_string());
                                } else if let MapPlace::Block = cell {
                                    ui.label("#".to_string());
                                } else if let MapPlace::Player(player) = cell {
                                    let (_, _, heading) = player.query();
                                    ui.colored_label(
                                        Color32::GREEN,
                                        match heading {
                                            'E' => '>',
                                            'S' => 'v',
                                            'W' => '<',
                                            'N' => '^',
                                            _ => 'N',
                                        }
                                        .to_string(),
                                    );
                                } else if let MapPlace::Enemy(enemy) = cell {
                                    let (_, _, heading) = enemy.query();
                                    ui.colored_label(
                                        Color32::RED,
                                        match heading {
                                            'E' => '>',
                                            'S' => 'v',
                                            'W' => '<',
                                            'N' => '^',
                                            _ => 'N',
                                        }
                                        .to_string(),
                                    );
                                } else if let MapPlace::Shoot(_shoot) = cell {
                                    ui.colored_label(Color32::BROWN, "·".to_string());
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(26.0);
                    let left = ui.add(
                        egui::Button::new(RichText::new("L").size(36.0))
                            .stroke(egui::Stroke::new(2.0, Color32::RED))
                            .min_size(vec2(360.0, 120.0)),
                    );
                    if left.clicked() && self.start == true {
                        self.map.player_move("L");
                    }
                    ui.add_space(57.0);
                    let straight = ui.add(
                        egui::Button::new(RichText::new("M").size(36.0))
                            .stroke(egui::Stroke::new(2.0, Color32::RED))
                            .min_size(vec2(360.0, 120.0)),
                    );
                    if straight.clicked() && self.start == true {
                        self.map.player_move("M");
                    }
                    ui.add_space(57.0);
                    let right = ui.add(
                        egui::Button::new(RichText::new("R").size(36.0))
                            .stroke(egui::Stroke::new(2.0, Color32::RED))
                            .min_size(vec2(360.0, 120.0)),
                    );
                    if right.clicked() && self.start == true {
                        self.map.player_move("R");
                    }
                })
            })
        });
    }
}

pub fn create_gui() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native("CAR", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))))
}
