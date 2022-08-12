#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

/// Stores RRR settings to start charts with.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub scroll_speed: f32,
    pub receptor_vertical_position: u8,
    pub scroll_direction: ScrollDirection,
    pub lane_gap: u8,
}

impl Default for Settings {
    #[must_use]
    fn default() -> Self {
        Self {
            scroll_speed: 1000.0,
            receptor_vertical_position: 64,
            scroll_direction: ScrollDirection::Up,
            lane_gap: 72,
        }
    }
}

impl Settings {
    #[must_use]
    pub fn new(
        scroll_speed: f32,
        receptor_vertical_position: u8,
        scroll_direction: ScrollDirection,
        lane_gap: u8,
    ) -> Self {
        Self {
            scroll_speed,
            receptor_vertical_position,
            scroll_direction,
            lane_gap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.scroll_speed, 1000.0);
        assert_eq!(settings.receptor_vertical_position, 64);
        assert_eq!(settings.scroll_direction, ScrollDirection::Up);
        assert_eq!(settings.lane_gap, 72);
    }
}
