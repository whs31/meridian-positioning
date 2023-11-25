use std::fmt::{Display};
use std::ops::{Add, Mul};
use float_cmp::approx_eq;
use crate::positioning::constants as Constants;
use crate::positioning::errors::PositioningError;
use crate::positioning::utility::{CoordinateField, CoordinateFieldType};
use crate::positioning::utility::CoordinateFieldType::{Latitude, Longitude};

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GeoCoordinateType
{
  InvalidCoordinate,
  Coordinate2D,
  Coordinate3D
}

#[derive(Debug, Copy, Clone)]
pub struct GeoCoordinate
{
  latitude: f64,
  longitude: f64,
  altitude: Option<f32>
}

impl PartialEq for GeoCoordinate
{
  fn eq(&self, other: &Self) -> bool
  {
    approx_eq!(f64, self.latitude, other.latitude, epsilon = 0.0000003) &&
    approx_eq!(f64, self.longitude, other.longitude, epsilon = 0.0000003) &&
    ((other.altitude.is_none() && other.altitude.is_none()) ||
      approx_eq!(f32, self.altitude.unwrap_or(1.0), other.altitude.unwrap_or(-1.0),
        epsilon = 0.0000003))
  }
}

impl Display for GeoCoordinate
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    match self.altitude {
      None => write!(f, "({:.7}°, {:.7}°)", self.latitude, self.longitude),
      Some(x) => write!(f, "({:.7}°, {:.7}°, {:.2}m)", self.latitude, self.longitude, x)
    }
  }
}

impl GeoCoordinate
{
  pub fn new(latitude: f64, longitude: f64, altitude: Option<f32>) -> Self
  {
    Self { latitude, longitude, altitude }
  }

  pub fn coordinate_type(&self) -> GeoCoordinateType
  {
    if self.latitude.valid(Latitude)
      && self.longitude.valid(Longitude)
    {
      return if self.altitude.is_some() { GeoCoordinateType::Coordinate3D }
      else { GeoCoordinateType::Coordinate2D }
    }
    GeoCoordinateType::InvalidCoordinate
  }

  pub fn valid(&self) -> bool
  {
    self.coordinate_type() != GeoCoordinateType::InvalidCoordinate
  }

  pub fn azimuth_to(&self, other: &GeoCoordinate) -> Result<f32, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.clone())) }
    if !other.valid() { return Err(PositioningError::InvalidCoordinate(other.clone())) }

    let d_lon = (other.longitude - self.longitude).to_radians();
    let lat1_rad = self.latitude.to_radians();
    let lat2_rad = other.latitude.to_radians();
    let y = d_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * d_lon.cos();
    let azimuth = y.atan2(x).to_degrees() + 360.0;
    let w = azimuth.trunc() as f32;
    let f = azimuth.fract() as f32;
    Ok(((w + 360.0) as i32 % 360) as f32 + f)
  }

  pub fn distance_to(&self, other: &GeoCoordinate) -> Result<f32, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.clone())) }
    if !other.valid() { return Err(PositioningError::InvalidCoordinate(other.clone())) }

    let d_lat = (other.latitude - self.latitude).to_radians();
    let d_lon = (other.longitude - self.longitude).to_radians();
    let haversine_d_lat = (d_lat / 2.0).sin().powi(2);
    let haversine_d_lon = (d_lon / 2.0).sin().powi(2);
    let y = self.latitude
      .to_radians()
      .cos()
      .mul(other.latitude
        .to_radians()
        .cos()
      ).mul(haversine_d_lon)
      .add(haversine_d_lat)
      .sqrt()
      .asin() as f32;
    Ok(Constants::EARTH_MEAN_RADIUS * y * 2.0f32)
  }

  pub fn at_distance_and_azimuth(&self, distance: f32, azimuth: f32) -> Result<GeoCoordinate, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.clone())); }

    let ratio = distance as f64 / Constants::EARTH_MEAN_RADIUS as f64;
    let lat = (self.latitude
      .to_radians()
      .sin() * ratio.cos() + self.latitude
      .to_radians()
      .cos() * ratio.sin() * (azimuth as f64)
      .to_radians()
      .cos())
      .asin();
    Ok(GeoCoordinate::new(
      lat.to_degrees(),
      ((self.longitude
        .to_radians() + ((azimuth as f64)
        .to_radians()
        .sin() *
        ratio.sin() * self.latitude
        .to_radians()
        .cos())
        .atan2(ratio.cos() - self.latitude
          .to_radians()
          .sin() * lat.sin()))
        .to_degrees())
        .wrap(Longitude),
      self.altitude
    ))
  }
}