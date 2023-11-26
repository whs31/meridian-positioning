#[derive(Debug, PartialOrd, PartialEq)]
pub enum CardinalDirection
{
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest
}

impl CardinalDirection
{
  pub fn to_degrees(&self) -> f32
  {
    match self {
      CardinalDirection::North => 0.0,
      CardinalDirection::NorthEast => 45.0,
      CardinalDirection::East => 90.0,
      CardinalDirection::SouthEast => 135.0,
      CardinalDirection::South => 180.0,
      CardinalDirection::SouthWest => 225.0,
      CardinalDirection::West => 270.0,
      CardinalDirection::NorthWest => 315.0
    }
  }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum CoordinateFieldType
{
  Latitude,
  Longitude
}

pub trait CoordinateField
{
  fn valid(&self, field_type: CoordinateFieldType) -> bool;
  fn wrap(&self, coordinate_field_type: CoordinateFieldType) -> f64;
}

impl CoordinateField for f64
{
  fn valid(&self, field_type: CoordinateFieldType) -> bool {
    match field_type {
      CoordinateFieldType::Latitude => *self >= -90.0 && *self <= 90.0,
      CoordinateFieldType::Longitude => *self >= -180.0 && *self <= 180.0
    }
  }

  fn wrap(&self, coordinate_field_type: CoordinateFieldType) -> f64 {
    return match coordinate_field_type {
      CoordinateFieldType::Latitude => {
        if *self > 90.0 { return 90.0 }
        else if *self < -90.0 { return -90.0 }
        *self
      }
      CoordinateFieldType::Longitude => {
        if *self > 180.0 { return 180.0 }
        else if *self < -180.0 { return -180.0 }
        *self
      }
    }
  }
}