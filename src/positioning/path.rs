use crate::positioning::errors::PositioningError;
use crate::positioning::GeoCoordinate;

pub enum GeoPathLengthType
{
  NoLoop,
  ClosedLoop
}

#[derive(Debug, Clone)]
pub struct GeoPath
{
  path: Vec<GeoCoordinate>
}

impl Default for GeoPath
{
  fn default() -> Self
  {
    Self { path: Vec::new() }
  }
}

impl GeoPath
{
  pub fn new(path: &Vec<GeoCoordinate>) -> Self
  {
    Self { path: path.clone() }
  }

  pub fn add(&mut self, coordinate: GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate)) }
    self.path.push(coordinate);
    Ok(())
  }

  pub fn insert(&mut self, index: usize, coordinate: GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate)) }
    self.path.insert(index, coordinate);
    Ok(())
  }

  pub fn at(&self, index: usize) -> Result<GeoCoordinate, PositioningError>
  {
    match self.path.get(index).cloned() {
      None => Err(PositioningError::IndexOutOfBounds(index, self.path.len())),
      Some(x) => Ok(x)
    }
  }

  pub fn remove(&mut self, index: usize) -> Result<(), PositioningError>
  {
    if index >= self.size() {
      return Err(PositioningError::IndexOutOfBounds(index, self.path.len()))
    }
    self.path.remove(index);
    Ok(())
  }

  pub fn replace(&mut self, index: usize, coordinate: GeoCoordinate) -> Result<(), PositioningError>
  {
    if !coordinate.valid() { return Err(PositioningError::InvalidCoordinate(coordinate)) }
    if index >= self.size() {
      return Err(PositioningError::IndexOutOfBounds(index, self.path.len()))
    }
    self.path[index] = coordinate;
    Ok(())
  }

  pub fn contains(&self, coordinate: GeoCoordinate) -> bool
  {
    self.path.contains(&coordinate)
  }

  pub fn clear(&mut self)
  {
    self.path.clear()
  }

  pub fn path(&self) -> &Vec<GeoCoordinate>
  {
    &self.path
  }

  pub fn set_path(&mut self, path: Vec<GeoCoordinate>)
  {
    self.path = path
  }

  pub fn size(&self) -> usize
  {
    self.path.len()
  }

  pub fn length(&self, from: usize, to: usize, length_type: GeoPathLengthType) -> Result<f32, PositioningError>
  {
    if self.path.is_empty() { return Ok(0.0) }
    let len: f32 = (from..to.clamp(0, self.size() - 1))
      .map(|i| self.path[i].distance_to(&self.path[i + 1])
        .expect("Distance calculation failed"))
      .sum();
    return match length_type {
      GeoPathLengthType::NoLoop => Ok(len),
      GeoPathLengthType::ClosedLoop => Ok(len + self.path
        .last()
        .unwrap()
        .distance_to(&self.path[from])?
      )
    }
  }

  pub fn bounding_georectangle(&self)// -> GeoRectangle
  {
    todo!("Implement GeoPath::bounding_georectangle()")
  }

  pub fn translate(&mut self, latitude: f64, longitude: f64)
  {
    todo!("Implement GeoPath::translate()")
  }

  pub fn translated(&self, latitude: f64, longitude: f64) -> GeoPath
  {
    todo!("Implement GeoPath::translated()")
  }

  fn mark_dirty(&mut self)
  {
    todo!("Implement GeoPath::mark_dirty()")
  }
}