use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use mint::Point2;
use cgmath;
use std::time::{Duration, SystemTime};
use ggez::nalgebra as na;

mod rain;
mod blob;

use crate::rain::{Rain};
use crate::blob::{Blob};

#[derive(Debug)]
struct MainState {
  // frames: usize,
  // gtext: graphics::Text,
  rain: Rain,
  text: String,
  tick: i64,
  // updateFps: function
}
impl MainState {
  fn new() -> MainState {
    MainState {
      text: String::from("asdsad"),
      tick: 0,
      rain: Rain::new(20, 5.0)
    }
  }

  fn updateFps(&mut self){
    self.increaseTick();
    self.text = format!("Frame: {}", self.tick);
  }

  fn increaseTick(&mut self){
    self.tick += 1;
  }
}
impl event::EventHandler for MainState{
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    self.rain.update();
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
    self.updateFps();
    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
    let dest_point = cgmath::Point2::new(1.0, 10.0);
    let stext: &str = &self.text;
    let text = graphics::Text::new((stext, font, 48.0));
    graphics::draw(ctx, &text, (dest_point,));

    self.rain.draw(ctx);
    
    graphics::present(ctx)?;
    Ok(())
  }
}

fn main() -> GameResult {
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };

  let cb = ggez::ContextBuilder::new("2D RAIN", "ggez").add_resource_path(resource_dir);
  let (ctx, event_loop) = &mut cb.build()?;
  let state = &mut MainState::new();
  event::run(ctx, event_loop, state)
}