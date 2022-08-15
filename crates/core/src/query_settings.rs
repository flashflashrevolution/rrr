use crate::ScrollDirection;
use inter_struct::prelude::*;
use reqwest::Url;
use std::str::FromStr;

#[derive(StructMerge, Debug, Clone, Copy, PartialEq, Default)]
#[struct_merge("crate::settings::Settings")]
pub struct SettingsMerge {
    pub scroll_speed: Option<u16>,
    pub judge_zero_point: Option<i128>,
    pub scroll_direction: Option<ScrollDirection>,
    pub lane_gap: Option<u8>,
}

/// Attempts to get the settings from the URL.
/// # Panics
///
/// If the `ScrollDirection` doesn't match the enum.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_optional_settings() -> SettingsMerge {
    let url = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.url().ok());

    let mut settings = SettingsMerge::default();

    if let Some(url_str) = url {
        if let Ok(url) = Url::parse(&url_str) {
            for (key, value) in url.query_pairs() {
                match &*key {
                    "scroll_speed" => {
                        let speed = value.parse::<u16>().unwrap();
                        settings.scroll_speed.replace(speed);
                    }
                    "scroll_direction" => {
                        let scroll_direction = value.parse::<String>().unwrap();
                        let direction = match ScrollDirection::from_str(&scroll_direction) {
                            Ok(direction) => direction,
                            Err(err) => panic!("{}", err),
                        };
                        settings.scroll_direction.replace(direction);
                    }

                    &_ => (),
                }
            }
        }
    }

    settings
}
