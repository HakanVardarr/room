extern crate sdl2;

use screen_shot::{run_mac, run_win};

fn main() -> Result<(), String> {
    let width = 0;
    let height = 0;
    let wx = 0;
    let wy = 0;
    let clicked = false;

    let os_type = std::env::consts::OS;
    match os_type {
        "macos" => {
            run_mac(width, height, wx, wy, clicked)?;
        }
        "windows" => {
            run_win(width, height, wx, wy, clicked)?;
        }
        _ => {
            unimplemented!();
        }
    }

    // run(width, height, wx, wy, clicked)?;

    Ok(())
}
