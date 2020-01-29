use rand::Rng;

static mut ID: isize = 0;

#[derive(Debug)]
pub struct Blob{
  pub radius: f32,
  pub position: cgmath::Point2<f32>,
  pub id: isize,
}

impl Blob{
  pub fn new(id: isize) -> Blob {
    let mut rng = rand::thread_rng();
    Blob {
      id,
      radius: rng.gen_range(10.0, 100.0),
      position: cgmath::Point2::new(
        rng.gen_range(0.0, 600.0),
        rng.gen_range(0.0, 300.0)
      )
    }
  }
}

