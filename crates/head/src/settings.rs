#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(inspectable)
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumString)]
pub enum ScrollDirection {
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Down,
}

/// Stores RRR settings to start charts with.
#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(inspectable)
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub scroll_speed: u16,
    pub offset: i8,
    pub judge_position: i32,
    pub scroll_direction: ScrollDirection,
    pub lane_gap: u8,
    pub muted: bool,
}

impl Default for Settings {
    #[must_use]
    fn default() -> Self {
        Self {
            scroll_speed: 3000,
            offset: 0,
            judge_position: 92,
            scroll_direction: ScrollDirection::Up,
            lane_gap: 72,
            muted: true,
        }
    }
}

impl Settings {
    #[must_use]
    pub fn new(
        scroll_speed: u16,
        offset: i8,
        judge_zero_point: i32,
        scroll_direction: ScrollDirection,
        lane_gap: u8,
        muted: bool,
    ) -> Self {
        Self {
            scroll_speed,
            offset,
            judge_position: judge_zero_point,
            scroll_direction,
            lane_gap,
            muted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.scroll_speed, 3000);
        assert_eq!(settings.judge_position, 92);
        assert_eq!(settings.scroll_direction, ScrollDirection::Up);
        assert_eq!(settings.lane_gap, 72);
    }
}
