use std::thread::sleep;
use std::{process::exit, time::Duration};

use soloud::{audio, AudioExt, LoadExt, Soloud};

fn main() {
    let player = match Soloud::default() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error while making player: {}", e);
            exit(1);
        }
    };

    let mut wandahoy = audio::Wav::default();

    match wandahoy.load_mem(include_bytes!("../res/wandahoy.mp3")) {
        Ok(_) => println!("WANDAHOY!!!!!!!!!!!!!!"),
        Err(e) => {
            eprintln!("error while loading wandahoy: {}", e);
            exit(2);
        }
    };

    player.play(&wandahoy);

    while player.voice_count() > 0 {
        sleep(Duration::from_millis(100));
    }
}
