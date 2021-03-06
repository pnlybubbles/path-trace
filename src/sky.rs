extern crate image;

use ray::Ray;
use math::vector::*;
use constant::*;
use std::fs::File;
use std::io::BufReader;

pub trait Sky {
  fn radiance(&self, &Ray) -> Vector3;
}

pub struct UniformSky {
  pub emission: Vector3,
}

impl Sky for UniformSky {
  fn radiance(&self, _: &Ray) -> Vector3 {
    self.emission
  }
}

pub struct SimpleSky {
  pub meridian: Vector3,
  pub horizon: Vector3,
}

impl Sky for SimpleSky {
  fn radiance(&self, ray: &Ray) -> Vector3 {
    let weight = ray.direction.dot(Vector3::new(0.0, 1.0, 0.0)).abs();
    self.meridian * weight + self.horizon * (1.0 - weight)
  }
}

pub struct IBLSky {
  hdr_image: Vec<image::Rgb<f32>>,
  height: usize,
  longitude_offset: f32,
}

impl IBLSky {
  pub fn new(path: &str, longitude_offset: f32) -> IBLSky {
    println!("loading hdr image...");
    let image_file = File::open(path).unwrap();
    let decoder = image::hdr::HDRDecoder::new(BufReader::new(image_file)).unwrap();
    println!("{:?}", decoder.metadata());
    let height = decoder.metadata().height as usize;
    let image = decoder.read_image_hdr().unwrap();
    IBLSky {
      hdr_image: image,
      height: height,
      longitude_offset: longitude_offset,
    }
  }
}

impl Sky for IBLSky {
  fn radiance(&self, ray: &Ray) -> Vector3 {
    // 0 <= theta <= pi
    let theta = (ray.direction.y).acos();
    // -pi < phi <= pi
    let phi = ray.direction.z.atan2(ray.direction.x);
    // 0 <= (u, v) < 1
    let u = ((phi + PI + self.longitude_offset) / (2.0 * PI)) % 1.0;
    let v = (theta / PI) % 1.0;
    let height = self.height;
    let width = self.height * 2;
    let all = width * height;
    let x = (width as f32 * u).floor() as usize;
    let y = (height as f32 * v).floor() as usize;
    let index = y * width + x;
    let color = self.hdr_image[index % all];
    return Vector3::new(
      color.data[0] as f32,
      color.data[1] as f32,
      color.data[2] as f32,
    );
  }
}
