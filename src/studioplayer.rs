/*
studioplayer - digiplay playout written in rust
Copyright (C) 2022  Luke Spademan

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::audioplayer::Player;
use crate::audiowall::AudioWall;

use eframe::{
    egui::{widgets::ProgressBar, CentralPanel, Color32, Context, Ui},
    epi::{App, Frame, Storage},
};

const PADDING: f32 = 10.0;
const C_WBG: Color32 = Color32::from_rgb(43, 43, 43);
const C_SBG: Color32 = Color32::from_rgb(69, 68, 68);
const C_WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const C_PG: Color32 = Color32::from_rgb(114, 144, 114);
const C_PLAY_BG: Color32 = Color32::from_rgb(224, 224, 224);
const C_PLAY_FG: Color32 = Color32::from_rgb(29, 29, 29);
const C_STOP_BG: Color32 = Color32::from_rgb(245, 0, 87);
const C_LOAD_BG: Color32 = Color32::from_rgb(63, 81, 181);

pub struct StudioPlayer {
    players: Vec<Player>,
    walls: Vec<AudioWall>,
}

impl StudioPlayer {
    pub fn new() -> StudioPlayer {
        StudioPlayer {
            players: Vec::from_iter((0..3).map(|_a| Player {
                title: String::from("Never Gonna Give You Up"),
                artist: String::from("Rick Astley"),
            })),
            walls: Vec::from_iter((0..2).map(|_a| AudioWall::new())),
        }
    }

    fn render_players(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            for p in &self.players {
                let mut f = eframe::egui::Frame::group(ui.style());
                f.fill = C_SBG;
                f.stroke = eframe::egui::Stroke::none();
                f.margin = eframe::egui::style::Margin::same(10.);
                f.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(eframe::egui::RichText::new("00:00:00").color(C_WHITE));
                        ui.vertical(|ui| {
                            ui.colored_label(C_WHITE, format!("Title: {}", &p.title));
                            ui.colored_label(C_WHITE, format!("Artist: {}", &p.artist));
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.colored_label(C_WHITE, "Vocal In: 00:00:00");
                        ui.colored_label(C_WHITE, "Vocal Out: 00:00:00");
                    });
                    ui.add_space(PADDING);
                    let progress = ProgressBar::new(0.4).desired_width(300.);
                    ui.add(progress);
                    ui.add_space(PADDING);
                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                eframe::egui::Button::new(
                                    eframe::egui::RichText::new("PLAY").color(C_PLAY_FG),
                                )
                                .fill(C_PLAY_BG)
                                .stroke(eframe::egui::Stroke::none()),
                            )
                            .clicked()
                        {
                            println!("clicked");
                            p.play_test();
                        }
                        if ui
                            .add(
                                eframe::egui::Button::new(
                                    eframe::egui::RichText::new("STOP").color(C_WHITE),
                                )
                                .fill(C_STOP_BG)
                                .stroke(eframe::egui::Stroke::none()),
                            )
                            .clicked()
                        {
                            println!("clicked");
                        }
                        if ui
                            .add(
                                eframe::egui::Button::new(
                                    eframe::egui::RichText::new("LOAD").color(C_WHITE),
                                )
                                .fill(C_LOAD_BG)
                                .stroke(eframe::egui::Stroke::none()),
                            )
                            .clicked()
                        {
                            println!("clicked");
                        }
                    });
                });
                ui.add_space(PADDING);
            }
        });
    }
    fn render_walls(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            for wall in &mut self.walls {
                let mut f = eframe::egui::Frame::group(ui.style());
                f.fill = C_SBG;
                f.stroke = eframe::egui::Stroke::none();
                f.margin = eframe::egui::style::Margin::same(10.);
                f.show(ui, |ui| {
                    ui.vertical(|ui| {
                        for row in 0..3 {
                            ui.horizontal(|ui| {
                                for col in 0..3 {
                                    let index = row * 3 + col;
                                    let title = &wall.items[index].title;
                                    let duration = "52.3s";

                                    let line2 = match &wall.items[index].playing {
                                        true => format!("Playing"),
                                        false if title.chars().count() > 15 => {
                                            format!("{:^15.15}", &title[15..])
                                        }
                                        _ => format!(""),
                                    };
                                    if ui
                                        .button(
                                            eframe::egui::RichText::new(format!(
                                                "{:^15.15}\n{:^15.15}\n{:^15.15}",
                                                title, line2, duration
                                            ))
                                            .monospace(),
                                        )
                                        .clicked()
                                    {
                                        wall.items[index].pressed();
                                    }
                                }
                            });
                        }
                    });
                });
                ui.add_space(PADDING);
            }
        });
    }
}

impl App for StudioPlayer {
    fn setup(&mut self, _ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {}
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        let mut frame = eframe::egui::Frame::none();
        frame.fill = C_WBG;
        frame.margin = eframe::egui::style::Margin::same(10.);
        CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.render_players(ui);
                self.render_walls(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "StudioPlayer"
    }
}
