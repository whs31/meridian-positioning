use thiserror::Error;
use crate::positioning::coordinate::GeoCoordinate;

#[derive(Debug, Error)]
pub enum PositioningError
{
  #[error("Operation on invalid coordinate: {0}")]
  InvalidCoordinate(GeoCoordinate),

  #[error("Index out of bounds: {0} out of {1}")]
  IndexOutOfBounds(usize, usize)
}