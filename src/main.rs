use mpris::{PlaybackStatus, PlayerFinder};
use std::fs::File;
use std::io::prelude::*;

type Ps = PlaybackStatus;

fn main() {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Didn't find any player");

    let mut event = player.events().expect("D-Bus yay");

    let refpath = home::home_dir().unwrap().join(".local/state/music.txt");
    let path = refpath.as_path();
    let display = path.display();

    loop {
        match event.next() {
            Some(_) => {}
            None => continue,
        };

        let mut file = match File::create(path) {
            Err(fun) => panic!("couldn't open {display}: {fun}"),
            Ok(file) => file,
        };

        match player.get_playback_status().unwrap() {
            Ps::Paused | Ps::Stopped => {
                let _ = file.write(b"Stopped/Paused");
            }
            Ps::Playing => {
                let track = player.get_metadata().unwrap();
                let title = track.title().unwrap();
                let artist = track.artists().unwrap()[0]; // There's only 1 artist

                let text = format!("Playing: {title} — {artist}");
                let _ = file.write(text.as_bytes());
            }
        }
    }
}
