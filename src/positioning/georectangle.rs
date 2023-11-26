use std::fmt::Display;
use crate::positioning::errors::PositioningError;
use crate::positioning::{CardinalDirection, GeoCoordinate};
use crate::positioning::utility::CoordinateField;
use crate::positioning::utility::CoordinateFieldType::Longitude;

#[derive(Debug, Clone)]
pub struct GeoRectangle
{
  tl: GeoCoordinate,
  br: GeoCoordinate
}

impl Default for GeoRectangle
{
  fn default() -> Self
  {
    Self { tl: GeoCoordinate::default(), br: GeoCoordinate::default() }
  }
}

impl Display for GeoRectangle
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "[{}, {}]", self.tl, self.br)
  }
}

impl GeoRectangle
{
  pub fn new(tl: GeoCoordinate, br: GeoCoordinate) -> Self
  {
    Self { tl, br }
  }

  pub fn from_center_degrees(center: GeoCoordinate, width_degrees: f64, height_degrees: f64) -> Self
  {
    let mut x = Self::new(center, center);
    x.set_width(width_degrees);
    x.set_height(height_degrees);
    x
  }

  pub fn from_center_meters(center: GeoCoordinate, width_meters: f32, height_meters: f32)
                            -> Result<Self, PositioningError>
  {
    let rect = GeoRectangle::new(
      center
        .at_distance_and_azimuth(height_meters / 2.0, CardinalDirection::North.to_degrees())?
        .at_distance_and_azimuth(width_meters / 2.0, CardinalDirection::West.to_degrees())?,
      center
        .at_distance_and_azimuth(height_meters / 2.0, CardinalDirection::South.to_degrees())?
        .at_distance_and_azimuth(width_meters / 2.0, CardinalDirection::East.to_degrees())?
    );
    Ok(rect)
  }

  pub fn from_list(coordinates: &Vec<GeoCoordinate>) -> Self
  {
    //if coordinates.len() < 2 { return GeoRectangle::default() }
    todo!("Implement GeoRectangle::from_list")
  }

  pub fn bottom_right(&self) -> GeoCoordinate { self.br }
  pub fn top_left(&self) -> GeoCoordinate { self.tl }
  pub fn bottom_left(&self) -> GeoCoordinate
  {
    GeoCoordinate::new(self.br.latitude, self.tl.longitude, None)
  }
  pub fn top_right(&self) -> GeoCoordinate
  {
    GeoCoordinate::new(self.tl.latitude, self.br.longitude, None)
  }

  pub fn center(&self) -> GeoCoordinate
  {
    if !self.valid() { return GeoCoordinate::default() }
    GeoCoordinate::new(
      (self.tl.latitude + self.br.latitude) / 2.0,
      if self.tl.longitude > self.br.longitude {
        (self.br.longitude + self.tl.longitude) / 2.0 - 180.0
      } else { (self.br.longitude + self.tl.longitude) / 2.0 }.wrap(Longitude),
      None
    )
  }

  pub fn contains(&self, coordinate: &GeoCoordinate) -> Result<bool, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.tl.clone())) }
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate.clone())) }

    if coordinate.latitude > self.tl.latitude || coordinate.latitude < self.br.latitude {
      return Ok(false)
    }
    if coordinate.latitude == 90.0 && self.tl.latitude == 90.0 { return Ok(true) }
    if coordinate.latitude == -90.0 && self.br.latitude == -90.0 { return Ok(true) }
    if self.tl.longitude <= self.br.longitude {
      if coordinate.longitude < self.tl.longitude || coordinate.longitude > self.br.longitude {
        return Ok(false)
      }
    }
    else {
      if coordinate.longitude < self.tl.longitude && coordinate.longitude > self.br.longitude {
        return Ok(false)
      }
    }
    Ok(true)
  }

  pub fn contains_rect(&self, other: &GeoRectangle) -> Result<bool, PositioningError>
  {
    let ret = self.contains(&other.top_left())? && self.contains(&other.top_right())?
      && self.contains(&other.bottom_left())? && self.contains(&other.bottom_right())?;
    Ok(ret)
  }

  pub fn width(&self) -> f64
  {
    if !self.valid() { return 0.0 }
    let mut w = self.br.longitude - self.tl.longitude;
    if w < 0.0 { w += 360.0 }
    if w > 360.0 { w -= 360.0 }
    w
  }

  pub fn height(&self) -> f64
  {
    if !self.valid() { return 0.0 }
    self.tl.latitude - self.br.latitude
  }

  pub fn width_meters(&self) -> Result<f32, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidGeorectangle(self.clone())) }
    let w = self
      .top_left()
      .distance_to(&self.top_right())?;
    Ok(w)
  }

  pub fn height_meters(&self) -> Result<f32, PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidGeorectangle(self.clone())) }
    let h = self
      .top_left()
      .distance_to(&self.bottom_left())?;
    Ok(h)
  }

  pub fn intersects(&self, other: &GeoRectangle) -> bool
  {
    if self.tl.latitude < other.br.latitude || self.br.latitude > other.tl.latitude { return false }
    if self.tl.latitude == 90.0 && self.tl.latitude == other.tl.latitude { return true }
    if self.br.latitude == -90.0 && self.br.latitude == other.br.latitude { return true }
    if self.tl.longitude < self.br.longitude
    {
      if other.tl.longitude < other.br.longitude {
        if self.tl.longitude > other.br.longitude || self.br.longitude < other.tl.longitude {
          return false
        }
      } else {
        if self.tl.longitude > other.br.longitude && self.br.longitude < other.tl.longitude {
          return false
        }
      }
    }
    else
    {
      if other.tl.longitude < other.br.longitude {
        if other.tl.longitude > self.br.longitude && other.br.longitude < self.tl.longitude {
          return false
        }
      } else { return true }
    }
    true
  }

  pub fn union(&self, other: &GeoRectangle) -> Self
  {
    todo!("Implement GeoRectangle::union")
  }

  pub fn intersection(&self, other: &GeoRectangle) -> Self
  {
    todo!("Implement GeoRectangle::intersection")
  }

  pub fn translate(&mut self, latitude: f64, longitude: f64)
  {
    todo!("Implement GeoRectangle::translate")
  }

  pub fn translated(&self, latitude: f64, longitude: f64) -> Self
  {
    todo!("Implement GeoRectangle::translated")
  }

  pub fn extend(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    todo!("Implement GeoRectangle::extend")
  }

  pub fn set_top_left(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate.clone())) }
    self.tl = coordinate.clone();
    Ok(())
  }

  pub fn set_top_right(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate.clone())) }
    self.tl.latitude = coordinate.latitude;
    self.br.longitude = coordinate.longitude;
    Ok(())
  }

  pub fn set_bottom_left(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate.clone())) }
    self.br.latitude = coordinate.latitude;
    self.tl.longitude = coordinate.longitude;
    Ok(())
  }

  pub fn set_bottom_right(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate.clone())) }
    self.br = coordinate.clone();
    Ok(())
  }

  pub fn set_width(&mut self, width_degrees: f64)
  {
    if !self.valid() { return }
    if width_degrees < 0.0 { return }
    if width_degrees > 360.0 {
      self.tl.longitude = -180.0;
      self.br.longitude = 180.0;
      return
    }
    let c = self.center();
    self.tl = GeoCoordinate::new(
      self.tl.latitude,
      (c.longitude - width_degrees / 2.0).wrap(Longitude),
      None
    );
    self.br = GeoCoordinate::new(
      self.br.latitude,
      (c.longitude + width_degrees / 2.0).wrap(Longitude),
      None
    );
  }

  pub fn set_height(&mut self, height_degrees: f64)
  {
    if !self.valid() { return }
    if height_degrees < 0.0 || height_degrees > 180.0 { return }

    let c = self.center();
    let mut tl_lat = c.latitude + height_degrees / 2.0;
    let mut br_lat = c.latitude - height_degrees / 2.0;
    if tl_lat > 90.0 {
      br_lat = 2.0 * c.latitude - 90.0;
      tl_lat = 90.0;
    }
    if tl_lat < -90.0 {
      br_lat = -90.0;
      tl_lat = -90.0;
    }
    if br_lat > 90.0 {
      br_lat = 90.0;
      tl_lat = 90.0;
    }
    if br_lat < -90.0 {
      br_lat = -90.0;
      tl_lat = 2.0 * c.latitude + 90.0;
    }
    self.tl = GeoCoordinate::new(tl_lat, self.tl.longitude, None);
    self.br = GeoCoordinate::new(br_lat, self.br.longitude, None);
  }

  pub fn set_center(&mut self, center: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !center.valid() { return Err(PositioningError::InvalidCoordinate(center.clone())) }
    if !self.valid() { return Err(PositioningError::InvalidGeorectangle(self.clone())) }
    let mut tl_lat = center.latitude + self.height() / 2.0;
    let mut tl_lon = (center.longitude - self.width() / 2.0).wrap(Longitude);
    let mut br_lat = center.latitude - self.height() / 2.0;
    let mut br_lon = (center.longitude + self.width() / 2.0).wrap(Longitude);
    if tl_lat > 90.0 {
      br_lat = 2.0 * center.latitude - 90.0;
      tl_lat = 90.0;
    }
    if tl_lat < -90.0 {
      br_lat = -90.0;
      tl_lat = -90.0;
    }
    if br_lat > 90.0 {
      br_lat = 90.0;
      tl_lat = 90.0;
    }
    if br_lat < -90.0 {
      tl_lat = 2.0 * center.latitude + 90.0;
      br_lat = -90.0;
    }
    if self.width() == 360.0 {
      tl_lon = -180.0;
      br_lon = 180.0;
    }

    self.tl = GeoCoordinate::new(tl_lat, tl_lon, None);
    self.br = GeoCoordinate::new(br_lat, br_lon, None);
    Ok(())
  }

  pub fn valid(&self) -> bool
  {
    self.tl.valid() && self.br.valid() && self.tl.latitude >= self.br.latitude
  }

  pub fn empty(&self) -> bool
  {
    if !self.valid() { return true }
    self.tl.latitude == self.br.latitude && self.tl.longitude == self.br.longitude
  }

  fn extend_shape(&mut self, coord: &GeoCoordinate) -> Result<(), PositioningError>
  {
    if !self.valid() { return Err(PositioningError::InvalidGeorectangle(self.clone())) }
    if !coord.valid() { return Err(PositioningError::InvalidCoordinate(coord.clone())) }
    if self.contains(coord)? { return Err(PositioningError::InvalidCoordinate(coord.clone())) }

    // TODO: rewrite in functional way (after tests ofc)
    let mut left = self.tl.longitude;
    let mut right = self.br.longitude;
    let mut top = self.tl.latitude;
    let mut bottom = self.br.latitude;

    top = top.max(coord.latitude);
    bottom = bottom.min(coord.latitude);
    let wrap = left > right;
    if wrap && coord.longitude > right && coord.longitude < left {
      if (left - coord.longitude).abs() < (right - coord.longitude).abs() {
        left = coord.longitude
      }
      else {
        right = coord.longitude
      }
    }
    else if !wrap {
      if coord.longitude < left {
        if 360.0 - (right - coord.longitude) < left - coord.longitude {
          right = coord.longitude
        }
        else {
          left = coord.longitude
        }
      }
      else if coord.longitude > right {
        if 360.0 - (coord.longitude - left) < coord.longitude - right {
          left = coord.longitude
        }
        else {
          right = coord.longitude
        }
      }
    }
    self.set_top_left(&GeoCoordinate::new(top, left, None))?;
    self.set_bottom_right(&GeoCoordinate::new(bottom, right, None))?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default() {
    let rect = GeoRectangle::default();
    assert!(rect.empty());
    assert!(!rect.valid());
    assert!(!rect.top_left().valid());
    assert!(!rect.bottom_right().valid());
  }

  #[test]
  fn test_from_center_degrees() {
    let rect = GeoRectangle::from_center_degrees(
      GeoCoordinate::new(5.0, 5.0, None),
      10.0,
      10.0
    );
    assert_eq!(rect.top_left(), GeoCoordinate::new(10.0, 0.0, None));
    assert_eq!(rect.bottom_right(), GeoCoordinate::new(0.0, 10.0, None));
  }
}