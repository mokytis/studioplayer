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

use studioplayer::studioplayer::StudioPlayer;
use studioplayer::audioplayer;

use eframe::{egui::Vec2, run_native, NativeOptions};

fn main() {
    let app = StudioPlayer::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(800., 600.));
    run_native(Box::new(app), win_option);
}
