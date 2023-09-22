use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub inner: usize,
    pub outer: usize,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.outer, self.inner).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (outer, inner) = Deserialize::deserialize(deserializer)?;
        Ok(Position { inner, outer })
    }
}

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position {
            inner: rng.gen_range(0..9),
            outer: rng.gen_range(0..9),
        }
    }
}
