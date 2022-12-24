use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::SDL_WindowFlags;

struct Ball {
  rect: sdl2::rect::Rect,
  vel_x: i32,
  vel_y: i32,
}

impl Ball {
  fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
    Self {
      rect: sdl2::rect::Rect::new(x, y, w, h),
      vel_x: 2,
      vel_y: 2,
    }
  }

  fn update(&mut self) {
    self.rect.x = self.rect.x + self.vel_x;
    self.rect.y = self.rect.y + self.vel_y;

    if self.rect.x <= 0 - self.rect.w || self.rect.x >= 1280 {
      self.rect.x = 1280 / 2 - 16;
      self.rect.y = 720 / 2 - 16;
      self.invert_x()
    }

    if self.rect.y <= 0 || self.rect.y >= 720 - self.rect.h {
      self.invert_y()
    }
  }

  fn invert_x(&mut self) {
    self.vel_x = self.vel_x * -1;
  }

  fn invert_y(&mut self) {
    self.vel_y = self.vel_y * -1;
  }
}

struct Padle {
  rect: sdl2::rect::Rect,
}

impl Padle {
  fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
    Self {
      rect: sdl2::rect::Rect::new(x, y, w, h),
    }
  }

  fn move_y(&mut self, val: i32) {
    self.rect.y = self.rect.y + val;
  }
}

fn main() {
  let sdl_ctx = sdl2::init().unwrap();
  let video_subsys = sdl_ctx.video().unwrap();

  let window = video_subsys
    .window("title", 1280, 720)
    .set_window_flags(SDL_WindowFlags::SDL_WINDOW_METAL as u32)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  canvas.set_draw_color(Color::RGB(18, 18, 18));
  canvas.clear();
  canvas.present();

  let mut events = sdl_ctx.event_pump().unwrap();

  let mut padle1 = Padle::new(16, 720 / 2 - 64, 32, 128);
  let mut padle2 = Padle::new(1280 - 48, 720 / 2 - 64, 32, 128);
  let mut ball = Ball::new(1280 / 2 - 16, 720 / 2 - 16, 32, 32);

  'window_loop: loop {
    canvas.set_draw_color(Color::RGB(18, 18, 18));
    canvas.clear();

    for event in events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'window_loop,
        _ => {}
      }
    }

    ball.update();

    if ball.rect.y >= padle2.rect.y + padle2.rect.h {
      padle1.move_y(16);
      padle2.move_y(16);
    }
    if ball.rect.y <= padle2.rect.y  {
      padle1.move_y(-16);
      padle2.move_y(-16);
    }

    if padle1.rect.contains_rect(ball.rect) || padle2.rect.contains_rect(ball.rect) {
      ball.invert_x()
    }

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(padle1.rect).expect("");
    canvas.fill_rect(padle2.rect).expect("");
    canvas.fill_rect(ball.rect).expect("");

    canvas.present();
    // Limit fps to 120
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
  }
}
