use std::fmt::{Display, Debug, Formatter};
use std::fmt;
use std::fmt::Result;

#[test]
fn test() {
    let p = Position {
        longitude: 1.0,
        latitude: 2.0,
    };
    println!("{:?}", p);
    println!("{}", p)
}

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        f.debug_tuple("")
            .field(&self.longitude)
            .field(&self.latitude)
            .finish()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}-{})", self.longitude, self.latitude)
    }
}