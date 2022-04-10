#[cfg(not(target_arch = "wasm32"))]
/// Artificial download function which simply opens the file from disk.
pub fn download_chart(chart_id: usize) -> Option<std::boxed::Box<Vec<u8>>> {
    let chart_string = format!("data/level_{chart_id}.swf");
    let path = std::path::Path::new(&chart_string);

    match std::fs::read(path) {
        Ok(file) => Some(Box::new(file)),
        Err(error) => {
            log::error!("Unable to load file: {} at {:?}", error, path);
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn download_chart(_chart_id: usize) -> Option<std::boxed::Box<Vec<u8>>> {
    return Some(Box::new(
        include_bytes!("..\\..\\..\\data\\level_3348.swf").to_vec(),
    ));
}
