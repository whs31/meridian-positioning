use thiserror::Error;
use crate::positioning::coordinate::GeoCoordinate;

#[derive(Debug, Error)]
pub enum PositioningError
{
  #[error("Operation on invalid coordinate: {0}")]
  InvalidCoordinate(GeoCoordinate)
}