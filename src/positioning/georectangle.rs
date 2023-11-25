use std::fmt::Display;
use std::ops::Mul;
use crate::positioning::errors::PositioningError;
use crate::positioning::GeoCoordinate;

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

  pub fn from_center_meters(center: GeoCoordinate, width_meters: f64, height_meters: f64) -> Self
  {
    todo!("Implement GeoRectangle::from_center_meters")
  }

  pub fn from_list(coordinates: &Vec<GeoCoordinate>) -> Self
  {
    //if coordinates.len() < 2 { return GeoRectangle::default() }
    todo!("Implement GeoRectangle::from_list")
  }

  pub fn bottom_left(&self) -> GeoCoordinate
  {
    todo!("Implement GeoRectangle::bottom_left")
  }

  pub fn bottom_right(&self) -> GeoCoordinate
  {
    todo!("Implement GeoRectangle::bottom_right")
  }

  pub fn top_left(&self) -> GeoCoordinate
  {
    todo!("Implement GeoRectangle::top_left")
  }

  pub fn top_right(&self) -> GeoCoordinate
  {
    todo!("Implement GeoRectangle::top_right")
  }

  pub fn center(&self) -> GeoCoordinate
  {
    todo!("Implement GeoRectangle::center")
  }

  pub fn contains(&self, coordinate: &GeoCoordinate) -> bool
  {
    todo!("Implement GeoRectangle::contains")
  }

  pub fn width(&self) -> f64
  {
    todo!("Implement GeoRectangle::width")
  }

  pub fn height(&self) -> f64
  {
    todo!("Implement GeoRectangle::height")
  }

  pub fn intersects(&self, other: &GeoRectangle) -> bool
  {
    todo!("Implement GeoRectangle::intersects")
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
    todo!("Implement GeoRectangle::set_top_left")
  }

  pub fn set_top_right(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    todo!("Implement GeoRectangle::set_top_right")
  }

  pub fn set_bottom_left(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    todo!("Implement GeoRectangle::set_bottom_left")
  }

  pub fn set_bottom_right(&mut self, coordinate: &GeoCoordinate) -> Result<(), PositioningError>
  {
    todo!("Implement GeoRectangle::set_bottom_right")
  }

  pub fn set_width(&mut self, width_degrees: f64)
  {
    todo!("Implement GeoRectangle::set_width")
  }

  pub fn set_height(&mut self, height_degrees: f64)
  {
    todo!("Implement GeoRectangle::set_height")
  }

  pub fn set_center(&mut self, center: &GeoCoordinate) -> Result<(), PositioningError>
  {
    todo!("Implement GeoRectangle::set_center")
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
    if !self.valid() { return Err(PositioningError::InvalidCoordinate(self.tl.clone())) }
    if !coord.valid() { return Err(PositioningError::InvalidCoordinate(coord.clone())) }
    if self.contains(coord) { return Err(PositioningError::InvalidCoordinate(coord.clone())) }

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

