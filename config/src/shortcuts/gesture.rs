use std::str::FromStr;

// SPDX-License-Identifier: MPL-2.0
use serde::{Deserialize, Serialize};

/// Description of a gesture that can be handled by the compositor
#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[serde(deny_unknown_fields)]
pub struct Gesture {
    /// How many fingers are held down
    pub fingers: i32,
    pub direction: Direction,
    // A custom description for a custom binding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Describes a direction, either absolute or relative
#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[serde(deny_unknown_fields)]
pub enum Direction {
    Relative(RelativeDirection),
    Absolute(AbsoluteDirection),
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Absolute(abs) => match abs {
                AbsoluteDirection::Up => "Up".to_string(),
                AbsoluteDirection::Down => "Down".to_string(),
                AbsoluteDirection::Left => "Left".to_string(),
                AbsoluteDirection::Right => "Right".to_string(),
            },
            Direction::Relative(rel) => match rel {
                RelativeDirection::Forward => "Forward".to_string(),
                RelativeDirection::Backward => "Backward".to_string(),
                RelativeDirection::SideLeft => "LeftRelative".to_string(),
                RelativeDirection::SideRight => "RightRelative".to_string(),
            },
        }
    }
}

/// Describes a relative direction (typically relative to the workspace direction)
#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[serde(deny_unknown_fields)]
pub enum RelativeDirection {
    Forward,
    Backward,
    SideLeft,
    SideRight,
}

/// Describes an absolute direction (i.e. not relative to workspace direction)
#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash)]
#[serde(deny_unknown_fields)]
pub enum AbsoluteDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Gesture {
    /// Creates a new gesture from a number of fingers and a direction
    pub fn new(fingers: impl Into<i32>, direction: impl Into<Direction>) -> Gesture {
        Gesture {
            fingers: fingers.into(),
            direction: direction.into(),
            description: None,
        }
    }

    /// Returns true if the direction is absolute
    pub fn is_absolute(&self) -> bool {
        matches!(self.direction, Direction::Absolute(_))
    }

    /// Append the binding to an existing string
    pub fn to_string_in_place(&self, string: &mut String) {
        string.push_str(&format!(
            "{} Finger {}",
            self.fingers,
            self.direction.to_string()
        ));
    }
}

impl ToString for Gesture {
    fn to_string(&self) -> String {
        let mut string = String::new();
        self.to_string_in_place(&mut string);
        string
    }
}


impl FromStr for Gesture {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut value_iter = value.split(" ");
        let n = match value_iter.next() {
            Some(val) => val,
            None => {
                return Err(format!("no value for the number of fingers"));
            },
        };
        let fingers = match i32::from_str(n) {
            Ok(a) => a,
            Err(_) => {
                return Err(format!("could not parse number of fingers"));
            },
        };

        let n2 = match value_iter.nth(1) {
            Some(val) => val,
            None => {
                return Err(format!("could not parse direction"));
            },
        };

        return Self {
            fingers,
            direction,
            
        }
    }
}