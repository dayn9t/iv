use std::path::Path;

use iv_mm::sound::{play_sound_n_wait, play_sound_wait};

fn main() {
    let p = Path::new("/home/jiang/ws/bot/sounds/app-start.wav");
    play_sound_wait(p);

    play_sound_n_wait(p, 1);
}
