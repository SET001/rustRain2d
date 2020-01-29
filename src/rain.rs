use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use rand::Rng;

use crate::blob::{Blob};

#[derive(Debug)]
pub struct Rain{
  pub blobs: Vec<Blob>,
  blobID: isize,
  maxBlobs: usize,
  intensity: f64
}

impl Rain{
  pub fn new(maxBlobs: usize, intensity: f64)->Rain{
    let mut rain = Rain {
      blobs: Vec::new(),
      blobID: 0,
      maxBlobs,
      intensity
    };
    for _i in 1..maxBlobs {
      rain.spawnBlob();
    }

    rain
  }
  
  pub fn spawnBlob(&mut self){
    self.blobID+=1;
    let blob = Blob::new(self.blobID);
    self.blobs.push(blob);
  }

  fn removeBlod(&mut self, index: usize){
    self.blobs.remove(index);
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult{
    for blob in &self.blobs {
      let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        na::Point2::new(blob.position.x, blob.position.y),
        blob.radius,
        0.1,
        graphics::WHITE,
      )?;
      graphics::draw(ctx, &circle, (na::Point2::new(blob.position.x, blob.position.y),))?;
    }
    graphics::present(ctx)?;
    Ok(())
  }

  pub fn update(&mut self){
    let mut toRemove: Vec<usize> = Vec::new();

    for (index, blob) in &mut (self.blobs.iter_mut().enumerate()) {
      blob.radius -= 1.0;
    }

    self.blobs.retain(|blob| {
      blob.radius > 0.0
    });
    let mut rng = rand::thread_rng();
    let spawnChance = rng.gen_range(0.0, 10.0);
    if 
      (self.blobs.len() < self.maxBlobs)
      && spawnChance < self.intensity
    {
      self.spawnBlob()
    }
  }
}