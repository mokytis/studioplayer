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

pub struct AudioWall {
    pub items: Vec<AudioWallItem>,
}

impl AudioWall {
    pub fn new() -> AudioWall {
        let mut aw = AudioWall {
            items: Vec::from_iter((5..14).map(|a| AudioWallItem {
                title: format!("Station Ad {}", a),
                playing: false,
            })),
        };
        aw.items[3].title = String::from("This is a very long title for testing purposes");
        aw
    }
}

pub struct AudioWallItem {
    pub title: String,
    pub playing: bool,
}

impl AudioWallItem {
    pub fn pressed(&mut self) {
        self.playing = !self.playing;
    }
}
