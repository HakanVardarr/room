use screenshots::Screen;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

pub fn get_screen(width: &mut u32, height: &mut u32, wx: &mut i32, wy: &mut i32) -> Vec<u8> {
    let screens = Screen::all().unwrap();
    let mut buffer = Vec::new();

    for screen in screens {
        let image = screen.capture().unwrap();
        *width = screen.display_info.width;
        *height = screen.display_info.height;
        *wx = *width as i32 / 2;
        *wy = *height as i32 / 2;

        buffer = image.buffer().to_owned();
    }

    buffer
}

pub fn run_win(
    mut width: u32,
    mut height: u32,
    mut wx: i32,
    mut wy: i32,
    mut clicked: bool,
) -> Result<(), String> {
    let buffer = get_screen(&mut width, &mut height, &mut wx, &mut wy);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .borderless()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(&buffer.as_slice())?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::MouseButtonDown {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks,
                    x,
                    y,
                } => {
                    clicked = true;
                    if clicks == 2 {
                        canvas.copy(&texture, None, None)?;
                        canvas.present();
                        clicked = false;
                    } else {
                        canvas.copy(
                            &texture,
                            Some(Rect::new(x, y, width - 500, height - 500)),
                            None,
                        )?;
                        canvas.present();
                        wx = x;
                        wy = y;
                    }
                }
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => {
                    if clicked {
                        if wx > 0 && wx < width as i32 - 500 && wy > 0 && wy < height as i32 - 500 {
                            if let Some(key) = keycode {
                                match key {
                                    Keycode::Left => {
                                        wx -= 30;
                                    }
                                    Keycode::Right => {
                                        wx += 30;
                                    }
                                    Keycode::Up => {
                                        wy -= 30;
                                    }
                                    Keycode::Down => {
                                        wy += 30;
                                    }
                                    _ => (),
                                }
                            }
                            canvas.copy(
                                &texture,
                                Some(Rect::new(wx, wy, width - 500, height - 500)),
                                None,
                            )?;
                            canvas.present();
                        } else {
                            if wx >= width as i32 - 500 {
                                wx = width as i32 - 501;
                            }
                            if wx <= 0 {
                                wx = 1;
                            }

                            if wy >= height as i32 - 500 {
                                wy = height as i32 - 501;
                            }

                            if wy <= 0 {
                                wy = 1;
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    Ok(())
}

pub fn run_mac(
    mut width: u32,
    mut height: u32,
    mut wx: i32,
    mut wy: i32,
    mut clicked: bool,
) -> Result<(), String> {
    let buffer = get_screen(&mut width, &mut height, &mut wx, &mut wy);
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", width, height)
        .position_centered()
        .borderless()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture_bytes(&buffer.as_slice())?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::MouseButtonDown {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks,
                    x,
                    y,
                } => {
                    clicked = true;
                    if clicks == 2 {
                        canvas.copy(&texture, None, None)?;
                        canvas.present();
                        clicked = false;
                    } else {
                        canvas.copy(
                            &texture,
                            Some(Rect::new(x, y, width - 300, height - 300)),
                            None,
                        )?;
                        canvas.present();
                        wx = x;
                        wy = y;
                    }
                }
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => {
                    if clicked {
                        if wx > 0 && wx < width as i32 - 300 && wy > 0 && wy < height as i32 - 300 {
                            if let Some(key) = keycode {
                                match key {
                                    Keycode::Left => {
                                        wx -= 30;
                                    }
                                    Keycode::Right => {
                                        wx += 30;
                                    }
                                    Keycode::Up => {
                                        wy -= 30;
                                    }
                                    Keycode::Down => {
                                        wy += 30;
                                    }
                                    _ => (),
                                }
                            }
                            canvas.copy(
                                &texture,
                                Some(Rect::new(wx, wy, width - 300, height - 300)),
                                None,
                            )?;
                            canvas.present();
                        } else {
                            if wx >= width as i32 {
                                wx = width as i32 - 1;
                            }
                            if wx <= 0 {
                                wx = 1;
                            }

                            if wy >= height as i32 {
                                wy = height as i32 - 1;
                            }

                            if wy <= 0 {
                                wy = 1;
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    Ok(())
}
