extern crate sdl2;

use screen_shot::run;

fn main() -> Result<(), String> {
    let width = 0;
    let height = 0;
    let wx = 0;
    let wy = 0;
    let clicked = false;

    run(width, height, wx, wy, clicked)?;

    Ok(())
}
