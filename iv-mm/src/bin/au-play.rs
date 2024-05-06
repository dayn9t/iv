use std::path::Path;

use iv_mm::sound::{play_sound_n_wait, play_sound_wait};

fn main() {
    let p = Path::new("/home/jiang/ws/bot/sounds/app-start.wav");
    play_sound_wait(p);

    for i in 1..4 {
        play_sound_n_wait(p, 1);
    }
}
