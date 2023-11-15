mod dielectric;
mod emissive;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use emissive::*;
pub use lambertian::*;
pub use metal::*;

use super::{Color3, HitRecord, Point3, Ray, Vector2};
use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _uv: &Vector2, _point: &Point3) -> Color3 {
        Color3::zero()
    }
}
