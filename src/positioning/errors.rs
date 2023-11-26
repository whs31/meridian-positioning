use thiserror::Error;
use crate::positioning::coordinate::GeoCoordinate;
use crate::positioning::georectangle::GeoRectangle;

#[derive(Debug, Error)]
pub enum PositioningError
{
  #[error("Operation on invalid coordinate: {0}")]
  InvalidCoordinate(GeoCoordinate),

  #[error("Operation on invalid georectangle: {0}")]
  InvalidGeorectangle(GeoRectangle),

  #[error("Index out of bounds: {0} out of {1}")]
  IndexOutOfBounds(usize, usize)
}