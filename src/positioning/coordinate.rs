use std::fmt::{Display};
use std::ops::{Add, Div, Mul, Sub};
use float_cmp::approx_eq;
use crate::positioning::constants as Constants;
use crate::positioning::errors::PositioningError;
use crate::positioning::utility::{CoordinateField};
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

impl Default for GeoCoordinate
{
  fn default() -> Self
  {
    Self { latitude: f64::NAN, longitude: f64::NAN, altitude: None }
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
    let azimuth = d_lon
      .sin()
      .mul(other.latitude
        .to_radians()
        .cos()
      ).atan2(self.latitude
      .to_radians()
      .cos()
      .mul(other.latitude
        .to_radians()
        .sin()
      ).sub(
      self.latitude
        .to_radians()
        .sin()
        .mul(other.latitude
          .to_radians()
          .cos()
          .mul(d_lon
            .cos()
          )
        )
    )
    ).to_degrees()
      .add(360.0);
    Ok(((azimuth.trunc() + 360.0) as i32 % 360) as f32 + azimuth.fract() as f32)
  }

  pub fn distance_to(&self, other: &GeoCoordinate) -> Result<f32, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.clone())) }
    if !other.valid() { return Err(PositioningError::InvalidCoordinate(other.clone())) }

    let res = Constants::EARTH_MEAN_RADIUS
      .mul(self.latitude
        .to_radians()
        .cos()
        .mul(other.latitude
          .to_radians()
          .cos()
        ).mul((other.longitude - self.longitude)
        .to_radians()
        .div(2.0)
        .sin()
        .powi(2)
      ).add((other.latitude - self.latitude)
        .to_radians()
        .div(2.0)
        .sin()
        .powi(2)
      ).sqrt()
        .asin() as f32
      ).mul(2.0f32);
    Ok(res)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default()
  {
    assert_eq!(GeoCoordinate::default(), GeoCoordinate::new(f64::NAN, f64::NAN, None));
  }

  #[test]
  fn test_coordinate_type()
  {
    assert_eq!(GeoCoordinate::default().coordinate_type(), GeoCoordinateType::InvalidCoordinate);
    assert_eq!(GeoCoordinate::new(60.0, 30.0, None).coordinate_type(), GeoCoordinateType::Coordinate2D);
    assert_eq!(GeoCoordinate::new(60.0, 30.0, Some(10.0)).coordinate_type(), GeoCoordinateType::Coordinate3D);
  }

  #[test]
  fn test_distance_to()
  {
    let t = GeoCoordinate::new(60.0, 30.0, None);
    assert_eq!(t.distance_to(&GeoCoordinate::new(60.0, 31.0, None)).unwrap().round(), 55597.0);
    assert_eq!(t.distance_to(&GeoCoordinate::new(60.0, 29.0, None)).unwrap().round(), 55597.0);
    assert_eq!(t.distance_to(&GeoCoordinate::new(59.0, 29.0, None)).unwrap().round(), 124694.0);
    assert_eq!(t.distance_to(&GeoCoordinate::new(59.0, 30.0, None)).unwrap().round(), 111195.0);
  }

  #[test]
  fn test_azimuth_to()
  {
    let t = GeoCoordinate::new(60.0, 30.0, None);
    assert_eq!(t.azimuth_to(&GeoCoordinate::new(60.0, 31.0, None)).unwrap(), 89.566986);
    assert_eq!(t.azimuth_to(&GeoCoordinate::new(60.0, 29.0, None)).unwrap(), 270.433);
    assert_eq!(t.azimuth_to(&GeoCoordinate::new(59.0, 29.0, None)).unwrap(), 207.34126);
    assert_eq!(t.azimuth_to(&GeoCoordinate::new(59.0, 30.0, None)).unwrap(), 180.0);
  }

  #[test]
  fn test_at_distance_and_azimuth()
  {
    let test_coord = GeoCoordinate::new(60.0, 30.0, None);
    let d: Vec<f32> = vec![10000.0, -10000.0, 55600.0, -43400.0];
    let az: Vec<f32> = vec![0.0, 90.0, 180.0, 270.0, 360.0];

    assert_eq!(test_coord.at_distance_and_azimuth(d[0], az[0]).unwrap(), GeoCoordinate::new(60.089932059, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[0], az[1]).unwrap(), GeoCoordinate::new(59.999877754, 30.179863675, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[0], az[2]).unwrap(), GeoCoordinate::new(59.910067941, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[0], az[3]).unwrap(), GeoCoordinate::new(59.999877754, 29.820136325, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[0], az[4]).unwrap(), GeoCoordinate::new(60.089932059, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[1], az[0]).unwrap(), GeoCoordinate::new(59.910067941, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[1], az[1]).unwrap(), GeoCoordinate::new(59.999877754, 29.820136325, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[1], az[2]).unwrap(), GeoCoordinate::new(60.089932059, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[1], az[3]).unwrap(), GeoCoordinate::new(59.999877754, 30.179863675, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[1], az[4]).unwrap(), GeoCoordinate::new(59.910067941, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[2], az[0]).unwrap(), GeoCoordinate::new(60.500022248, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[2], az[1]).unwrap(), GeoCoordinate::new(59.996221155, 30.999968343, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[2], az[2]).unwrap(), GeoCoordinate::new(59.499977752, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[2], az[3]).unwrap(), GeoCoordinate::new(59.996221155, 29.000031657, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[2], az[4]).unwrap(), GeoCoordinate::new(60.500022248, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[3], az[0]).unwrap(), GeoCoordinate::new(59.609694864, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[3], az[1]).unwrap(), GeoCoordinate::new(59.997697499, 29.219425949, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[3], az[2]).unwrap(), GeoCoordinate::new(60.390305136, 30.000000000, None));
    assert_eq!(test_coord.at_distance_and_azimuth(d[3], az[3]).unwrap(), GeoCoordinate::new(59.997697499, 30.780574051, None));
  }
}