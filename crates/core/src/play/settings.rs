pub enum ScrollDirection {
    Up,
    Down,
}

pub struct Settings {
    pub scroll_speed: f64,
    pub receptor_vertical_position: u8,
    pub scroll_direction: ScrollDirection,
    pub lane_gap: u8,
}

impl Settings {
    #[must_use]
    pub fn new(
        scroll_speed: f64,
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

    #[must_use]
    pub fn default() -> Self {
        Self {
            scroll_speed: 1000.0,
            receptor_vertical_position: 64,
            scroll_direction: ScrollDirection::Up,
            lane_gap: 72,
        }
    }
}
