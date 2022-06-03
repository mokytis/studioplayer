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

use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::{OutputStream, Sink, Decoder, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub struct Player {
    pub title: String,
    pub artist: String,
}

impl Player {
    pub fn play_test(&self) {
        println!("playing...");
        thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("/home/mokytis/raw/audio/FINNEAS_Claudia.mp3").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        println!("dur: {:?}", source.total_duration());
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
        });
        // Play the sound directly on the device
        //stream_handle.play_raw(source.convert_samples());

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        //std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
